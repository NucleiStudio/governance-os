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

use crate as governance_os_pallet_bylaws;
use codec::{Decode, Encode};
use frame_support::{construct_runtime, parameter_types, traits::GenesisBuild};
use governance_os_support::{
    impl_enum_default,
    testing::{
        primitives::{AccountId, BlockNumber},
        ROOT,
    },
};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup, RuntimeDebug};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Bylaws: governance_os_pallet_bylaws::{Module, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
    pub static ExistentialDeposit: u64 = 0;
}
impl frame_system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = BlockWeights;
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Call = Call;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}
#[derive(
    Eq,
    PartialEq,
    RuntimeDebug,
    Encode,
    Decode,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    Ord,
    PartialOrd,
)]
pub enum MockRoles {
    Root,
    RemarkOnly,
}
impl_enum_default!(MockRoles, RemarkOnly);
impl governance_os_pallet_bylaws::RoleBuilder for MockRoles {
    type Role = MockRoles;

    fn manage_roles() -> MockRoles {
        Self::root()
    }

    fn root() -> MockRoles {
        MockRoles::Root
    }
}

parameter_types! {
    pub const RootRole: MockRoles = MockRoles::Root;
    pub const MaxRoles: u32 = 5;
}

impl governance_os_pallet_bylaws::Config for Test {
    type Event = Event;
    type Role = MockRoles;
    type WeightInfo = ();
    type MaxRoles = MaxRoles;
    type RoleBuilder = MockRoles;
}

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

        governance_os_pallet_bylaws::GenesisConfig::<Test> { roles: self.roles }
            .assimilate_storage(&mut t)
            .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
