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

use codec::{Codec, FullCodec};
use frame_support::traits::Currency;
use sp_runtime::{
    traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize},
    DispatchError, DispatchResult,
};
use sp_std::{
    cmp::{Eq, PartialEq},
    convert::{TryFrom, TryInto},
    fmt::Debug,
    result,
};

/// Abstraction trait over a multiple currencies system, each currency type
/// is identified by a `CurrencyId`, if it is set to `None` when calling a
/// function the implementation should default to the native currency of the
/// runtime.
pub trait Currencies<AccountId> {
    /// The type used to identify currencies
    type CurrencyId: FullCodec + Eq + PartialEq + Copy + MaybeSerializeDeserialize + Debug;

    /// The native currency type, used when `CurrencyId` is set to `None` in
    /// function calls
    type NativeCurrency: Currency<AccountId>;

    /// The balance of an account.
    type Balance: AtLeast32BitUnsigned
        + FullCodec
        + Copy
        + MaybeSerializeDeserialize
        + Debug
        + Default;

    /// The combined balance of `who` for the currency specified by `currency_id`, if
    /// set to `None` we default to the `NativeCurrency` type.
    fn total_balance(currency_id: Option<Self::CurrencyId>, who: &AccountId) -> Self::Balance;

    /// The total amount of issuance in the system for the currency specified by `currency_id`,
    /// if set to `None` we default to the `NativeCurrency` type.
    fn total_issuance(currency_id: Option<Self::CurrencyId>) -> Self::Balance;

    /// Returns `Ok` if the account has enough funds to complete a call to `withdraw` with
    /// similar parameters without an error, if not return an `Err(...)` with some explanation
    /// inside.
    fn ensure_can_withdraw(
        currency_id: Option<Self::CurrencyId>,
        who: &AccountId,
        amount: Self::Balance,
    ) -> DispatchResult;

    /// Transfer some balance between two accounts for the currency specified
    /// by `currency_id`, if set to `None` we default to the `NativeCurrency` type.
    fn transfer(
        currency_id: Option<Self::CurrencyId>,
        source: &AccountId,
        dest: &AccountId,
        value: Self::Balance,
    ) -> DispatchResult;

    /// Add `amount` to the balance of `who` and increases the total issuance for the
    /// currency specified by `currency_id`, if set to `None` we default to the
    /// `NativeCurrency` type.
    fn issue(
        currency_id: Option<Self::CurrencyId>,
        who: &AccountId,
        amount: Self::Balance,
    ) -> DispatchResult;

    /// Removes some free balance from `who` and reduce the total issuance for the
    /// currency specified by `currency_id`, if set to `None` we default to the
    /// `NativeCurrency` type.
    fn burn(
        currency_id: Option<Self::CurrencyId>,
        who: &AccountId,
        value: Self::Balance,
    ) -> DispatchResult;
}
