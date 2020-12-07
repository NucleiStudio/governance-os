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
use frame_support::traits::{
    BalanceStatus, Currency, ExistenceRequirement, Get, Imbalance, ReservableCurrency,
    SignedImbalance, WithdrawReasons,
};
use governance_os_support::traits::{Currencies, ReservableCurrencies};
use imbalances::{NegativeImbalance, PositiveImbalance};
use sp_runtime::{
    traits::{Bounded, CheckedAdd, CheckedSub, Zero},
    DispatchError, DispatchResult,
};
use sp_std::{marker, result};

/// This struct is useful to implement the `Currency` trait for any given
/// currency inside the system. It basically takes an interface to the
/// `tokens` pallet and a `CurrencyId` to expose them under the `Currency`
/// trait.
pub struct NativeCurrencyAdapter<Pallet, GetCurrencyId>(
    marker::PhantomData<(Pallet, GetCurrencyId)>,
);

impl<Pallet, GetCurrencyId> Currency<Pallet::AccountId>
    for NativeCurrencyAdapter<Pallet, GetCurrencyId>
where
    Pallet: Trait,
    GetCurrencyId: Get<Pallet::CurrencyId>,
{
    type Balance = Pallet::Balance;
    type PositiveImbalance = PositiveImbalance<Pallet, GetCurrencyId>;
    type NegativeImbalance = NegativeImbalance<Pallet, GetCurrencyId>;

    fn total_balance(who: &Pallet::AccountId) -> Self::Balance {
        Module::<Pallet>::total_balance(GetCurrencyId::get(), who)
    }

    fn can_slash(who: &Pallet::AccountId, amount: Self::Balance) -> bool {
        Self::free_balance(who) >= amount
    }

    fn total_issuance() -> Self::Balance {
        Module::<Pallet>::total_issuance(GetCurrencyId::get())
    }

    fn minimum_balance() -> Self::Balance {
        Zero::zero()
    }

    fn burn(mut amount: Self::Balance) -> Self::PositiveImbalance {
        // Reduce the total issuance but doesn't reduce the balance of
        // any token holder, hence the imbalance.

        <TotalIssuances<Pallet>>::mutate(GetCurrencyId::get(), |issued| {
            *issued = issued.checked_sub(&amount).unwrap_or_else(|| {
                amount = *issued;
                Zero::zero()
            });
        });
        PositiveImbalance::new(amount)
    }

    fn issue(mut amount: Self::Balance) -> Self::NegativeImbalance {
        // Bump the total issuance but don't credit the coins to anybody
        // hence the imbalance.

        <TotalIssuances<Pallet>>::mutate(GetCurrencyId::get(), |issued| {
            *issued = issued.checked_add(&amount).unwrap_or_else(|| {
                amount = Self::Balance::max_value() - *issued;
                Self::Balance::max_value()
            })
        });
        NegativeImbalance::new(amount)
    }

    fn free_balance(who: &Pallet::AccountId) -> Self::Balance {
        Module::<Pallet>::free_balance(GetCurrencyId::get(), who)
    }

    fn ensure_can_withdraw(
        who: &Pallet::AccountId,
        amount: Self::Balance,
        _: WithdrawReasons,
        _: Self::Balance,
    ) -> DispatchResult {
        Module::<Pallet>::ensure_can_withdraw(GetCurrencyId::get(), who, amount)
    }

    fn transfer(
        source: &Pallet::AccountId,
        dest: &Pallet::AccountId,
        amount: Self::Balance,
        _: ExistenceRequirement,
    ) -> DispatchResult {
        <Module<Pallet> as Currencies<Pallet::AccountId>>::transfer(
            GetCurrencyId::get(),
            source,
            dest,
            amount,
        )
    }

    fn slash(
        who: &Pallet::AccountId,
        amount: Self::Balance,
    ) -> (Self::NegativeImbalance, Self::Balance) {
        // Return slashed, unslashed (ex: not enough balance)

        let mut slashed = amount;
        let mut left_to_be_slashed: Self::Balance = 0.into();
        let old_balance = Self::free_balance(who);
        Module::<Pallet>::set_free_balance(
            GetCurrencyId::get(),
            who,
            old_balance.checked_sub(&amount).unwrap_or_else(|| {
                // Balance is too low to slash everything, slash all the balance and log this
                left_to_be_slashed = amount - old_balance; // What's left, can't overfloww cause we enter here only if amount > old_balance
                slashed = old_balance;

                0.into()
            }),
        );

        (Self::NegativeImbalance::new(slashed), left_to_be_slashed)
    }

    fn deposit_into_existing(
        who: &Pallet::AccountId,
        amount: Self::Balance,
    ) -> result::Result<Self::PositiveImbalance, DispatchError> {
        <Module<Pallet> as Currencies<Pallet::AccountId>>::mint(GetCurrencyId::get(), who, amount)?;
        Ok(Self::PositiveImbalance::new(amount))
    }

    fn deposit_creating(who: &Pallet::AccountId, amount: Self::Balance) -> Self::PositiveImbalance {
        Self::deposit_into_existing(who, amount).unwrap_or_else(|_| Self::PositiveImbalance::zero())
    }

    fn withdraw(
        who: &Pallet::AccountId,
        value: Self::Balance,
        reasons: WithdrawReasons,
        _: ExistenceRequirement,
    ) -> result::Result<Self::NegativeImbalance, DispatchError> {
        // Unlike `Currencies::burn`, this isn't supposed to reduce the total token supply

        Self::ensure_can_withdraw(who, value, reasons, 0.into())?;
        let currency_id = GetCurrencyId::get();
        Module::<Pallet>::set_free_balance(
            currency_id,
            who,
            // `ensure_can_withdraw` already does the math checks
            Module::<Pallet>::free_balance(currency_id, who) - value,
        );

        Ok(Self::NegativeImbalance::new(value))
    }

    fn make_free_balance_be(
        who: &Pallet::AccountId,
        value: Self::Balance,
    ) -> SignedImbalance<Self::Balance, Self::PositiveImbalance> {
        // This create an imbalance since some coins have to be either burned or
        // reallocated somewhere else.

        let old_balance = Self::free_balance(who);
        Module::<Pallet>::set_free_balance(GetCurrencyId::get(), who, value);
        if old_balance <= value {
            SignedImbalance::Positive(PositiveImbalance::new(value - old_balance))
        } else {
            SignedImbalance::Negative(NegativeImbalance::new(old_balance - value))
        }
    }
}

impl<Pallet, GetCurrencyId> ReservableCurrency<Pallet::AccountId>
    for NativeCurrencyAdapter<Pallet, GetCurrencyId>
where
    Pallet: Trait,
    GetCurrencyId: Get<Pallet::CurrencyId>,
{
    fn can_reserve(who: &Pallet::AccountId, amount: Self::Balance) -> bool {
        Module::<Pallet>::can_reserve(GetCurrencyId::get(), who, amount)
    }

    fn slash_reserved(
        who: &Pallet::AccountId,
        amount: Self::Balance,
    ) -> (Self::NegativeImbalance, Self::Balance) {
        let mut slashed = amount;
        Module::<Pallet>::mutate_currency_account(GetCurrencyId::get(), who, |data| {
            slashed = data.reserved.min(amount);
            // Slashed will be at most equal to data.reserved, no underflow
            data.reserved -= slashed;
        });

        (Self::NegativeImbalance::new(slashed), amount - slashed)
    }

    fn reserved_balance(who: &Pallet::AccountId) -> Self::Balance {
        Module::<Pallet>::reserved_balance(GetCurrencyId::get(), who)
    }

    fn reserve(who: &Pallet::AccountId, amount: Self::Balance) -> DispatchResult {
        Module::<Pallet>::reserve(GetCurrencyId::get(), who, amount)
    }

    fn unreserve(who: &Pallet::AccountId, amount: Self::Balance) -> Self::Balance {
        Module::<Pallet>::unreserve(GetCurrencyId::get(), who, amount)
    }

    fn repatriate_reserved(
        slashed: &Pallet::AccountId,
        beneficiary: &Pallet::AccountId,
        amount: Self::Balance,
        status: BalanceStatus,
    ) -> result::Result<Self::Balance, DispatchError> {
        Module::<Pallet>::repatriate_reserved(
            GetCurrencyId::get(),
            slashed,
            beneficiary,
            amount,
            status,
        )
    }
}
