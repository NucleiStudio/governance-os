/*
 * Copyright 2020 Nuclei Studio OÜ
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{
    AccountCurrencyData, Balances, Config, Error, RoleBuilder, RoleBuilderOf, RoleManagerOf,
    TotalIssuances,
};
use frame_support::{StorageDoubleMap, StorageMap};
use governance_os_support::traits::RoleManager;
use sp_runtime::{
    traits::{CheckedAdd, CheckedSub, Saturating, Zero},
    DispatchResult,
};
use sp_std::{collections::btree_map::BTreeMap, marker};

/// An internal helper to represent balance changes. It is used to express in a better manner
/// operations done on balances while saving on weight costs by fetching the required data
/// only when necessary and forwarding errors approprietaly.
#[derive(Clone)]
pub struct Mutation<T: Config> {
    currency_id: T::CurrencyId,
    // bools are wether the balance was modified and wether the balance was zero when we first read it.
    balances: BTreeMap<T::AccountId, (AccountCurrencyData<T::Balance>, bool, bool)>,
    coins_created: T::Balance,
    coins_burned: T::Balance,
    _phantom: marker::PhantomData<T>,
}
impl<T: Config> Mutation<T> {
    pub fn new_for_currency(currency_id: T::CurrencyId) -> Self {
        Self {
            currency_id: currency_id,
            balances: BTreeMap::new(),
            coins_created: Zero::zero(),
            coins_burned: Zero::zero(),
            _phantom: marker::PhantomData,
        }
    }

    /// Return a balance from the `self.balances` cache or fetch it from the node DB.
    pub fn get_or_fetch_balance(&mut self, who: &T::AccountId) -> AccountCurrencyData<T::Balance> {
        let in_cache = self.balances.get(who);
        match in_cache {
            None => {
                let in_node = Balances::<T>::get(who, self.currency_id);
                self.balances.insert(
                    who.clone(),
                    (in_node.clone(), false, in_node.total() == Zero::zero()),
                );

                in_node
            }
            Some(cache) => cache.clone().0,
        }
    }

    /// Save an updated balance in our memory cache
    pub fn save_balance(&mut self, who: &T::AccountId, balance: AccountCurrencyData<T::Balance>) {
        let in_memory_snapshot_first_started_at_0 = match self.balances.get(who) {
            None => false,
            Some(data) => data.2,
        };

        // Note how the boolean is set to true since we modified the balance
        self.balances.insert(
            who.clone(),
            (balance, true, in_memory_snapshot_first_started_at_0),
        );
    }

    /// Verify that the currency is transferable
    pub fn ensure_must_be_transferable_for(&mut self, who: &T::AccountId) -> DispatchResult {
        if !RoleManagerOf::<T>::has_role(
            who,
            RoleBuilderOf::<T>::transfer_currency(self.currency_id),
        ) {
            return Err(Error::<T>::UnTransferableCurrency.into());
        }

        Ok(())
    }

    pub fn add_free_balance(
        &mut self,
        who: &T::AccountId,
        increment: T::Balance,
    ) -> DispatchResult {
        let mut balance = self.get_or_fetch_balance(who);
        balance.free = balance
            .free
            .checked_add(&increment)
            .ok_or(Error::<T>::BalanceOverflow)?;
        self.coins_created = self.coins_created.saturating_add(increment);
        self.save_balance(who, balance);

        Ok(())
    }

    pub fn sub_free_balance(
        &mut self,
        who: &T::AccountId,
        decrement: T::Balance,
    ) -> DispatchResult {
        let mut balance = self.get_or_fetch_balance(who);
        balance.free = balance
            .free
            .checked_sub(&decrement)
            .ok_or(Error::<T>::BalanceTooLow)?;
        if balance.free < balance.frozen {
            return Err(Error::<T>::BalanceLockTriggered.into());
        }
        self.coins_burned = self.coins_burned.saturating_add(decrement);
        self.save_balance(who, balance);

        Ok(())
    }

    pub fn add_reserved_balance(
        &mut self,
        who: &T::AccountId,
        increment: T::Balance,
    ) -> DispatchResult {
        let mut balance = self.get_or_fetch_balance(who);
        balance.reserved = balance
            .reserved
            .checked_add(&increment)
            .ok_or(Error::<T>::BalanceOverflow)?;
        self.coins_created = self.coins_created.saturating_add(increment);
        self.save_balance(who, balance);

        Ok(())
    }

    pub fn sub_up_to_reserved_balance(
        &mut self,
        who: &T::AccountId,
        decrement: T::Balance,
    ) -> T::Balance {
        let mut balance = self.get_or_fetch_balance(who);
        let actual_subed = balance.reserved.min(decrement);
        // We just capped `actual_subed` to `balance.reserved` itself.
        balance.reserved -= actual_subed;
        self.coins_burned = self.coins_burned.saturating_add(actual_subed);
        self.save_balance(who, balance);

        actual_subed
    }

    pub fn sub_up_to_free_balance(
        &mut self,
        who: &T::AccountId,
        decrement: T::Balance,
    ) -> T::Balance {
        let mut balance = self.get_or_fetch_balance(who);
        let actual_subed = balance.free.saturating_sub(balance.frozen).min(decrement);
        // We just capped `actual_subed` to `balance.free - balance.frozen` itself.
        balance.free -= actual_subed;
        self.coins_burned = self.coins_burned.saturating_add(actual_subed);
        self.save_balance(who, balance);

        actual_subed
    }

    pub fn frozen(&mut self, who: &T::AccountId) -> T::Balance {
        let balance = self.get_or_fetch_balance(who);
        balance.frozen
    }

    pub fn add_frozen(&mut self, who: &T::AccountId, increment: T::Balance) -> DispatchResult {
        let mut balance = self.get_or_fetch_balance(who);
        balance.frozen = balance
            .frozen
            .checked_add(&increment)
            .ok_or(Error::<T>::BalanceOverflow)?;
        self.coins_created = self.coins_created.saturating_add(increment);
        self.save_balance(who, balance);

        Ok(())
    }

    pub fn sub_frozen(&mut self, who: &T::AccountId, decrement: T::Balance) -> DispatchResult {
        let mut balance = self.get_or_fetch_balance(who);
        balance.frozen = balance
            .frozen
            .checked_sub(&decrement)
            .ok_or(Error::<T>::BalanceTooLow)?;
        self.coins_burned = self.coins_burned.saturating_sub(decrement);
        self.save_balance(who, balance);

        Ok(())
    }

    /// Does what it says and return the old balance.
    pub fn overwrite_frozen_balance(
        &mut self,
        who: &T::AccountId,
        new_balance: T::Balance,
    ) -> T::Balance {
        let mut balance = self.get_or_fetch_balance(who);
        if balance.frozen < new_balance {
            self.coins_created = self
                .coins_created
                .saturating_add(new_balance.saturating_sub(balance.frozen));
        } else {
            self.coins_burned = self
                .coins_burned
                .saturating_add(balance.frozen.saturating_sub(new_balance));
        }

        let frozen_balance_bak = balance.frozen;
        balance.frozen = new_balance;

        self.save_balance(who, balance);

        frozen_balance_bak
    }

    /// Does what it says and return the old balance.
    pub fn overwrite_free_balance(
        &mut self,
        who: &T::AccountId,
        new_balance: T::Balance,
    ) -> T::Balance {
        let mut balance = self.get_or_fetch_balance(who);
        if balance.free < new_balance {
            self.coins_created = self
                .coins_created
                .saturating_add(new_balance.saturating_sub(balance.free));
        } else {
            self.coins_burned = self
                .coins_burned
                .saturating_add(balance.free.saturating_sub(new_balance));
        }

        let free_balance_bak = balance.free;
        balance.free = new_balance;

        self.save_balance(who, balance);

        free_balance_bak
    }

    /// Reset the `coins_burned` and `coins_created` values to avoid modifiying `Totalissuances`
    /// when calling `apply`.
    pub fn forget_issuance_changes(&mut self) {
        self.coins_created = Zero::zero();
        self.coins_burned = Zero::zero();
    }

    /// Commit all the changes to the chain state or error.
    pub fn apply(self) -> DispatchResult {
        self.balances
            .iter()
            .filter(|(_account, (_bal, changed, _snapshot_was_0))| *changed)
            .map(|(account, (balance, _changed, snapshot_was_0))| {
                (account, balance, snapshot_was_0)
            })
            .for_each(|(account, balance, snapshot_0)| {
                if balance.total() == Zero::zero() {
                    if !*snapshot_0 {
                        if frame_system::Pallet::<T>::dec_providers(account).is_err() {
                            log::warn!(
                                target: "runtime::tokens",
                                "Warning: Attempt to remove lock provider reference failed. \
                                This is unexpected but should be safe."
                            );
                        }
                        frame_system::Pallet::<T>::dec_consumers(account);
                        Balances::<T>::remove(account, self.currency_id);
                    }
                } else {
                    if *snapshot_0 {
                        frame_system::Pallet::<T>::inc_providers(account);
                        if frame_system::Pallet::<T>::inc_consumers(account).is_err() {
                            log::warn!(
                                target: "runtime::tokens",
                                "Warning: Attempt to introduce lock consumer reference, yet no providers. \
                                This is unexpected but should be safe."
                            );
                        }
                    }
                    Balances::<T>::insert(account, self.currency_id, balance);
                }
            });

        if self.coins_created != self.coins_burned {
            TotalIssuances::<T>::try_mutate(self.currency_id, |d| -> DispatchResult {
                *d = d
                    .checked_sub(&self.coins_burned)
                    .ok_or(Error::<T>::TotalIssuanceUnderflow)?;
                *d = d
                    .checked_add(&self.coins_created)
                    .ok_or(Error::<T>::TotalIssuanceOverflow)?;

                Ok(())
            })?;
        }

        Ok(())
    }
}
