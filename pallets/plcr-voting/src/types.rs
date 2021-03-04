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

//! Type definitions for the plcr voting pallet.

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{traits::Saturating, RuntimeDebug};

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct VotingParameters<BlockNumber, CurrencyId> {
    /// Defines the duration of the "commit phase". During this period
    /// users can submit "commit votes" that are hashed version of their
    /// votes.
    pub commit_duration: BlockNumber,
    /// Defines the duration of the "reveal phase". During this period
    /// users can submit "reveal votes" that should match their previous
    /// commits.
    pub reveal_duration: BlockNumber,

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
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ProposalState<Balance, BlockNumber, CurrencyId> {
    /// Parameters that this proposal was created with.
    pub parameters: VotingParameters<BlockNumber, CurrencyId>,
    /// How many tokens where staked against this proposal. Computed only
    /// from revealed votes.
    pub revealed_against: Balance,
    /// How many tokens where staked in favor of this proposal. Computed
    /// only from revealed votes.
    pub revealed_favorable: Balance,

    /// When this proposal was created. Used to compute phases ending blocks.
    pub created_on: BlockNumber,
}
impl<Balance: Saturating + Copy, BlockNumber, CurrencyId>
    ProposalState<Balance, BlockNumber, CurrencyId>
{
    pub fn add_support(&mut self, support: bool, stake: Balance) {
        if support {
            self.revealed_favorable = self.revealed_favorable.saturating_add(stake);
        } else {
            self.revealed_against = self.revealed_against.saturating_add(stake);
        }
    }
}

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum VoteData<Balance, Hash> {
    /// A commit vote is the hash of the vote that will be revealed
    /// later on.
    Commit(Hash),
    /// The revelation of vote that was previously committed. Fields
    /// are \[amount of coins locked, support, salt\].
    Reveal(Balance, bool, u64),
}
impl<Balance: Default, Hash: Default> Default for VoteData<Balance, Hash> {
    fn default() -> Self {
        Self::Commit(Hash::default())
    }
}
