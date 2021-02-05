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

//! This pallet can be used to deploy a coin voting system. It supports direct,
//! simple coin voting as well as the possibility to configure it to support
//! quadratic voting.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_event, decl_module, decl_storage};
use governance_os_support::traits::{Currencies, ProposalResult, StandardizedVoting};
use sp_runtime::{DispatchError, DispatchResult};

#[cfg(test)]
mod tests;
mod types;

use crate::types::{ProposalState, VoteData, VotingParameters};

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event> + Into<<Self as frame_system::Trait>::Event>;
    /// Pallet in charge of currencies. Used so that we can lock tokens etc...
    type Currencies: Currencies<Self::AccountId>;
}

type BalanceOf<T> =
    <<T as Trait>::Currencies as Currencies<<T as frame_system::Trait>::AccountId>>::Balance;

decl_storage! {
    trait Store for Module<T: Trait> as CoinVoting {
        pub ProposalsState get(fn proposals_state): map hasher(blake2_128_concat) T::Hash => ProposalState<BalanceOf<T>, VotingParameters>;
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

impl<T: Trait> StandardizedVoting for Module<T> {
    type ProposalID = T::Hash;
    type Parameters = VotingParameters;
    type VoteData = VoteData;

    fn initiate(proposal: Self::ProposalID, parameters: Self::Parameters) -> DispatchResult {
        todo!()
    }

    fn veto(proposal: Self::ProposalID) -> DispatchResult {
        todo!()
    }

    fn vote(proposal: Self::ProposalID, data: Self::VoteData) -> DispatchResult {
        todo!()
    }

    fn close(proposal: Self::ProposalID) -> Result<ProposalResult, DispatchError> {
        todo!()
    }
}
