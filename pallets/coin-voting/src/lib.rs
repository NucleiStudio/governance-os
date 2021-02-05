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

use frame_support::{decl_error, decl_event, decl_module, decl_storage};
use governance_os_support::traits::{Currencies, ProposalResult, StandardizedVoting};
use sp_runtime::{traits::Zero, DispatchError, DispatchResult};

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
        pub Proposals get(fn proposals): map hasher(blake2_128_concat) T::Hash => ProposalState<BalanceOf<T>, VotingParameters>;
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// This proposal ID is already pending a vote, thus it can not
        /// be created again for now.
        DuplicatedProposal,
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
        Proposals::<T>::try_mutate_exists(proposal, |maybe_existing_state| -> DispatchResult {
            if maybe_existing_state.is_some() {
                // duplicate detected, we do not want to erase any pending vote's
                // state and thus fail.
                return Err(Error::<T>::DuplicatedProposal.into());
            }

            // no duplicates, we can create a new state
            *maybe_existing_state = Some(ProposalState {
                parameters,
                total_against: Zero::zero(),
                total_favorable: Zero::zero(),
            });

            Ok(())
        })?;

        Ok(())
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
