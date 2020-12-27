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

use crate::*;
use frame_support::traits::BalanceStatus;
use governance_os_support::traits::{Currencies, ReservableCurrencies};
use sp_runtime::{
    traits::{CheckedAdd, CheckedSub, Saturating},
    DispatchError, DispatchResult,
};
use sp_std::{collections::btree_map::BTreeMap, marker};

/// An internal helper to represent balance changes. It is used to express in a better manner
/// operations done on balances while saving on weight costs by fetching the required data
/// only when necessary and forwarding errors approprietaly.
#[derive(Clone)]
struct Mutation<T: Trait> {
    currency_id: T::CurrencyId,
    balances: BTreeMap<T::AccountId, (AccountCurrencyData<T::Balance>, bool)>,
    coins_created: T::Balance,
    coins_burned: T::Balance,
    _phantom: marker::PhantomData<T>,
}
impl<T: Trait> Mutation<T> {
    fn new_for_currency(currency_id: T::CurrencyId) -> Self {
        Self {
            currency_id: currency_id,
            balances: BTreeMap::new(),
            coins_created: 0.into(),
            coins_burned: 0.into(),
            _phantom: marker::PhantomData,
        }
    }

    /// Return a balance from the `self.balances` cache or fetch it from the node DB.
    fn get_or_fetch_balance(&mut self, who: &T::AccountId) -> AccountCurrencyData<T::Balance> {
        let in_cache = self.balances.get(who);
        match in_cache {
            None => {
                let in_node = Balances::<T>::get(who, self.currency_id);
                self.balances.insert(who.clone(), (in_node.clone(), false));

                in_node
            }
            Some(cache) => cache.clone().0,
        }
    }

    /// Save an updated balance in our memory cache
    fn save_balance(&mut self, who: &T::AccountId, balance: AccountCurrencyData<T::Balance>) {
        // Note how the boolean is set to true since we modified the balance
        self.balances.insert(who.clone(), (balance, true));
    }

    /// Verify that the currency is transferable
    fn ensure_must_be_transferable_for(&mut self, who: &T::AccountId) -> DispatchResult {
        if !RoleManagerOf::<T>::has_role(
            who,
            RoleBuilderOf::<T>::transfer_currency(self.currency_id),
        ) {
            return Err(Error::<T>::UnTransferableCurrency.into());
        }

        Ok(())
    }

    fn add_free_balance(&mut self, who: &T::AccountId, increment: T::Balance) -> DispatchResult {
        let mut balance = self.get_or_fetch_balance(who);
        balance.free = balance
            .free
            .checked_add(&increment)
            .ok_or(Error::<T>::BalanceOverflow)?;
        self.coins_created = self.coins_created.saturating_add(increment);
        self.save_balance(who, balance);

        Ok(())
    }

    fn sub_free_balance(&mut self, who: &T::AccountId, decrement: T::Balance) -> DispatchResult {
        let mut balance = self.get_or_fetch_balance(who);
        balance.free = balance
            .free
            .checked_sub(&decrement)
            .ok_or(Error::<T>::BalanceTooLow)?;
        self.coins_burned = self.coins_burned.saturating_add(decrement);
        self.save_balance(who, balance);

        Ok(())
    }

    fn add_reserved_balance(
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

    fn sub_up_to_reserved_balance(
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

    /// Commit all the changes to the chain state or error.
    fn apply(self) -> DispatchResult {
        self.balances
            .iter()
            .filter(|(_account, (_bal, changed))| *changed)
            .map(|(account, (balance, _changed))| (account, balance))
            .for_each(|(account, balance)| {
                if balance.total() == Zero::zero() {
                    Balances::<T>::remove(account, self.currency_id);
                } else {
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

impl<T: Trait> Currencies<T::AccountId> for Module<T> {
    type CurrencyId = T::CurrencyId;
    type Balance = T::Balance;

    fn total_issuance(currency_id: Self::CurrencyId) -> Self::Balance {
        Self::total_issuances(currency_id)
    }

    fn burn(
        currency_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        let mut mutation = Mutation::<T>::new_for_currency(currency_id);
        mutation.sub_free_balance(who, amount)?;
        mutation.apply()?;

        Self::deposit_event(RawEvent::CurrencyBurned(currency_id, who.clone(), amount));
        Ok(())
    }

    fn mint(
        currency_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        let mut mutation = Mutation::<T>::new_for_currency(currency_id);
        mutation.add_free_balance(who, amount)?;
        mutation.apply()?;

        Self::deposit_event(RawEvent::CurrencyMinted(currency_id, who.clone(), amount));
        Ok(())
    }

    fn free_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
        Self::get_currency_account(currency_id, who).free
    }

    fn total_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
        Self::get_currency_account(currency_id, who).total()
    }

    fn ensure_can_withdraw(
        currency_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        // We simulate a withdrawal but never executes it and rather returns any error that
        // happens along the way
        let mut mutation = Mutation::<T>::new_for_currency(currency_id);
        mutation.ensure_must_be_transferable_for(who)?;
        mutation.sub_free_balance(who, amount)?;

        Ok(())
    }

    fn transfer(
        currency_id: Self::CurrencyId,
        source: &T::AccountId,
        dest: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        let mut mutation = Mutation::<T>::new_for_currency(currency_id);
        mutation.ensure_must_be_transferable_for(source)?;
        mutation.sub_free_balance(source, amount)?;
        mutation.add_free_balance(dest, amount)?;
        mutation.apply()?;

        Self::deposit_event(RawEvent::CurrencyTransferred(
            currency_id,
            source.clone(),
            dest.clone(),
            amount,
        ));
        Ok(())
    }
}

impl<T: Trait> ReservableCurrencies<T::AccountId> for Module<T> {
    fn can_reserve(
        currency_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> bool {
        Self::ensure_can_withdraw(currency_id, who, amount).is_ok()
    }

    fn slash_reserved(
        currency_id: Self::CurrencyId,
        who: &T::AccountId,
        value: Self::Balance,
    ) -> Self::Balance {
        let mut mutation = Mutation::<T>::new_for_currency(currency_id);
        let actual = mutation.sub_up_to_reserved_balance(who, value);
        mutation
            .apply()
            .expect("we assume the coins were created and added to the total issuance previously");

        // Return whatever we couldn't slash
        value - actual
    }

    fn reserved_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
        Self::get_currency_account(currency_id, who).reserved
    }

    fn reserve(
        currency_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        // We do not require the asset to be transferable, it is assume that it is acceptable
        // to reserve non transferable currencies
        let mut mutation = Mutation::<T>::new_for_currency(currency_id);
        mutation.sub_free_balance(who, amount)?;
        mutation.add_reserved_balance(who, amount)?;
        mutation.apply()?;

        Ok(())
    }

    fn unreserve(
        currency_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> Self::Balance {
        let mut mutation = Mutation::<T>::new_for_currency(currency_id);
        let unreserved = mutation.sub_up_to_reserved_balance(who, amount);
        mutation
            .add_free_balance(who, unreserved)
            .expect("we are merely reallocating balances");
        mutation
            .apply()
            .expect("we are not modifiying the total issuance and thus do not expect any errors");

        // Return the amount of coins we couldn't unreserve
        amount - unreserved
    }

    fn repatriate_reserved(
        currency_id: Self::CurrencyId,
        slashed: &T::AccountId,
        beneficiary: &T::AccountId,
        value: Self::Balance,
        status: BalanceStatus,
    ) -> Result<Self::Balance, DispatchError> {
        if slashed == beneficiary {
            return match status {
                BalanceStatus::Free => Ok(Self::unreserve(currency_id, slashed, value)),
                BalanceStatus::Reserved => {
                    // If balance > value saturates to 0
                    Ok(value.saturating_sub(Self::reserved_balance(currency_id, slashed)))
                }
            };
        }

        let mut mutation = Mutation::<T>::new_for_currency(currency_id);
        let actual = mutation.sub_up_to_reserved_balance(slashed, value);
        match status {
            BalanceStatus::Free => {
                mutation.add_free_balance(beneficiary, actual)?;
            }
            BalanceStatus::Reserved => {
                mutation.add_reserved_balance(beneficiary, actual)?;
            }
        };
        mutation.apply()?;

        Ok(value - actual)
    }
}
