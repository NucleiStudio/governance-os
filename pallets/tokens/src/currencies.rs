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
        Self::get_currency_account(currency_id, who).free
    }

    fn total_balance(currency_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
        let account_data = Self::get_currency_account(currency_id, who);
        account_data.free.saturating_add(account_data.reserved)
    }

    fn ensure_can_withdraw(
        currency_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        // First verify permissions
        if !RoleManagerOf::<T>::has_role(who, RoleBuilderOf::<T>::transfer_currency(currency_id)) {
            return Err(Error::<T>::UnTransferableCurrency.into());
        }

        // Then balances
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

        Self::set_free_balance(
            currency_id,
            source,
            Self::free_balance(currency_id, source)
                .checked_sub(&amount)
                .ok_or(Error::<T>::BalanceTooLow)?,
        );
        Self::set_free_balance(
            currency_id,
            dest,
            Self::free_balance(currency_id, dest)
                .checked_add(&amount)
                .ok_or(Error::<T>::BalanceOverflow)?,
        );

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
        Self::mutate_currency_account(currency_id, who, |data| data.reserved -= actual);
        <TotalIssuances<T>>::mutate(currency_id, |v| *v -= actual);

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
        Self::ensure_can_withdraw(currency_id, who, amount)?;

        // Because of the call to `ensure_can_withdraw` we now that amount is at most equal
        // to the balance of the account, thus neflecting the use for the safe math checks.
        Self::mutate_currency_account(currency_id, who, |data| {
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
        let unreserved = Self::get_currency_account(currency_id, who)
            .reserved
            .min(amount);

        // The amount will be at most equal to the reserved balance, thus we cannot have any
        // overflow / underflow situation
        Self::mutate_currency_account(currency_id, who, |data| {
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

        let from_account = Self::get_currency_account(currency_id, slashed);
        let to_account = Self::get_currency_account(currency_id, beneficiary);
        let actual = from_account.reserved.min(value);
        match status {
            BalanceStatus::Free => {
                Self::set_free_balance(currency_id, beneficiary, to_account.free + actual);
            }
            BalanceStatus::Reserved => {
                Self::mutate_currency_account(currency_id, beneficiary, |d| d.reserved += actual);
            }
        }
        Self::mutate_currency_account(currency_id, slashed, |d| d.reserved -= actual);

        Ok(value - actual)
    }
}
