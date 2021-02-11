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

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure, traits::LockIdentifier,
};
use governance_os_support::traits::{
    Currencies, LockableCurrencies, ProposalResult, StandardizedVoting,
};
use sp_runtime::{
    traits::{Saturating, Zero},
    DispatchError, DispatchResult,
};
use sp_std::vec::Vec;

#[cfg(test)]
mod tests;
mod types;

use crate::types::{ProposalState, VoteData, VotingParameters};

pub const COIN_VOTING_LOCK_ID: LockIdentifier = *b"coinvote";

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
type LockDataOf<T> = (<T as frame_system::Trait>::Hash, bool, BalanceOf<T>);
type LockIdentifierOf<T> = (CurrencyIdOf<T>, <T as frame_system::Trait>::AccountId);

decl_storage! {
    trait Store for Module<T: Trait> as CoinVoting {
        pub Proposals get(fn proposals): map hasher(blake2_128_concat) T::Hash => ProposalState<BalanceOf<T>, VotingParameters<CurrencyIdOf<T>>, LockIdentifierOf<T>>;
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
    type Parameters = VotingParameters<CurrencyIdOf<T>>;
    type VoteData = VoteData<BalanceOf<T>>;
    type AccountId = T::AccountId;

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
                initialized: true,
                locks: vec![],
            });

            Ok(())
        })?;

        Ok(())
    }

    fn veto(proposal: Self::ProposalID) -> DispatchResult {
        // not the use of take instead of get which also deletes the storage
        Proposals::<T>::take(proposal).locks.iter().try_for_each(
            |lock_identifier| -> DispatchResult {
                let lock_data = Locks::<T>::get(lock_identifier);

                let new_lock_data: Vec<LockDataOf<T>> = lock_data
                    .iter()
                    .cloned()
                    .filter(|(prop, _, _)| prop != &proposal)
                    .collect();

                if !new_lock_data.is_empty() {
                    Locks::<T>::insert(lock_identifier, new_lock_data.clone());
                } else {
                    Locks::<T>::remove(lock_identifier);
                }

                let max_to_lock: BalanceOf<T> = new_lock_data.iter().cloned().fold(
                    Zero::zero(),
                    |acc, (_proposal, _support, power)| {
                        if power > acc {
                            power
                        } else {
                            acc
                        }
                    },
                );

                if max_to_lock == Zero::zero() {
                    T::Currencies::remove_lock(
                        lock_identifier.0,
                        COIN_VOTING_LOCK_ID,
                        &lock_identifier.1,
                    )?;
                } else {
                    T::Currencies::set_lock(
                        lock_identifier.0,
                        COIN_VOTING_LOCK_ID,
                        &lock_identifier.1,
                        max_to_lock,
                    )?;
                }

                Ok(())
            },
        )?;

        Proposals::<T>::remove(proposal);

        Ok(())
    }

    fn vote(
        proposal: Self::ProposalID,
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
            |_proposal, old_support, old_power| {
                // We found a duplicated vote, thus we need to remove it from our precomputed
                // state to avoid mistakes
                if old_support {
                    state.total_favorable = state.total_favorable.saturating_sub(old_power);
                } else {
                    state.total_against = state.total_against.saturating_sub(old_power);
                }

                this_vote_is_a_duplicate = true;
            },
        )?;

        if !this_vote_is_a_duplicate {
            state
                .locks
                .push((state.parameters.voting_currency, voter.clone()));
        }

        if data.in_support {
            state.total_favorable = state.total_favorable.saturating_add(data.power);
        } else {
            state.total_against = state.total_against.saturating_add(data.power);
        }

        Proposals::<T>::insert(proposal, state);

        Ok(())
    }

    fn close(proposal: Self::ProposalID) -> Result<ProposalResult, DispatchError> {
        todo!()
    }
}

impl<T: Trait> Module<T> {
    fn update_locks<F>(
        voting_currency: CurrencyIdOf<T>,
        voter: &T::AccountId,
        proposal: T::Hash,
        support: bool,
        power: BalanceOf<T>,
        mut on_duplicate_vote_found: F,
    ) -> DispatchResult
    where
        F: FnMut(T::Hash, bool, BalanceOf<T>),
    {
        Locks::<T>::try_mutate((voting_currency, voter), |locks| -> DispatchResult {
            // because we use iterators we have to first create a vec for
            // a use with chain() later on
            let locks_addition = vec![(proposal, support, power)];

            // Filter and remove any duplicate votes
            *locks = locks
                .iter()
                .cloned()
                .filter(|&(maybe_duplicate_proposal, support, power)| {
                    if maybe_duplicate_proposal != proposal {
                        true
                    } else {
                        // callback
                        on_duplicate_vote_found(maybe_duplicate_proposal, support, power);
                        //continue
                        false
                    }
                })
                .chain(locks_addition.iter().cloned())
                .collect();

            let max_to_lock: BalanceOf<T> = locks.iter().fold(
                Zero::zero(),
                |acc, &(_proposal, _support, power)| {
                    if power > acc {
                        power
                    } else {
                        acc
                    }
                },
            );

            T::Currencies::set_lock(voting_currency, COIN_VOTING_LOCK_ID, voter, max_to_lock)?;
            Ok(())
        })?;

        Ok(())
    }
}
