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

use frame_support::{decl_error, decl_module, decl_storage, ensure, traits::LockIdentifier};
use governance_os_support::traits::{
    Currencies, LockableCurrencies, ProposalResult, StandardizedVoting,
};
use sp_runtime::{
    traits::{Saturating, Zero},
    DispatchError, DispatchResult, Perbill,
};
use sp_std::{prelude::*, vec::Vec};
use types::ProposalState;

#[cfg(test)]
mod tests;
mod types;

pub use types::{VoteCountingStrategy, VoteData, VotingParameters};

pub const COIN_VOTING_LOCK_ID: LockIdentifier = *b"coinvote";

pub trait Trait: frame_system::Trait {
    /// Pallet in charge of currencies. Used so that we can lock tokens etc...
    type Currencies: LockableCurrencies<Self::AccountId>;
}

type BalanceOf<T> =
    <<T as Trait>::Currencies as Currencies<<T as frame_system::Trait>::AccountId>>::Balance;
type CurrencyIdOf<T> =
    <<T as Trait>::Currencies as Currencies<<T as frame_system::Trait>::AccountId>>::CurrencyId;
type LockDataOf<T> = (
    <T as frame_system::Trait>::Hash,
    bool,
    BalanceOf<T>,
    VoteCountingStrategy,
);
type LockIdentifierOf<T> = (CurrencyIdOf<T>, <T as frame_system::Trait>::AccountId);
type CoinProposalStateOf<T> = ProposalState<
    BalanceOf<T>,
    <T as frame_system::Trait>::BlockNumber,
    CurrencyIdOf<T>,
    LockIdentifierOf<T>,
>;

decl_storage! {
    trait Store for Module<T: Trait> as CoinVoting {
        pub Proposals get(fn proposals): map hasher(blake2_128_concat) T::Hash => CoinProposalStateOf<T>;
        pub Locks get(fn locks): map hasher(blake2_128_concat) LockIdentifierOf<T> => Vec<LockDataOf<T>>;
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// This proposal ID is already pending a vote, thus it can not
        /// be created again for now.
        DuplicatedProposal,
        /// There are not enough tokens in the user's balance to proceed
        /// to this action.
        NotEnoughBalance,
        /// This proposal was not initialized with this pallet or simply
        /// does not exists.
        ProposalNotInitialized,
        /// Proposal cannot be closed yet, it is likely too early.
        CannotClose,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    }
}

impl<T: Trait> StandardizedVoting for Module<T> {
    type ProposalId = T::Hash;
    type Parameters = VotingParameters<T::BlockNumber, CurrencyIdOf<T>>;
    type VoteData = VoteData<BalanceOf<T>>;
    type AccountId = T::AccountId;

    fn initiate(proposal: Self::ProposalId, parameters: Self::Parameters) -> DispatchResult {
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
                initialized: true,
                locks: vec![],
                created_on: Self::now(),
            });

            Ok(())
        })?;

        Ok(())
    }

    fn vote(
        proposal: Self::ProposalId,
        voter: &Self::AccountId,
        data: Self::VoteData,
    ) -> DispatchResult {
        let mut state = Self::proposals(proposal);
        ensure!(state.initialized, Error::<T>::ProposalNotInitialized);

        // We want to prevent votes for user with less coins than they'd like to lock.
        ensure!(
            T::Currencies::free_balance(state.parameters.voting_currency, voter) >= data.power,
            Error::<T>::NotEnoughBalance
        );
        let mut this_vote_is_a_duplicate = false;
        Self::update_locks(
            state.parameters.voting_currency,
            voter,
            proposal,
            data.in_support,
            data.power,
            state.parameters.vote_counting_strategy,
            |_proposal, old_support, old_power, _strategy| {
                // We found a duplicated vote, thus we need to remove it from our precomputed
                // state to avoid mistakes
                state.unrecord_vote(old_support, old_power);
                this_vote_is_a_duplicate = true;
            },
        )?;

        if !this_vote_is_a_duplicate {
            state
                .locks
                .push((state.parameters.voting_currency, voter.clone()));
        }

        state.record_vote(data.in_support, data.power);

        Proposals::<T>::insert(proposal, state);

        Ok(())
    }

    fn veto(proposal: Self::ProposalId) -> DispatchResult {
        // note the use of take instead of get which also deletes the storage
        Self::unlock(Proposals::<T>::take(proposal).locks, proposal)
    }

    fn close(proposal: Self::ProposalId) -> Result<ProposalResult, DispatchError> {
        let state = Proposals::<T>::get(proposal);

        let total_supply = T::Currencies::total_issuance(state.parameters.voting_currency);
        let total_participation = state.total_against + state.total_favorable;

        let enough_participation = Perbill::from_percent(state.parameters.min_participation)
            * total_supply
            < total_participation;
        let enough_quorum = Perbill::from_percent(state.parameters.min_quorum)
            * total_participation
            < state.total_favorable;

        let result = if enough_participation && enough_quorum {
            ProposalResult::Passing
        } else {
            ProposalResult::Failing
        };

        let can_close = state.created_on.saturating_add(state.parameters.ttl) < Self::now();
        ensure!(
            can_close || result == ProposalResult::Passing,
            Error::<T>::CannotClose
        );

        Self::unlock(state.locks, proposal)?;

        Proposals::<T>::remove(proposal);
        Ok(result)
    }
}

impl<T: Trait> Module<T> {
    fn update_locks<F>(
        voting_currency: CurrencyIdOf<T>,
        voter: &T::AccountId,
        proposal: T::Hash,
        support: bool,
        power: BalanceOf<T>,
        strategy: VoteCountingStrategy,
        mut on_duplicate_vote_found: F,
    ) -> DispatchResult
    where
        F: FnMut(T::Hash, bool, BalanceOf<T>, VoteCountingStrategy),
    {
        Locks::<T>::try_mutate((voting_currency, voter), |locks| -> DispatchResult {
            // because we use iterators we have to first create a vec for
            // a use with chain() later on
            let locks_addition = vec![(proposal, support, power, strategy)];

            // Filter and remove any duplicate votes
            *locks = locks
                .iter()
                .cloned()
                .filter(|&(maybe_duplicate_proposal, support, power, strategy)| {
                    if maybe_duplicate_proposal != proposal {
                        true
                    } else {
                        // callback
                        on_duplicate_vote_found(maybe_duplicate_proposal, support, power, strategy);
                        //continue
                        false
                    }
                })
                .chain(locks_addition.iter().cloned())
                .collect();

            Self::rejig_locks(locks.to_vec(), voting_currency, voter)?;
            Ok(())
        })?;

        Ok(())
    }

    fn unlock(locks: Vec<LockIdentifierOf<T>>, proposal: T::Hash) -> DispatchResult {
        locks
            .iter()
            .try_for_each(|lock_identifier| -> DispatchResult {
                let lock_data = Locks::<T>::get(lock_identifier);

                let new_lock_data: Vec<LockDataOf<T>> = lock_data
                    .iter()
                    .cloned()
                    .filter(|(prop, _, _, _)| prop != &proposal)
                    .collect();

                if !new_lock_data.is_empty() {
                    Locks::<T>::insert(lock_identifier, new_lock_data.clone());
                } else {
                    Locks::<T>::remove(lock_identifier);
                }

                Self::rejig_locks(new_lock_data, lock_identifier.0, &lock_identifier.1)?;
                Ok(())
            })?;

        Ok(())
    }

    fn rejig_locks(
        locks: Vec<LockDataOf<T>>,
        voting_currency: CurrencyIdOf<T>,
        who: &T::AccountId,
    ) -> DispatchResult {
        let max_to_lock_with_simple_voting: BalanceOf<T> = locks
            .iter()
            .cloned()
            .filter(|(_proposal, _support, _power, strategy)| {
                strategy == &VoteCountingStrategy::Simple
            })
            .fold(
                Zero::zero(),
                |acc, (_proposal, _support, power, _strategy)| {
                    if power > acc {
                        power
                    } else {
                        acc
                    }
                },
            );

        let max_to_lock_with_quadratic_voting: BalanceOf<T> = locks
            .iter()
            .cloned()
            .filter(|(_proposal, _support, _power, strategy)| {
                strategy == &VoteCountingStrategy::Quadratic
            })
            .fold(
                Zero::zero(),
                |acc, (_proposal, _support, power, _strategy)| acc.saturating_add(power),
            );

        let max_to_lock =
            max_to_lock_with_simple_voting.saturating_add(max_to_lock_with_quadratic_voting);

        if max_to_lock == Zero::zero() {
            T::Currencies::remove_lock(voting_currency, COIN_VOTING_LOCK_ID, who)?;
        } else {
            T::Currencies::set_lock(voting_currency, COIN_VOTING_LOCK_ID, who, max_to_lock)?;
        }

        Ok(())
    }

    fn now() -> T::BlockNumber {
        frame_system::Module::<T>::block_number()
    }
}
