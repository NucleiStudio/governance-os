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

use crate::{Module, RoleBuilder, Trait};
use governance_os_support::mock_runtime;

mock_runtime!(Test);

impl Trait for Test {
    type Event = ();
    type Call = Call;
    type RoleManager = Bylaws;
    type RoleBuilder = MockRoles;
    type Currencies = Tokens;
}

impl RoleBuilder for MockRoles {
    type OrganizationId = AccountId;
    type Role = MockRoles;

    fn create_organizations() -> Self::Role {
        MockRoles::CreateOrganizations
    }

    fn apply_as_organization(org_id: Self::OrganizationId) -> Self::Role {
        MockRoles::ApplyAsOrganization(org_id)
    }
}

pub type Organizations = Module<Test>;
