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

use crate::{
    self as governance_os_pallet_bylaws, // compat with `mock_runtime`
    GenesisConfig,
};
use codec::{Decode, Encode};
use frame_support::{impl_outer_dispatch, impl_outer_origin, parameter_types};
use governance_os_support::{
    acl::Role,
    impl_enum_default, mock_runtime,
    testing::{
        primitives::{AccountId, CurrencyId},
        AvailableBlockRatio, BlockHashCount, MaximumBlockLength, MaximumBlockWeight, ROOT,
    },
};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    RuntimeDebug,
};

mock_runtime!(Test);

pub struct ExtBuilder {
    roles: Vec<(MockRoles, Option<AccountId>)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            roles: vec![(MockRoles::Root, Some(ROOT))],
        }
    }
}

impl ExtBuilder {
    pub fn with_role(mut self, role: MockRoles, target: Option<AccountId>) -> Self {
        self.roles.push((role, target));
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        GenesisConfig::<Test> { roles: self.roles }
            .assimilate_storage(&mut t)
            .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
