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

//! Type definitions for the conviction voting pallet.

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{
    traits::{CheckedDiv, Saturating, UniqueSaturatedInto, Zero},
    DispatchError, DispatchResult, RuntimeDebug,
};
use sp_std::{result, vec::Vec};

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
    /// Decay parameter for the half life decay curve used to count vote's
    /// "conviction". Refer to https://en.wikipedia.org/wiki/Exponential_decay
    /// for the mathematical explanation.
    pub decay: u32,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Conviction<Balance> {
    /// Wether we are favorable to the proposal or opposed to it.
    pub in_support: bool,
    /// How much coins we want to stake on this decision.
    pub power: Balance,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ConvictionSnapshot<Balance, BlockNumber> {
    /// When was the last snapshot was taken.
    pub last_snapshot: BlockNumber,
    /// Amount of favorable conviction.
    pub favorable: Balance,
    /// Amount of opposed conviction.
    pub against: Balance,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ProposalState<AccountId, Balance, BlockNumber, CurrencyId> {
    /// Parameters that this proposal was created with.
    pub parameters: VotingParameters<BlockNumber, CurrencyId>,
    /// Record when the proposal was created. Used to know when it is expired
    /// (when the ttl is over).
    pub created_on: BlockNumber,
    /// Conviction votes on this proposal.
    pub convictions: Vec<(AccountId, BlockNumber, Conviction<Balance>)>,
    /// How much tokens are staked in favor of the proposal.
    pub conviction_for: Balance,
    /// How much tokens are staked against the proposal.
    pub conviction_against: Balance,
    /// Snapshot of our different conviction records. Used to compute
    /// the current conviction progressively.
    pub snapshot: ConvictionSnapshot<Balance, BlockNumber>,
}
impl<
        AccountId: Clone,
        Balance: Zero + Saturating + CheckedDiv + Copy + Clone + From<u32>,
        BlockNumber: Clone + Copy + Saturating + UniqueSaturatedInto<u32>,
        CurrencyId,
    > ProposalState<AccountId, Balance, BlockNumber, CurrencyId>
{
    /// Compute the current amount of conviction for or against the proposal
    /// and save its latest value in the proposal state. `now` should be the
    /// current block number. `decay` is the decay variable of the half life
    /// exponential formula.
    ///
    /// Refer to this [work from EthParis](https://hackmd.io/@EtCgawsxS2mC6-Q0rCqhAw/rJMvfgOv4?type=view).
    pub fn mutate_conviction_snapshot(
        &mut self,
        now: BlockNumber,
        decay: (Balance, Balance),
    ) -> DispatchResult {
        let (d, aD) = decay;

        self.snapshot.favorable = self.conviction_for;
        self.snapshot.against = self.conviction_against;

        Ok(())
    }
}
