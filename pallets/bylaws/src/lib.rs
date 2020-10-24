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

//! This pallet implements a bylaws system for account permissioning.
//! A bylaw is typically made of a "tag" and a "rule". Tags represents
//! the kinds of calls being filtered, for instance an hypothetic tag
//! `Monetary` could refer to all extrinsics moving tokens. A rule is
//! then a simple script implemented in a DSL (typically Rust enums)
//! to help the system decide if the call should be approved or discarded.
//!
//! The bylaws pallet should typically be added to the `SignedExtra` of a
//! runtime so that it could filter incoming calls.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_error, decl_event, decl_module, decl_storage};

pub mod bylaw;
#[cfg(test)]
mod tests;

pub use bylaw::Bylaw;

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Bylaws {
        pub Sample get(fn sample): bool;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        Sample(AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        SampleError,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;
    }
}
