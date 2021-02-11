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
use sp_runtime::RuntimeDebug;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
pub struct VotingParameters<CurrencyId> {
    pub voting_currency: CurrencyId,
}

pub struct VoteData<Balance> {
    pub in_support: bool,
    pub power: Balance,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
pub struct ProposalState<Balance, Parameters, LockIdentifier> {
    pub parameters: Parameters,
    pub total_favorable: Balance,
    pub total_against: Balance,

    /// Used to detect wether a proposal is ready to accept votes or not
    pub initialized: bool,

    /// Used to list all opened locks on coins to later free those
    pub locks: Vec<LockIdentifier>,
}
