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

use crate::ReservableCurrencies;
use frame_support::Parameter;
use sp_runtime::{
    traits::{MaybeSerializeDeserialize, Member},
    DispatchResult,
};

pub trait VotingSystem: Parameter + Member + MaybeSerializeDeserialize {}

/// Called by the host pallet to let the developer implement custom voting actions
/// according to its own model.
pub trait VotingHooks {
    type AccountId;
    type OrganizationId;
    type VotingSystem: VotingSystem;
    type Currencies: ReservableCurrencies<Self::AccountId>;
    type Data;

    /// Somebody is creating a proposal. Called before any state changes. This
    /// is where you have the ability to try and reserve a certain amount of coins
    /// for instance.
    fn on_creating_proposal(
        voting_system: Self::VotingSystem,
        creator: &Self::AccountId,
    ) -> (DispatchResult, Self::Data);
}
