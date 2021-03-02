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
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{traits::Saturating, RuntimeDebug};

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct VotingParameters<BlockNumber, CurrencyId> {
    pub commit_duration: BlockNumber,
    pub reveal_duration: BlockNumber,

    pub voting_currency: CurrencyId,
    pub min_quorum: u32,
    pub min_participation: u32,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ProposalState<Balance, BlockNumber, CurrencyId> {
    pub parameters: VotingParameters<BlockNumber, CurrencyId>,
    pub revealed_against: Balance,
    pub revealed_favorable: Balance,

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
    Commit(Hash),
    Reveal(Balance, bool, u64), // Coins locked, support, salt
}
impl<Balance: Default, Hash: Default> Default for VoteData<Balance, Hash> {
    fn default() -> Self {
        Self::Commit(Hash::default())
    }
}
