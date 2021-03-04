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

//! Type definitions for the coin based voting pallet.

use codec::{Decode, Encode};
use governance_os_support::impl_enum_default;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{
    traits::{IntegerSquareRoot, Saturating},
    RuntimeDebug,
};
use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct VotingParameters<BlockNumber, CurrencyId> {
    /// How old a proposal can get before it can be closed and considered
    /// as failing.
    pub ttl: BlockNumber,
    /// Currency used to represent vote "powers". The more tokens one has
    /// the more power to tip the balance they have.
    pub voting_currency: CurrencyId,
    /// Minimum quorum that needs to be met in order for a proposal to be
    /// considered passing. Should be a percentage value as it is passed to
    /// `Perbill::from_percent` later on. Quorum is computed from all the
    /// votes casted. If quorum is set to 50% it means a proposal will pass
    /// if 50% + 1 of the votes are favorable to it.
    pub min_quorum: u32,
    /// Minimum participation that needs to be met in order for a proposal
    /// to be considered passing. Should be a percentage value as it is
    /// passed to `Perbill::from_percent` later on. If the minimum participation
    /// is set to 33% it means that at least 33% + 1 of the `voting_currency`
    /// total supply need to have been used in votes for or against the proposal.
    pub min_participation: u32,

    /// Defines how we are going to count and register votes.
    pub vote_counting_strategy: VoteCountingStrategy,
}

/// Various strategies we support to count votes.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Copy)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum VoteCountingStrategy {
    /// 1 coin is equal to 1 vote.
    Simple,
    /// The price to acquire votes increases quadratically.
    Quadratic,
}
impl_enum_default!(VoteCountingStrategy, Simple);

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct VoteData<Balance> {
    /// Wether we are favorable to the proposal or opposed to it.
    pub in_support: bool,
    /// How much coins we want to stake on this decision.
    pub power: Balance,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ProposalState<Balance, BlockNumber, CurrencyId, LockIdentifier> {
    /// Parameters that this proposal was created with.
    pub parameters: VotingParameters<BlockNumber, CurrencyId>,
    /// Total amount of votes staked in favor of this proposal.
    pub total_favorable: Balance,
    /// Total amount of votes staked against this proposal.
    pub total_against: Balance,

    /// Used to list all opened locks on coins to later free those.
    pub locks: Vec<LockIdentifier>,

    /// Record when the proposal was created. Used to know when it is expired
    /// (when the ttl is over).
    pub created_on: BlockNumber,
}

impl<Balance: Saturating + Copy + IntegerSquareRoot, BlockNumber, CurrencyId, LockIdentifier>
    ProposalState<Balance, BlockNumber, CurrencyId, LockIdentifier>
{
    /// Record a new vote based on wether it is favorable or not.
    pub fn record_vote(&mut self, favorable: bool, power: Balance) {
        if favorable {
            self.total_favorable = self.total_favorable.saturating_add(self.real_power(power));
        } else {
            self.total_against = self.total_against.saturating_add(self.real_power(power));
        }
    }

    /// Unrecord an existing vote based on wether it was favorable or not.
    pub fn unrecord_vote(&mut self, favorable: bool, power: Balance) {
        if favorable {
            self.total_favorable = self.total_favorable.saturating_sub(self.real_power(power));
        } else {
            self.total_against = self.total_against.saturating_sub(self.real_power(power));
        }
    }

    /// Compute the actual voting power someone has based on the selected vote
    /// vote counting strategy.
    fn real_power(&self, power: Balance) -> Balance {
        match self.parameters.vote_counting_strategy {
            VoteCountingStrategy::Simple => power,
            VoteCountingStrategy::Quadratic => {
                // cost to the voter = number of votes ** 2, so
                // number of votes = sqrt(cost to the voter)

                power
                    .integer_sqrt_checked()
                    .expect("we are supposed to use uints and thus the value cannot be negative")
            }
        }
    }
}
