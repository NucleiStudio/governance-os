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
use sp_runtime::{
    traits::{IntegerSquareRoot, Saturating},
    Perbill, RuntimeDebug,
};

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
pub struct VotingParameters<BlockNumber, CurrencyId> {
    pub ttl: BlockNumber,
    pub voting_currency: CurrencyId,
    pub min_quorum: Perbill,
    pub min_participation: Perbill,

    pub vote_counting_strategy: VoteCountingStrategy,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Copy)]
pub enum VoteCountingStrategy {
    Simple,
    Quadratic,
}
impl_enum_default!(VoteCountingStrategy, Simple);

pub struct VoteData<Balance> {
    pub in_support: bool,
    pub power: Balance,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
pub struct ProposalState<Balance, BlockNumber, CurrencyId, LockIdentifier> {
    pub parameters: VotingParameters<BlockNumber, CurrencyId>,
    pub total_favorable: Balance,
    pub total_against: Balance,

    /// Used to detect wether a proposal is ready to accept votes or not
    pub initialized: bool,

    /// Used to list all opened locks on coins to later free those
    pub locks: Vec<LockIdentifier>,

    pub created_on: BlockNumber,
}

impl<Balance: Saturating + Copy + IntegerSquareRoot, BlockNumber, CurrencyId, LockIdentifier>
    ProposalState<Balance, BlockNumber, CurrencyId, LockIdentifier>
{
    pub fn record_vote(&mut self, favorable: bool, power: Balance) {
        if favorable {
            self.total_favorable = self.total_favorable.saturating_add(self.real_power(power));
        } else {
            self.total_against = self.total_against.saturating_add(self.real_power(power));
        }
    }

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
