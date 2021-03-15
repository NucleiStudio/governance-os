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

//! This pallet can be used to deploy a conviction voting system. With
//! conviction voting, votes accumulate and de-accumulate over time via
//! a half life decay curve.
//! It incentivizes people to vote earlier as their votes will have more
//! value over time (up to a certain limit). In our implementation, we
//! have decided that every new vote's conviction will start at 0 and
//! slowly accumulate to whatever the user staked on the proposal. This
//! means that even if they stake a lot of coins they won't be accounted
//! immediately.

#![cfg_attr(not(feature = "std"), no_std)]

use crate::types::ProposalState;
use frame_support::{decl_error, decl_module, decl_storage, ensure, traits::LockIdentifier};
use governance_os_support::traits::{
    Currencies, LockableCurrencies, ProposalResult, StandardizedVoting,
};
use sp_runtime::{traits::Zero, DispatchError, DispatchResult};
use sp_std::prelude::*;

#[cfg(test)]
mod tests;
mod types;

pub use types::{Conviction, VotingParameters};

pub const CONVICTION_VOTING_LOCK_ID: LockIdentifier = *b"convvote";

pub trait Trait: frame_system::Trait {
    /// Pallet in charge of currencies. Used so that we can lock tokens etc...
    type Currencies: LockableCurrencies<Self::AccountId>;
}

type BalanceOf<T> =
    <<T as Trait>::Currencies as Currencies<<T as frame_system::Trait>::AccountId>>::Balance;
type CurrencyIdOf<T> =
    <<T as Trait>::Currencies as Currencies<<T as frame_system::Trait>::AccountId>>::CurrencyId;
type ConvictionProposalStateOf<T> = ProposalState<
    <T as frame_system::Trait>::AccountId,
    BalanceOf<T>,
    <T as frame_system::Trait>::BlockNumber,
    CurrencyIdOf<T>,
>;

decl_storage! {
    trait Store for Module<T: Trait> as PlcrVoting {
        pub Proposals get(fn proposals): map hasher(blake2_128_concat) T::Hash => ConvictionProposalStateOf<T>;
        pub Locks get(fn locks): map hasher(blake2_128_concat) (CurrencyIdOf<T>, T::AccountId) => Vec<(T::Hash, bool, BalanceOf<T>)>;
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// There are not enough tokens in the user's balance to proceed
        /// to this action.
        NotEnoughBalance,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    }
}

impl<T: Trait> StandardizedVoting for Module<T> {
    type ProposalId = T::Hash;
    type Parameters = VotingParameters<T::BlockNumber, CurrencyIdOf<T>>;
    type VoteData = Conviction<BalanceOf<T>>;
    type AccountId = T::AccountId;

    fn initiate(proposal: Self::ProposalId, parameters: Self::Parameters) -> DispatchResult {
        Proposals::<T>::try_mutate_exists(proposal, |maybe_existing_state| -> DispatchResult {
            // no duplicates, we can create a new state
            *maybe_existing_state = Some(ProposalState {
                parameters,
                created_on: Self::now(),
                convictions: Vec::new(),
            });

            Ok(())
        })?;

        Ok(())
    }

    fn veto(proposal: Self::ProposalId) -> DispatchResult {
        // note the take() function instead of the usual get()
        Self::finalize(proposal, Proposals::<T>::take(proposal))
    }

    fn vote(
        proposal: Self::ProposalId,
        voter: &Self::AccountId,
        data: Self::VoteData,
    ) -> DispatchResult {
        let mut state = Proposals::<T>::get(proposal);

        ensure!(
            data.power < T::Currencies::free_balance(state.parameters.voting_currency, voter),
            Error::<T>::NotEnoughBalance
        );

        let mut locks = Locks::<T>::get((state.parameters.voting_currency, voter));
        locks = locks
            .iter()
            .cloned()
            .filter(|(proposal_hash, _support, _power)| *proposal_hash != proposal)
            .collect::<Vec<_>>();
        locks.push((proposal, data.in_support, data.power));

        Self::rejig_locks(state.parameters.voting_currency, voter, locks)?;

        state.convictions = state
            .convictions
            .iter()
            .cloned()
            .filter(|(participant, _)| participant != voter)
            .collect::<Vec<_>>();
        state.convictions.push((voter.clone(), data));
        Proposals::<T>::insert(proposal, state);

        Ok(())
    }

    fn close(proposal: Self::ProposalId) -> Result<ProposalResult, DispatchError> {
        let state = Proposals::<T>::get(proposal);
        Self::finalize(proposal, state)?;
        Proposals::<T>::remove(proposal);

        Ok(ProposalResult::Failing)
    }
}

impl<T: Trait> Module<T> {
    /// Simple helper function to return the current block number.
    fn now() -> T::BlockNumber {
        frame_system::Module::<T>::block_number()
    }

    /// Iterates the `locks` vector and lock the maximum amount of coins needed for
    /// the pair `voting_currency` and `voter`.
    fn rejig_locks(
        voting_currency: CurrencyIdOf<T>,
        voter: &T::AccountId,
        locks: Vec<(T::Hash, bool, BalanceOf<T>)>,
    ) -> DispatchResult {
        let max = locks.iter().cloned().fold(
            Zero::zero(),
            |acc, (_proposal, _support, power)| {
                if acc > power {
                    acc
                } else {
                    power
                }
            },
        );
        if max == Zero::zero() {
            T::Currencies::remove_lock(voting_currency, CONVICTION_VOTING_LOCK_ID, voter)?;
        } else {
            T::Currencies::set_lock(voting_currency, CONVICTION_VOTING_LOCK_ID, voter, max)?;
        }
        if locks.is_empty() {
            Locks::<T>::remove((voting_currency, voter));
        } else {
            Locks::<T>::insert((voting_currency, voter), locks);
        }

        Ok(())
    }

    /// Goes through all the elements related to the passed proposal state and cleans
    /// up any associated storage elements such as locks.
    fn finalize(proposal: T::Hash, state: ConvictionProposalStateOf<T>) -> DispatchResult {
        state
            .convictions
            .iter()
            .try_for_each(|(voter, _conviction)| -> DispatchResult {
                let mut locks = Locks::<T>::get((state.parameters.voting_currency, voter));
                locks = locks
                    .iter()
                    .cloned()
                    .filter(|(proposal_hash, _support, _power)| *proposal_hash != proposal)
                    .collect::<Vec<_>>();
                Self::rejig_locks(state.parameters.voting_currency, voter, locks)?;

                Ok(())
            })?;

        Ok(())
    }
}
