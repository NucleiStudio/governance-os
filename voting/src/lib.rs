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

//! This crate is in charge of containing voting systems as supported by the Governance OS.
//! They were made to integrate nicely with our own `organizations` pallet but should be reusable
//! by other projects as well.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use governance_os_support::{voting::VotingHooks, Currencies, ReservableCurrencies};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{
    traits::{Saturating, Zero},
    DispatchError, DispatchResult, Perbill, RuntimeDebug,
};
use sp_std::{cmp::PartialOrd, collections::btree_map::BTreeMap, marker};

#[cfg(test)]
mod tests;

pub enum VotingErrors {
    NotAVotingSystem,
    UnderCreationFee,
}

impl Into<DispatchError> for VotingErrors {
    fn into(self) -> DispatchError {
        match self {
            VotingErrors::NotAVotingSystem => DispatchError::Other("not a voting system"),
            VotingErrors::UnderCreationFee => {
                DispatchError::Other("new voting weight is under the creation fee")
            }
        }
    }
}

/// Metadata used and managed by all our voting systems. Need to be passed to their functions and
/// persisted in storage.
#[derive(Eq, PartialEq, RuntimeDebug, Encode, Decode, Clone, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ProposalMetadata<AccountId, Balance, BlockNumber>
where
    AccountId: Ord,
{
    /// A map of voter => (coins, in favor or against)
    pub votes: BTreeMap<AccountId, (Balance, bool)>,
    /// Cached value of how many coins support the proposal
    pub favorable: Balance,
    /// Cached value of how many coins oppose the proposal
    pub against: Balance,
    /// Account who create the proposal, useful to avoid an attack
    /// where it would be possible to create a proposal paying the
    /// fee but then creating a 0 vote to get it back.
    pub creator: AccountId,
    /// When the proposal should be considered expired.
    pub expiry: BlockNumber,
}

/// This enum is in charge of representing and handling all existing voting systems.
#[derive(Eq, PartialEq, RuntimeDebug, Encode, Decode, Copy, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum VotingSystems<Balance, CurrencyId, BlockNumber, Currencies, AccountId> {
    /// An empty voting system. Useful if someone wants to create an organization
    /// managed by other things than votes.
    None,

    /// A simple coin based voting system where people vote by reserving some of their own coins.
    /// Our implementation has the following specificities:
    /// - creating a proposal requires the creator to block some of their coins to avoid spam; the coins
    ///   are considered in support of the proposal.
    /// - in the event that someone makes multiple votes we consider future votes as updates of the previous
    ///   ones; this means that weight and locked coins are recomputed every time.
    /// - a proposal passes once a configurable quorum is met (for instance, 50% of the coins need to agree)
    /// - a minimum quorum is in place which represents the minimum amount of coins a proposal need to be even
    ///   considered
    /// - a voting period duration is configured; after it is elapsed proposals can be closed by anyone
    CoinBased(CoinBasedVotingParameters<Balance, CurrencyId, BlockNumber>),

    /// We need this entry for accepting a `Currencies` and a `AccountId` type.
    _Phantom(marker::PhantomData<(AccountId, Currencies)>),
}

impl<Balance, CurrencyId, BlockNumber, Currencies, AccountId> Default
    for VotingSystems<Balance, CurrencyId, BlockNumber, Currencies, AccountId>
{
    fn default() -> Self {
        Self::None
    }
}

/// Parameters for the coin based voting system.
#[derive(Eq, PartialEq, RuntimeDebug, Encode, Decode, Copy, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CoinBasedVotingParameters<Balance, CurrencyId, BlockNumber> {
    /// What currency is used to represent vote inside the organization.
    pub voting_currency: CurrencyId,
    /// How much one has to lock to create a proposal and fight spam.
    pub creation_fee: Balance,
    /// What percentage of coins need to be in favor of a proposal for it to pass.
    /// We will then use it via a `perbill::from_percent` call.
    pub min_quorum: u32,
    /// What percentage of coins need to participate before a quorum is reached.
    /// We will then use it via a `perbill::from_percent` call.
    pub min_participation: u32,
    /// After how many blocks will a proposal be considered failed if it doesn't reach
    /// a favorable outcome.
    pub ttl: BlockNumber,
}

impl<AccountId, BlockNumber, C> VotingHooks
    for VotingSystems<C::Balance, C::CurrencyId, BlockNumber, C, AccountId>
where
    AccountId: Ord + Default + Clone,
    C: ReservableCurrencies<AccountId> + Currencies<AccountId>,
    C::Balance: Default,
    BlockNumber: Default + Saturating + PartialOrd,
{
    type AccountId = AccountId;
    type BlockNumber = BlockNumber;
    type Currencies = C;
    type Data = ProposalMetadata<AccountId, C::Balance, Self::BlockNumber>;
    type OrganizationId = AccountId;
    type VotingSystem = Self;

    fn on_create_proposal(
        voting_system: Self::VotingSystem,
        creator: &Self::AccountId,
        current_block: Self::BlockNumber,
    ) -> (DispatchResult, Self::Data) {
        match voting_system {
            VotingSystems::CoinBased(parameters) => {
                let reserve_is_successful = Self::Currencies::reserve(
                    parameters.voting_currency,
                    creator,
                    parameters.creation_fee,
                );
                let mut metadata = ProposalMetadata::<AccountId, C::Balance, Self::BlockNumber> {
                    ..Default::default()
                };

                if reserve_is_successful.is_ok() {
                    metadata.creator = (*creator).clone();
                    metadata.favorable = metadata.favorable.saturating_add(parameters.creation_fee);
                    metadata.expiry = current_block.saturating_add(parameters.ttl);
                    metadata
                        .votes
                        .insert((*creator).clone(), (parameters.creation_fee, true));
                }

                (reserve_is_successful, metadata)
            }
            _ => (
                Err(VotingErrors::NotAVotingSystem.into()),
                Default::default(),
            ),
        }
    }

    fn on_veto_proposal(voting_system: Self::VotingSystem, data: Self::Data) -> DispatchResult {
        // For now, vetoing is simply an early close which releases the funds. There are no punishments
        // or anything else in place.
        Self::on_close_proposal(voting_system, data, false)
    }

    fn on_decide_on_proposal(
        voting_system: Self::VotingSystem,
        mut data: Self::Data,
        voter: &Self::AccountId,
        new_weight: <Self::Currencies as Currencies<Self::AccountId>>::Balance,
        new_support: bool,
    ) -> (DispatchResult, Self::Data) {
        match voting_system {
            VotingSystems::CoinBased(parameters) => {
                // Prevent potential attack where a proposal creator manages to get his
                // fee reimbursed by creating a vote with 0 weight.
                if voter == &data.creator && new_weight < parameters.creation_fee {
                    return (Err(VotingErrors::UnderCreationFee.into()), data);
                }

                let default_state = (Zero::zero(), true);
                let old_state = data.votes.get(voter).unwrap_or(&default_state);
                let old_weight = old_state.0;
                let old_support = old_state.1;

                if old_weight > new_weight {
                    Self::Currencies::unreserve(
                        parameters.voting_currency,
                        voter,
                        old_weight.saturating_sub(new_weight),
                    );
                } else if new_weight > old_weight {
                    let res = Self::Currencies::reserve(
                        parameters.voting_currency,
                        voter,
                        new_weight.saturating_sub(old_weight),
                    );
                    if res.is_err() {
                        return (res, data);
                    }
                }

                match (old_support, new_support) {
                    (true, false) => {
                        // Switching from favorable to against
                        data.favorable = data.favorable.saturating_sub(old_weight);
                        data.against = data.against.saturating_add(new_weight);
                    }
                    (false, true) => {
                        // Switching from against to favorable
                        data.against = data.favorable.saturating_sub(old_weight);
                        data.favorable = data.against.saturating_add(new_weight);
                    }
                    (true, true) => {
                        // Still favorable but weight change
                        data.favorable = data
                            .favorable
                            .saturating_sub(old_weight)
                            .saturating_add(new_weight);
                    }
                    (false, false) => {
                        // Still against but weight change
                        data.against = data
                            .against
                            .saturating_sub(old_weight)
                            .saturating_add(new_weight);
                    }
                };

                data.votes
                    .insert((*voter).clone(), (new_weight, new_support));

                (Ok(()), data)
            }
            _ => (Err(VotingErrors::NotAVotingSystem.into()), data),
        }
    }

    fn can_close(
        voting_system: Self::VotingSystem,
        data: Self::Data,
        current_block: Self::BlockNumber,
    ) -> bool {
        data.expiry < current_block || Self::passing(voting_system, data)
    }

    fn passing(voting_system: Self::VotingSystem, data: Self::Data) -> bool {
        match voting_system {
            VotingSystems::CoinBased(parameters) => {
                let total_votes = data.favorable.saturating_add(data.against);
                let min_votes_for_quorum =
                    Perbill::from_percent(parameters.min_quorum) * total_votes;
                let min_participating_tokens = Perbill::from_percent(parameters.min_participation)
                    * Self::Currencies::total_issuance(parameters.voting_currency);

                data.favorable > min_votes_for_quorum && total_votes > min_participating_tokens
            }
            _ => false,
        }
    }

    fn on_close_proposal(
        voting_system: Self::VotingSystem,
        data: Self::Data,
        _executed: bool, // Could be used for rewards / punishments later on
    ) -> DispatchResult {
        match voting_system {
            VotingSystems::CoinBased(parameters) => {
                data.votes.iter().for_each(|(account, (val, _support))| {
                    Self::Currencies::unreserve(parameters.voting_currency, account, *val);
                });

                Ok(())
            }
            _ => Err(VotingErrors::NotAVotingSystem.into()),
        }
    }
}
