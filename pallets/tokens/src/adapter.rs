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
    imbalances::{NegativeImbalance, PositiveImbalance},
    mutations::Mutation,
    Config, Module, TotalIssuances,
};
use frame_support::{
    traits::{
        BalanceStatus, Currency, ExistenceRequirement, Get, Imbalance, LockIdentifier,
        LockableCurrency, ReservableCurrency, SignedImbalance, WithdrawReasons,
    },
    StorageMap,
};
use governance_os_support::traits::{Currencies, LockableCurrencies, ReservableCurrencies};
use sp_runtime::{
    traits::{Bounded, CheckedAdd, CheckedSub, Saturating, Zero},
    DispatchError, DispatchResult,
};
use sp_std::marker;

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
    Pallet: Config,
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

        let mut mutation = Mutation::<Pallet>::new_for_currency(GetCurrencyId::get());
        let free_slashed = mutation.sub_up_to_free_balance(who, amount);

        // Still has some to slash, we try to take the remaining coins from the
        // reserved balance.
        let slashed = {
            if free_slashed != Zero::zero() {
                let reserved_slashed =
                    mutation.sub_up_to_reserved_balance(who, amount.saturating_sub(free_slashed));
                free_slashed.saturating_add(reserved_slashed)
            } else {
                free_slashed
            }
        };

        mutation.forget_issuance_changes();
        mutation
            .apply()
            .expect("we just forgot issuance changes which were the only error source");

        // `slashed` is at most equal to `amount`
        (Self::NegativeImbalance::new(slashed), amount - slashed)
    }

    fn deposit_into_existing(
        who: &Pallet::AccountId,
        amount: Self::Balance,
    ) -> Result<Self::PositiveImbalance, DispatchError> {
        <Module<Pallet> as Currencies<Pallet::AccountId>>::mint(GetCurrencyId::get(), who, amount)?;
        Ok(Self::PositiveImbalance::new(amount))
    }

    fn deposit_creating(who: &Pallet::AccountId, amount: Self::Balance) -> Self::PositiveImbalance {
        Self::deposit_into_existing(who, amount).unwrap_or_else(|_| Self::PositiveImbalance::zero())
    }

    fn withdraw(
        who: &Pallet::AccountId,
        value: Self::Balance,
        _: WithdrawReasons,
        _: ExistenceRequirement,
    ) -> Result<Self::NegativeImbalance, DispatchError> {
        // Unlike `Currencies::burn`, this isn't supposed to reduce the total token supply

        let mut mutation = Mutation::<Pallet>::new_for_currency(GetCurrencyId::get());
        mutation.ensure_must_be_transferable_for(who)?;
        mutation.sub_free_balance(who, value)?;
        mutation.forget_issuance_changes();
        mutation.apply()?;

        Ok(Self::NegativeImbalance::new(value))
    }

    fn make_free_balance_be(
        who: &Pallet::AccountId,
        value: Self::Balance,
    ) -> SignedImbalance<Self::Balance, Self::PositiveImbalance> {
        // This create an imbalance since some coins have to be either burned or
        // reallocated somewhere else.

        let mut mutation = Mutation::<Pallet>::new_for_currency(GetCurrencyId::get());
        let old_balance = mutation.overwrite_free_balance(who, value);
        mutation.forget_issuance_changes();
        mutation
            .apply()
            .expect("we just forgot issuance changes which were the only error source");

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
    Pallet: Config,
    GetCurrencyId: Get<Pallet::CurrencyId>,
{
    fn can_reserve(who: &Pallet::AccountId, amount: Self::Balance) -> bool {
        Module::<Pallet>::can_reserve(GetCurrencyId::get(), who, amount)
    }

    fn slash_reserved(
        who: &Pallet::AccountId,
        amount: Self::Balance,
    ) -> (Self::NegativeImbalance, Self::Balance) {
        let mut mutation = Mutation::<Pallet>::new_for_currency(GetCurrencyId::get());
        let slashed = mutation.sub_up_to_reserved_balance(who, amount);
        mutation.forget_issuance_changes();
        mutation
            .apply()
            .expect("we just forgot issuance changes which were the only error source");

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
    ) -> Result<Self::Balance, DispatchError> {
        Module::<Pallet>::repatriate_reserved(
            GetCurrencyId::get(),
            slashed,
            beneficiary,
            amount,
            status,
        )
    }
}

impl<Pallet, GetCurrencyId> LockableCurrency<Pallet::AccountId>
    for NativeCurrencyAdapter<Pallet, GetCurrencyId>
where
    Pallet: Config,
    GetCurrencyId: Get<Pallet::CurrencyId>,
{
    type Moment = Pallet::BlockNumber;
    type MaxLocks = ();

    fn set_lock(
        id: LockIdentifier,
        who: &Pallet::AccountId,
        amount: Self::Balance,
        _reasons: WithdrawReasons,
    ) {
        drop(Module::<Pallet>::set_lock(
            GetCurrencyId::get(),
            id,
            who,
            amount,
        ))
    }

    fn extend_lock(
        id: LockIdentifier,
        who: &Pallet::AccountId,
        amount: Self::Balance,
        _reasons: WithdrawReasons,
    ) {
        drop(Module::<Pallet>::extend_lock(
            GetCurrencyId::get(),
            id,
            who,
            amount,
        ))
    }

    fn remove_lock(id: LockIdentifier, who: &Pallet::AccountId) {
        drop(Module::<Pallet>::remove_lock(GetCurrencyId::get(), id, who))
    }
}
