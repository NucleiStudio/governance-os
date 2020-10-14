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
use sp_runtime::{
    traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize},
    DispatchResult,
};
use sp_std::{
    cmp::{Eq, PartialEq},
    fmt::Debug,
};

/// Abstraction trait over a multiple currencies system, each currency type
/// is identified by a `CurrencyId`, if it is set to `None` when calling a
/// function the implementation should default to the native currency of the
/// runtime.
pub trait Currencies<AccountId> {
    /// The type used to identify currencies
    type CurrencyId: FullCodec + Eq + PartialEq + Copy + MaybeSerializeDeserialize + Debug;

    /// The balance of an account.
    type Balance: AtLeast32BitUnsigned
        + FullCodec
        + Copy
        + MaybeSerializeDeserialize
        + Debug
        + Default;

    // PUBLIC IMMUTABLES

    /// The combined balance of `who`.
    fn total_balance(currency_id: Self::CurrencyId, who: &AccountId) -> Self::Balance;

    /// The total amount of issuance in the system.
    fn total_issuance(currency_id: Self::CurrencyId) -> Self::Balance;

    /// Reduce the total issuance by `amount`.
    fn burn(currency_id: Self::CurrencyId, amount: Self::Balance) -> DispatchResult;

    /// Increase the total issuance by `amount`.
    fn mint(currency_id: Self::CurrencyId, amount: Self::Balance) -> DispatchResult;

    /// The 'free' balance of a given account.
    fn free_balance(currency_id: Self::CurrencyId, who: &AccountId) -> Self::Balance;

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

    /// Removes some free balance from `who`. This may check any locks, vesting, and
    /// liquidity requirements. If the removal is not possible, then it returns `Err`.
    fn withdraw(
        currency_id: Self::CurrencyId,
        who: &AccountId,
        value: Self::Balance,
    ) -> DispatchResult;
}
