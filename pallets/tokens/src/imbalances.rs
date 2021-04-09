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

// Shamelessly took this from `pallet_balances` and modified it.
// Kuddos to the work on open-runtime-library from Acala & Laminar
// that greatly helped here too.

use crate::*;
use frame_support::traits::{Get, Imbalance, TryDrop};
use sp_runtime::traits::{Saturating, Zero};
use sp_std::{marker, mem, result};

/// Opaque, move-only struct with private fields that serves as a token denoting that
/// funds have been created without any equal and opposite accounting.
#[must_use]
pub struct PositiveImbalance<T: Config, GetCurrencyId: Get<T::CurrencyId>>(
    T::Balance,
    marker::PhantomData<GetCurrencyId>,
);

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> PositiveImbalance<T, GetCurrencyId> {
    /// Create a new positive imbalance from a balance.
    pub fn new(amount: T::Balance) -> Self {
        PositiveImbalance(amount, marker::PhantomData::<GetCurrencyId>)
    }
}

/// Opaque, move-only struct with private fields that serves as a token denoting that
/// funds have been destroyed without any equal and opposite accounting.
#[must_use]
pub struct NegativeImbalance<T: Config, GetCurrencyId: Get<T::CurrencyId>>(
    T::Balance,
    marker::PhantomData<GetCurrencyId>,
);

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> NegativeImbalance<T, GetCurrencyId> {
    /// Create a new negative imbalance from a balance.
    pub fn new(amount: T::Balance) -> Self {
        NegativeImbalance(amount, marker::PhantomData::<GetCurrencyId>)
    }
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> TryDrop for PositiveImbalance<T, GetCurrencyId> {
    fn try_drop(self) -> result::Result<(), Self> {
        self.drop_zero()
    }
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> Imbalance<T::Balance>
    for PositiveImbalance<T, GetCurrencyId>
{
    type Opposite = NegativeImbalance<T, GetCurrencyId>;

    fn zero() -> Self {
        Self::new(Zero::zero())
    }
    fn drop_zero(self) -> result::Result<(), Self> {
        if self.0.is_zero() {
            Ok(())
        } else {
            Err(self)
        }
    }
    fn split(self, amount: T::Balance) -> (Self, Self) {
        let first = self.0.min(amount);
        let second = self.0 - first;

        mem::forget(self);
        (Self::new(first), Self::new(second))
    }
    fn merge(mut self, other: Self) -> Self {
        self.0 = self.0.saturating_add(other.0);
        mem::forget(other);

        self
    }
    fn subsume(&mut self, other: Self) {
        self.0 = self.0.saturating_add(other.0);
        mem::forget(other);
    }
    fn offset(self, other: Self::Opposite) -> result::Result<Self, Self::Opposite> {
        let (a, b) = (self.0, other.0);
        mem::forget((self, other));

        if a >= b {
            Ok(Self::new(a - b))
        } else {
            Err(NegativeImbalance::new(b - a))
        }
    }
    fn peek(&self) -> T::Balance {
        self.0.clone()
    }
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> TryDrop for NegativeImbalance<T, GetCurrencyId> {
    fn try_drop(self) -> result::Result<(), Self> {
        self.drop_zero()
    }
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> Imbalance<T::Balance>
    for NegativeImbalance<T, GetCurrencyId>
{
    type Opposite = PositiveImbalance<T, GetCurrencyId>;

    fn zero() -> Self {
        Self::new(Zero::zero())
    }
    fn drop_zero(self) -> result::Result<(), Self> {
        if self.0.is_zero() {
            Ok(())
        } else {
            Err(self)
        }
    }
    fn split(self, amount: T::Balance) -> (Self, Self) {
        let first = self.0.min(amount);
        let second = self.0 - first;

        mem::forget(self);
        (Self::new(first), Self::new(second))
    }
    fn merge(mut self, other: Self) -> Self {
        self.0 = self.0.saturating_add(other.0);
        mem::forget(other);

        self
    }
    fn subsume(&mut self, other: Self) {
        self.0 = self.0.saturating_add(other.0);
        mem::forget(other);
    }
    fn offset(self, other: Self::Opposite) -> result::Result<Self, Self::Opposite> {
        let (a, b) = (self.0, other.0);
        mem::forget((self, other));

        if a >= b {
            Ok(Self::new(a - b))
        } else {
            Err(PositiveImbalance::new(b - a))
        }
    }
    fn peek(&self) -> T::Balance {
        self.0.clone()
    }
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> Drop for PositiveImbalance<T, GetCurrencyId> {
    /// Basic drop handler will just square up the total issuance.
    fn drop(&mut self) {
        <TotalIssuances<T>>::mutate(GetCurrencyId::get(), |v| *v = v.saturating_add(self.0));
    }
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> Drop for NegativeImbalance<T, GetCurrencyId> {
    /// Basic drop handler will just square up the total issuance.
    fn drop(&mut self) {
        <TotalIssuances<T>>::mutate(GetCurrencyId::get(), |v| *v = v.saturating_sub(self.0));
    }
}
