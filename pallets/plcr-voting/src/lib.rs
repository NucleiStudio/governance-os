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

//! This pallet can be used to deploy a PLCR voting system. With PLCR
//! voting, votes are hashed before being revealed and counted. This
//! allows people to hide their votes until everyone places theirs as
//! to prevent collusion between participants or typical human biases.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_error, decl_event, decl_module, decl_storage, traits::LockIdentifier};
use governance_os_support::traits::{
    Currencies, LockableCurrencies, ProposalResult, StandardizedVoting,
};

#[cfg(test)]
mod tests;

pub const PLCR_VOTING_LOCK_ID: LockIdentifier = *b"plcrvote";

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event> + Into<<Self as frame_system::Trait>::Event>;
    /// Pallet in charge of currencies. Used so that we can lock tokens etc...
    type Currencies: LockableCurrencies<Self::AccountId>;
}

type BalanceOf<T> =
    <<T as Trait>::Currencies as Currencies<<T as frame_system::Trait>::AccountId>>::Balance;
type CurrencyIdOf<T> =
    <<T as Trait>::Currencies as Currencies<<T as frame_system::Trait>::AccountId>>::CurrencyId;

decl_storage! {
    trait Store for Module<T: Trait> as PlcrVoting {
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
    }
}

decl_event!(
    pub enum Event {}
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;
    }
}
