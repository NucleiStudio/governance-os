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

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure, traits::LockIdentifier,
};
use governance_os_support::traits::{
    Currencies, LockableCurrencies, ProposalResult, StandardizedVoting,
};
use sp_runtime::{
    traits::{Hash, Zero},
    DispatchError, DispatchResult,
};
use sp_std::prelude::*;
use types::{ProposalState, VoteData, VotingParameters};

#[cfg(test)]
mod tests;
mod types;

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
type ProposalStateOf<T> =
    ProposalState<BalanceOf<T>, <T as frame_system::Trait>::BlockNumber, CurrencyIdOf<T>>;

decl_storage! {
    trait Store for Module<T: Trait> as PlcrVoting {
        pub Proposals get(fn proposals): map hasher(blake2_128_concat) T::Hash => ProposalStateOf<T>;
        pub Locks get(fn locks): map hasher(blake2_128_concat) (CurrencyIdOf<T>, T::AccountId) => Vec<(T::Hash, BalanceOf<T>)>;
        pub Votes get(fn votes): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) T::Hash => VoteData<BalanceOf<T>, T::Hash>;
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// This proposal ID is already pending a vote, thus it can not
        /// be created again for now.
        DuplicatedProposal,
        /// The reveal vote does not match our saved commit.
        RevealCommitMismatch,
        /// We were not able to find a commit for the given reveal vote.
        NoCommitFound,
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
    type Parameters = VotingParameters<T::BlockNumber, CurrencyIdOf<T>>;
    type VoteData = VoteData<BalanceOf<T>, T::Hash>;
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
                created_on: Self::now(),

                revealed_against: Zero::zero(),
                revealed_favorable: Zero::zero(),
            });

            Ok(())
        })?;

        Ok(())
    }

    fn veto(proposal: Self::ProposalID) -> DispatchResult {
        todo!()
    }

    fn vote(
        proposal: Self::ProposalID,
        voter: &Self::AccountId,
        data: Self::VoteData,
    ) -> DispatchResult {
        let mut state = Self::proposals(proposal);

        match data {
            VoteData::Commit(hash, decoy) => {
                Self::lock(proposal, state.parameters.voting_currency, voter, decoy)?;
            }
            VoteData::Reveal(balance, support, salt) => {
                if let VoteData::Commit(hash, decoy) = Self::votes(voter, proposal) {
                    let hashed_reveal = T::Hashing::hash_of(&(balance, support, salt));
                    ensure!(hashed_reveal == hash, Error::<T>::RevealCommitMismatch);

                    Self::unlock(proposal, state.parameters.voting_currency, voter, decoy)?;
                    Self::lock(proposal, state.parameters.voting_currency, voter, balance)?;

                    state.add_support(support, balance);
                    Proposals::<T>::insert(proposal, state);
                } else {
                    return Err(Error::<T>::NoCommitFound.into());
                }
            }
        };

        Votes::<T>::insert(voter, proposal, data);
        Ok(())
    }

    fn close(proposal: Self::ProposalID) -> Result<ProposalResult, DispatchError> {
        todo!()
    }
}

impl<T: Trait> Module<T> {
    fn now() -> T::BlockNumber {
        frame_system::Module::<T>::block_number()
    }

    fn lock(
        proposal: T::Hash,
        currency: CurrencyIdOf<T>,
        who: &T::AccountId,
        amount: BalanceOf<T>,
    ) -> DispatchResult {
        Locks::<T>::try_mutate((currency, who), |locks| -> DispatchResult {
            // because we use iterators we have to first create a vec for
            // a use with chain() later on
            let locks_addition = vec![(proposal, amount)];

            // Filter and remove any duplicate votes
            *locks = locks
                .iter()
                .cloned()
                .filter(|&(maybe_duplicate_proposal, _locked_amount)| {
                    maybe_duplicate_proposal != proposal
                })
                .chain(locks_addition.iter().cloned())
                .collect();

            Self::rejig_locks(locks.to_vec(), currency, who)?;
            Ok(())
        })?;

        Ok(())
    }

    fn unlock(
        proposal: T::Hash,
        currency: CurrencyIdOf<T>,
        who: &T::AccountId,
        amount: BalanceOf<T>,
    ) -> DispatchResult {
        let mut lock_data = Locks::<T>::get((currency, who));
        lock_data = lock_data
            .iter()
            .cloned()
            .filter(|&(maybe_duplicate_proposal, locked_amount)| {
                maybe_duplicate_proposal != proposal && locked_amount != amount
            })
            .collect();

        if lock_data.is_empty() {
            Locks::<T>::remove((currency, who));
        } else {
            Locks::<T>::insert((currency, who), lock_data.clone());
        }

        Self::rejig_locks(lock_data, currency, who)?;
        Ok(())
    }

    fn rejig_locks(
        locks: Vec<(T::Hash, BalanceOf<T>)>,
        currency: CurrencyIdOf<T>,
        who: &T::AccountId,
    ) -> DispatchResult {
        let max_to_lock =
            locks
                .into_iter()
                .fold(Zero::zero(), |acc, (_proposal, locked_amount)| {
                    if acc < locked_amount {
                        locked_amount
                    } else {
                        acc
                    }
                });

        if max_to_lock == Zero::zero() {
            T::Currencies::remove_lock(currency, PLCR_VOTING_LOCK_ID, who)?;
        } else {
            T::Currencies::set_lock(currency, PLCR_VOTING_LOCK_ID, who, max_to_lock)?;
        }

        Ok(())
    }
}
