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

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::Vec;

/// This structure is used to encode metadata about an organization.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OrganizationDetails<AccountId, VotingSystem> {
    /// A set of accounts that have access to the `apply_as` function
    /// of an organization.
    pub executors: Vec<AccountId>,

    /// Which voting system is in place. `executors` do not need to go
    /// through it due to their higher privilege permission.
    pub voting: VotingSystem,
}

impl<AccountId: Ord, VotingSystem> OrganizationDetails<AccountId, VotingSystem> {
    /// Sort all the vectors inside the strutcture.
    pub fn sort(&mut self) {
        self.executors.sort();
    }
}

/// Represent a proposal as stored by the pallet.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Proposal<AccountId, Call, Metadata> {
    pub creator: AccountId,
    pub call: Call,
    pub metadata: Metadata,
}
