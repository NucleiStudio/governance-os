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

use crate::{mutations::Mutation, Locks, Module, RawEvent, Trait};
use frame_support::{
    traits::{BalanceStatus, LockIdentifier},
    StorageDoubleMap,
};
use governance_os_support::traits::{Currencies, LockableCurrencies, ReservableCurrencies};
use sp_runtime::{traits::Saturating, DispatchError, DispatchResult};

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
        // We do not require the asset to be transferable, it is assumed that it is acceptable
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

// Some helper to avoid code repetition when using locks
impl<T: Trait> Module<T> {
    fn conditionally_write_lock<Cond: FnOnce(T::Balance) -> bool>(
        currency_id: T::CurrencyId,
        lock_id: LockIdentifier,
        who: &T::AccountId,
        amount: T::Balance,
        condition: Cond,
    ) -> DispatchResult {
        Locks::<T>::try_mutate_exists(
            (who, currency_id),
            lock_id,
            |maybe_existing_lock| -> DispatchResult {
                let mut mutation = Mutation::<T>::new_for_currency(currency_id);

                if maybe_existing_lock.is_none() {
                    // We are creating a new lock. Add a few checks to handle cases when
                    // more than one lock is present.
                    if mutation.frozen(who) < amount {
                        mutation.add_frozen(who, amount)?;
                    }
                } else {
                    // We are overwriting an existing lock
                    let existing_lock = maybe_existing_lock
                        .take()
                        .expect("we just did a is_none check");

                    // If it isn't necessary to change anything we stop here
                    if !condition(existing_lock) {
                        return Ok(());
                    }

                    // We check that it is needed to increase the locked amounts,
                    // or not.
                    if mutation.frozen(who).saturating_sub(existing_lock) < amount {
                        // We use the fact that the mutation helper keeps things in memory,
                        // so substracting and adding values to a balance does not require
                        // multiple DB reads.
                        mutation.sub_frozen(who, existing_lock)?;
                        mutation.add_frozen(who, amount)?;
                    }
                }

                // Update balance
                mutation.apply()?;

                // Write the lock
                *maybe_existing_lock = Some(amount);

                Ok(())
            },
        )
    }
}

impl<T: Trait> LockableCurrencies<T::AccountId> for Module<T> {
    fn set_lock(
        currency_id: Self::CurrencyId,
        lock_id: LockIdentifier,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        Self::conditionally_write_lock(currency_id, lock_id, who, amount, |_| true)
    }

    fn extend_lock(
        currency_id: Self::CurrencyId,
        lock_id: LockIdentifier,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        Self::conditionally_write_lock(currency_id, lock_id, who, amount, |existing_lock| {
            existing_lock < amount
        })
    }

    fn remove_lock(
        currency_id: Self::CurrencyId,
        lock_id: LockIdentifier,
        who: &T::AccountId,
    ) -> DispatchResult {
        unimplemented!()
    }
}
