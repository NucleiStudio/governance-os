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

use crate::{Call, Event, Runtime};
use frame_support::parameter_types;
use governance_os_primitives::{AccountId, Role};
use sp_runtime::traits::DispatchInfoOf;
use sp_std::prelude::*;

pub struct CallFilter;
impl governance_os_support::acl::CallFilter<AccountId, Call, Role> for CallFilter {
    fn roles_for(
        _who: &AccountId,
        call: &Call,
        _info: &DispatchInfoOf<Call>,
        _len: usize,
    ) -> Vec<Role> {
        match call {
            // TODO: add sudo module using the root bylaw
            Call::Tokens(governance_os_pallet_tokens::Call::create(..)) => {
                vec![Role::CreateCurrencies]
            }
            _ => vec![],
        }
    }
}

parameter_types! {
    pub const RootRole: Role = Role::Root;
    pub const MaxRoles: u32 = 4;
}

impl governance_os_pallet_bylaws::Trait for Runtime {
    type Event = Event;
    type Role = Role;
    type RootRole = RootRole;
    type CallFilter = CallFilter;
    type WeightInfo = ();
    type MaxRoles = MaxRoles;
}
