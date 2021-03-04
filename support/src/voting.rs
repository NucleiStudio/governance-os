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

//! A set of common traits to voting systems.

use codec::{Decode, Encode};
use frame_support::Parameter;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::result;

/// End result of a proposal being closed.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ProposalResult {
    Passing,
    Failing,
}

/// A common trait accross all voting implementations to make it easy to change
/// between voting models or implementations.
/// A pallet implementing this trait is not necessarily in charge of storing
/// proposals but could stick to only to support the actual decison making
/// code while proposal storage is delegated to another pallet.
pub trait StandardizedVoting {
    /// How we represent the a proposal as passed to the underlying functions.
    /// This can be used to fetch any state associated to the proposal.
    type ProposalId;

    /// How the parameters of a voting system are represented and set at the
    /// organization level.
    type Parameters;

    /// How voting data is passed to the underlying pallet.
    type VoteData;

    /// How accounts are represented, used to identify voters.
    type AccountId;

    /// A proposal is being created. Handle any eventual registration and trigger
    /// an error if any preconditions are not met. Shall be called before any other
    /// state changes so that it is safe to fail here.
    fn initiate(proposal: Self::ProposalId, parameters: Self::Parameters) -> DispatchResult;

    /// Special function to handle the case when a proposal is being vetoed. This
    /// should clean any storage or state associated to the given proposal.
    fn veto(proposal: Self::ProposalId) -> DispatchResult;

    /// Handle the reception of a new vote for the given proposal. This should mutate any
    /// state linked to the proposal accordingly.
    fn vote(
        proposal: Self::ProposalId,
        voter: &Self::AccountId,
        data: Self::VoteData,
    ) -> DispatchResult;

    /// Handle the closure of a proposal or return an error if it cannot be closed because
    /// some conditions are not met. Shall return an indicator on wether the proposal is
    /// passing (should be executed) or not (should be discarded).
    fn close(proposal: Self::ProposalId) -> result::Result<ProposalResult, DispatchError>;
}

/// Used to route votes and related actions between different voting system implementations.
pub trait VotingRouter {
    /// How accounts are represented, used to identify voters.
    type AccountId;

    /// How the runtime defines a voting system. And how users can select it. Typically this
    /// would be a rust `enum`.
    type VotingSystemId: Parameter;

    /// How the parameters of a voting system are represented and set at the
    /// organization level. Typically an `enum` to account for all the different
    /// parameters for the different voting systems.
    type Parameters: Parameter;

    /// How we represent a proposal. Typically a `Hash`.
    type ProposalId: Parameter;

    /// How the runtime represents the different vote data of the different voting systems.
    /// Typically an `enum` to account for all the different voting systems.
    type VoteData: Parameter;

    /// Route the `initiate` call to the right `StandardizedVoting` implementation based
    /// on the value of `voting_systems`.
    fn initiate(
        voting_system: Self::VotingSystemId,
        proposal: Self::ProposalId,
        parameters: Self::Parameters,
    ) -> DispatchResult;

    /// Route the `veto` call to the right `StandardizedVoting` implementation based
    /// on the value of `voting_systems`.
    fn veto(voting_system: Self::VotingSystemId, proposal: Self::ProposalId) -> DispatchResult;

    /// Route the `vote` call to the right `StandardizedVoting` implementation based
    /// on the value of `voting_systems`.
    fn vote(
        voting_system: Self::VotingSystemId,
        proposal: Self::ProposalId,
        voter: &Self::AccountId,
        data: Self::VoteData,
    ) -> DispatchResult;

    /// Route the `close` call to the right `StandardizedVoting` implementation based
    /// on the value of `voting_systems`.
    fn close(
        voting_system: Self::VotingSystemId,
        proposal: Self::ProposalId,
    ) -> result::Result<ProposalResult, DispatchError>;
}
