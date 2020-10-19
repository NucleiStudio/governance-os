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
use governance_os_support::{Currencies, ReservableCurrencies};
use sp_runtime::{
    traits::{CheckedAdd, CheckedSub, Saturating},
    DispatchError, DispatchResult,
};
use sp_std::result;

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
        let new_total = Self::total_issuances(currency_id)
            .checked_sub(&amount)
            .ok_or(Error::<T>::TotalIssuanceUnderflow)?;
        <TotalIssuances<T>>::insert(currency_id, new_total);
        Self::set_free_balance(
            currency_id,
            who,
            Self::free_balance(currency_id, who)
                .checked_sub(&amount)
                .ok_or(Error::<T>::BalanceTooLow)?,
        );

        Ok(())
    }

    fn mint(
        currency_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        let new_total = Self::total_issuances(currency_id)
            .checked_add(&amount)
            .ok_or(Error::<T>::TotalIssuanceOverflow)?;
        <TotalIssuances<T>>::insert(currency_id, new_total);
        Self::set_free_balance(
            currency_id,
            who,
            Self::free_balance(currency_id, who)
                .checked_add(&amount)
                .ok_or(Error::<T>::BalanceOverflow)?,
        );

        Ok(())
    }

    fn free_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
        Self::balances(who, currency_id).free
    }

    fn total_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
        let account_data = Self::balances(who, currency_id);
        account_data.free.saturating_add(account_data.reserved)
    }

    fn ensure_can_withdraw(
        currency_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        let _new_balance = Self::free_balance(currency_id, who)
            .checked_sub(&amount)
            .ok_or(Error::<T>::BalanceTooLow)?;
        Ok(())
    }

    fn transfer(
        currency_id: Self::CurrencyId,
        source: &T::AccountId,
        dest: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        Self::ensure_can_withdraw(currency_id, source, amount)?;

        let source_new_balance = Self::free_balance(currency_id, source)
            .checked_sub(&amount)
            .ok_or(Error::<T>::BalanceTooLow)?;
        let dest_new_balance = Self::free_balance(currency_id, dest)
            .checked_add(&amount)
            .ok_or(Error::<T>::BalanceOverflow)?;

        Self::set_free_balance(currency_id, source, source_new_balance);
        Self::set_free_balance(currency_id, dest, dest_new_balance);

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
        let reserved_balance = Self::reserved_balance(currency_id, who);
        let actual = reserved_balance.min(value);

        // Amount will be at most equal to the total reserved balance thus neglecting the
        // risk of any underflow
        <Balances<T>>::mutate(who, currency_id, |d| d.reserved -= actual);
        <TotalIssuances<T>>::mutate(currency_id, |v| *v -= actual);

        // Return whatever we couldn't slash
        value - actual
    }

    fn reserved_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
        Self::balances(who, currency_id).reserved
    }

    fn reserve(
        currency_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        Self::ensure_can_withdraw(currency_id, who, amount)?;

        // Because of the call to `ensure_can_withdraw` we now that amount is at most equal
        // to the balance of the account, thus neflecting the use for the safe math checks.
        <Balances<T>>::mutate(who, currency_id, |data| {
            data.free -= amount;
            data.reserved += amount;
        });

        Ok(())
    }

    fn unreserve(
        currency_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> Self::Balance {
        // Take the smallest between all the coins reserved or the amount
        let unreserved = Self::balances(who, currency_id).reserved.min(amount);

        // The amount will be at most equal to the reserved balance, thus we cannot have any
        // overflow / underflow situation
        <Balances<T>>::mutate(who, currency_id, |data| {
            data.free += unreserved;
            data.reserved -= unreserved;
        });

        // Return the amount of coins we couldn't unreserve
        amount - unreserved
    }

    fn repatriate_reserved(
        currency_id: Self::CurrencyId,
        slashed: &T::AccountId,
        beneficiary: &T::AccountId,
        value: Self::Balance,
        status: BalanceStatus,
    ) -> result::Result<Self::Balance, DispatchError> {
        if slashed == beneficiary {
            return match status {
                BalanceStatus::Free => Ok(Self::unreserve(currency_id, slashed, value)),
                BalanceStatus::Reserved => {
                    // If balance > value saturates to 0
                    Ok(value.saturating_sub(Self::reserved_balance(currency_id, slashed)))
                }
            };
        }

        let from_account = Self::balances(slashed, currency_id);
        let to_account = Self::balances(beneficiary, currency_id);
        let actual = from_account.reserved.min(value);
        match status {
            BalanceStatus::Free => {
                Self::set_free_balance(currency_id, beneficiary, to_account.free + actual);
            }
            BalanceStatus::Reserved => {
                <Balances<T>>::mutate(beneficiary, currency_id, |d| d.reserved += actual);
            }
        }
        <Balances<T>>::mutate(slashed, currency_id, |d| d.reserved -= actual);

        Ok(value - actual)
    }
}
