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

use crate::{CheckRole, GenesisConfig, Module, Trait};
use codec::{Decode, Encode};
use frame_support::{impl_outer_dispatch, impl_outer_origin, parameter_types};
use governance_os_support::{
    acl::{CallFilter, Role},
    testing::{
        primitives::AccountId, AvailableBlockRatio, BlockHashCount, MaximumBlockLength,
        MaximumBlockWeight, ALICE,
    },
};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, DispatchInfoOf, IdentityLookup},
    RuntimeDebug,
};
use sp_std::{fmt::Debug, marker};

impl_outer_origin! {
    pub enum Origin for Test {}
}

impl_outer_dispatch! {
    pub enum Call for Test where origin: Origin {
        frame_system::System,
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Test;

impl frame_system::Trait for Test {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type PalletInfo = ();
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
}

#[derive(Eq, PartialEq, RuntimeDebug, Encode, Decode, Copy, Clone, Serialize, Deserialize)]
pub enum MockRoles {
    Root,
    RemarkOnly,
}
impl Role for MockRoles {}

pub struct MockCallFilter<T>(marker::PhantomData<T>);
impl<T: Trait> CallFilter<AccountId, Call, MockRoles> for MockCallFilter<T> {
    fn roles_for(
        _who: &AccountId,
        call: &Call,
        _info: &DispatchInfoOf<Call>,
        _len: usize,
    ) -> Vec<MockRoles> {
        match call {
            Call::System(frame_system::Call::remark(..)) => vec![MockRoles::RemarkOnly],
            Call::System(frame_system::Call::suicide()) => vec![], // Everybody can call it
            _ => vec![MockRoles::Root],
        }
    }
}

parameter_types! {
    pub const RootRole: MockRoles = MockRoles::Root;
}

impl Trait for Test {
    type Event = ();
    type Role = MockRoles;
    type RootRole = RootRole;
    type CallFilter = MockCallFilter<Test>;
}

pub type System = frame_system::Module<Test>;
pub type Bylaws = Module<Test>;
pub type MockCheckRole = CheckRole<Test>;

pub struct ExtBuilder {
    roles: Vec<(MockRoles, Option<AccountId>)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self { roles: vec![] }
    }
}

impl ExtBuilder {
    pub fn alice_as_root(self) -> Self {
        self.with_role(MockRoles::Root, Some(ALICE))
    }

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
