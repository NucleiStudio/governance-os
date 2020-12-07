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
use sp_runtime::DispatchResult;

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