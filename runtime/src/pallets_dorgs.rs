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

use crate::{voting_router::RuntimeVotingRouter, Bylaws, Call, Event, Runtime, Tokens};
use frame_support::parameter_types;
use governance_os_primitives::{AccountId, Balance, BlockNumber, CurrencyId, Role};

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

impl governance_os_pallet_coin_voting::Trait for Runtime {
    type Event = Event;
    type Currencies = Tokens;
}

impl governance_os_pallet_plcr_voting::Trait for Runtime {
    type Event = Event;
    type Currencies = Tokens;
}

parameter_types! {
    // For now we keep this value small to avoid extrinsics with abusive weights.
    // Future upgrades should bring this value up without increasing the maximum
    // block weight.
    pub const MaxVotes: u32 = 100;
    pub const MaxExecutors: u32 = 100;
}

impl governance_os_pallet_organizations::Trait for Runtime {
    type Event = Event;
    type Call = Call;
    type RoleManager = Bylaws;
    type RoleBuilder = Role;
    type VotingRouter = RuntimeVotingRouter;
    type MaxVotes = MaxVotes;
    type MaxExecutors = MaxExecutors;
    type WeightInfo = ();
}
