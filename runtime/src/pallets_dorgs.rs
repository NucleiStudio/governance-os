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

use crate::{Bylaws, Call, Event, Runtime, Tokens};
use frame_support::parameter_types;
use governance_os_primitives::{AccountId, Balance, BlockNumber, CurrencyId, Role};
use governance_os_voting::{ProposalMetadata, VotingSystems};

parameter_types! {
    pub const MaxRoles: u32 = 50;
}

impl governance_os_pallet_bylaws::Trait for Runtime {
    type Event = Event;
    type Role = Role;
    type WeightInfo = ();
    type MaxRoles = MaxRoles;
    type RoleBuilder = Role;
}

parameter_types! {
    pub const MaxVotes: u32 = 10_000;
    pub const MaxExecutors: u32 = 100;
}

impl governance_os_pallet_organizations::Trait for Runtime {
    type Event = Event;
    type Call = Call;
    type RoleManager = Bylaws;
    type RoleBuilder = Role;
    type Currencies = Tokens;
    type VotingSystem =
        VotingSystems<Balance, CurrencyId, BlockNumber, Self::Currencies, AccountId>;
    type ProposalMetadata = ProposalMetadata<AccountId, Balance, BlockNumber>;
    type VotingHooks = VotingSystems<Balance, CurrencyId, BlockNumber, Self::Currencies, AccountId>;
    type MaxVotes = MaxVotes;
    type MaxExecutors = MaxExecutors;
    type WeightInfo = ();
}
