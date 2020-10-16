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
use sp_runtime::{
    traits::{CheckedAdd, CheckedSub, Saturating},
    DispatchResult,
};

impl<T: Trait> governance_os_support::Currencies<T::AccountId> for Module<T> {
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
