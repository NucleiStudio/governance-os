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

//! A collection of traits and helpers to create a runtime that can support
//! multiple currencies by implementing a trait similar to `Currency`. We
//! tried to maintain API compatibility between the native `Currency` and
//! this one.

use codec::FullCodec;
use frame_support::traits::BalanceStatus;
use sp_runtime::{
    traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize},
    DispatchError, DispatchResult,
};
use sp_std::{
    cmp::{Eq, PartialEq},
    fmt::Debug,
    result,
};

/// Abstraction trait over a multiple currencies system, each currency type
/// is identified by a `CurrencyId`, if it is set to `None` when calling a
/// function the implementation should default to the native currency of the
/// runtime.
pub trait Currencies<AccountId> {
    /// The type used to identify currencies
    type CurrencyId: FullCodec + Eq + PartialEq + Copy + MaybeSerializeDeserialize + Debug + Default;

    /// The balance of an account.
    type Balance: AtLeast32BitUnsigned
        + FullCodec
        + Copy
        + MaybeSerializeDeserialize
        + Debug
        + Default;

    // PUBLIC IMMUTABLES

    /// The total amount of issuance in the system.
    fn total_issuance(currency_id: Self::CurrencyId) -> Self::Balance;

    /// Remove `amount` tokens from `who` balance. If there are not enough tokens
    /// it will error.
    fn burn(
        currency_id: Self::CurrencyId,
        who: &AccountId,
        amount: Self::Balance,
    ) -> DispatchResult;

    /// Increase the balance of `who` by `amount`.
    fn mint(
        currency_id: Self::CurrencyId,
        who: &AccountId,
        amount: Self::Balance,
    ) -> DispatchResult;

    /// The 'free' balance of a given account.
    fn free_balance(currency_id: Self::CurrencyId, who: &AccountId) -> Self::Balance;

    /// The total balance of a given account. This may include non `free` funds.
    fn total_balance(currency_id: Self::CurrencyId, who: &AccountId) -> Self::Balance;

    /// Returns `Ok` if the account is able to make a withdrawal of the given amount.
    /// Basically, it's just a dry-run of `withdraw`.
    ///
    /// `Err(...)` with the reason why not otherwise.
    fn ensure_can_withdraw(
        currency_id: Self::CurrencyId,
        who: &AccountId,
        amount: Self::Balance,
    ) -> DispatchResult;

    // PUBLIC MUTABLES (DANGEROUS)

    /// Transfer some liquid free balance to another account.
    ///
    /// This is a very high-level function. It will ensure all appropriate fees are paid
    /// and no imbalance in the system remains.
    fn transfer(
        currency_id: Self::CurrencyId,
        source: &AccountId,
        dest: &AccountId,
        value: Self::Balance,
    ) -> DispatchResult;
}

/// An extension of the `Currencies` trait to allow the runtime to reserve
/// funds from the token holders.
pub trait ReservableCurrencies<AccountId>: Currencies<AccountId> {
    /// Same result as `reserve(who, value)` (but without the side-effects) assuming there
    /// are no balance changes in the meantime.
    fn can_reserve(currency_id: Self::CurrencyId, who: &AccountId, value: Self::Balance) -> bool;

    /// Deducts up to `value` from reserved balance of `who`. This function cannot fail.
    ///
    /// As much funds up to `value` will be deducted as possible. If the reserve balance of `who`
    /// is less than `value`, then a non-zero second item will be returned.
    ///
    /// This will update the total issuance of the currency.
    fn slash_reserved(
        currency_id: Self::CurrencyId,
        who: &AccountId,
        value: Self::Balance,
    ) -> Self::Balance;

    /// The amount of the balance of a given account that is externally reserved; this can still get
    /// slashed, but gets slashed last of all.
    ///
    /// This balance is a 'reserve' balance that other subsystems use in order to set aside tokens
    /// that are still 'owned' by the account holder, but which are suspendable.
    fn reserved_balance(currency_id: Self::CurrencyId, who: &AccountId) -> Self::Balance;

    /// Moves `value` from balance to reserved balance.
    ///
    /// If the free balance is lower than `value`, then no funds will be moved and an `Err` will
    /// be returned to notify of this. This is different behavior than `unreserve`.
    fn reserve(
        currency_id: Self::CurrencyId,
        who: &AccountId,
        value: Self::Balance,
    ) -> DispatchResult;

    /// Moves up to `value` from reserved balance to free balance. This function cannot fail.
    ///
    /// As much funds up to `value` will be moved as possible. If the reserve balance of `who`
    /// is less than `value`, then the remaining amount will be returned.
    ///
    /// # NOTES
    ///
    /// - This is different from `reserve`.
    fn unreserve(
        currency_id: Self::CurrencyId,
        who: &AccountId,
        value: Self::Balance,
    ) -> Self::Balance;

    /// Moves up to `value` from reserved balance of account `slashed` to balance of account
    /// `beneficiary`. `beneficiary` must exist for this to succeed. If it does not, `Err` will be
    /// returned. Funds will be placed in either the `free` balance or the `reserved` balance,
    /// depending on the `status`.
    ///
    /// As much funds up to `value` will be deducted as possible. If this is less than `value`,
    /// then `Ok(non_zero)` will be returned.
    fn repatriate_reserved(
        currency_id: Self::CurrencyId,
        slashed: &AccountId,
        beneficiary: &AccountId,
        value: Self::Balance,
        status: BalanceStatus,
    ) -> result::Result<Self::Balance, DispatchError>;
}
