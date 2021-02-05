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

use crate::traits::{Currencies, ReservableCurrencies};
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::result;

/// End result of a proposal being closed.
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
    type ProposalID;

    /// How the parameters of a voting system are represented and set at the
    /// organization level.
    type Parameters;

    /// How voting data is passed to the underlying pallet.
    type VoteData;

    /// A proposal is being created. Handle any eventual registration and trigger
    /// an error if any preconditions are not met. Shall be called before any other
    /// state changes so that it is safe to fail here.
    fn initiate(proposal: Self::ProposalID, parameters: Self::Parameters) -> DispatchResult;

    /// Special function to handle the case when a proposal is being vetoed. This
    /// should clean any storage or state associated to the given proposal.
    fn veto(proposal: Self::ProposalID) -> DispatchResult;

    /// Handle the reception of a new vote for the given proposal. This should mutate any
    /// state linked to the proposal accordingly.
    fn vote(proposal: Self::ProposalID, data: Self::VoteData) -> DispatchResult;

    /// Handle the closure of a proposal or return an error if it cannot be closed because
    /// some conditions are not met. Shall return an indicator on wether the proposal is
    /// passing (should be executed) or not (should be discarded).
    fn close(proposal: Self::ProposalID) -> result::Result<ProposalResult, DispatchError>;
}

/// Called by the host pallet to let the developer implement custom voting actions
/// according to its own model.
pub trait VotingHooks {
    type AccountId;
    type BlockNumber;
    type Currencies: ReservableCurrencies<Self::AccountId>;
    type Data;
    type OrganizationId;
    type VotingSystem;

    /// Somebody is creating a proposal. Called before any state changes. This
    /// is where you have the ability to try and reserve a certain amount of coins
    /// for instance.
    fn on_create_proposal(
        voting_system: Self::VotingSystem,
        creator: &Self::AccountId,
        current_block: Self::BlockNumber,
    ) -> (DispatchResult, Self::Data);

    /// A proposal is going to be vetoed. Called before any state changes. This is
    /// where you have the possibility to free any funds reserved.
    fn on_veto_proposal(voting_system: Self::VotingSystem, data: Self::Data) -> DispatchResult;

    /// Handle an incoming vote. Returns a result and some updated metadata.
    fn on_decide_on_proposal(
        voting_system: Self::VotingSystem,
        data: Self::Data,
        voter: &Self::AccountId,
        power: <Self::Currencies as Currencies<Self::AccountId>>::Balance,
        in_support: bool,
    ) -> (DispatchResult, Self::Data);

    /// Return wether we should enable calls to closing the proposal. Closing a proposal
    /// means executing it if it passed and then cleaning the storage.
    fn can_close(
        voting_system: Self::VotingSystem,
        data: Self::Data,
        current_block: Self::BlockNumber,
    ) -> bool;

    /// Return if a proposal is passing. If it is, it will likely be executed in the same
    /// transaction and then closed.
    fn passing(voting_system: Self::VotingSystem, data: Self::Data) -> bool;

    /// Called before cleaning the storage related to a proposal and before its eventual
    /// execution. Last chance to prevent it from running!
    fn on_close_proposal(
        voting_system: Self::VotingSystem,
        data: Self::Data,
        executed: bool,
    ) -> DispatchResult;
}
