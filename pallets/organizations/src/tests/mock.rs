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
    self as governance_os_pallet_organizations, OrganizationDetails, OrganizationDetailsOf,
    VotingRouter,
};
use codec::{Decode, Encode};
use frame_support::{
    construct_runtime,
    dispatch::{DispatchError, DispatchResult},
    parameter_types,
    traits::GenesisBuild,
};
use governance_os_support::{
    impl_enum_default,
    testing::{
        primitives::{AccountId, BlockNumber},
        ALICE,
    },
    traits::ProposalResult,
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
        Organizations: governance_os_pallet_organizations::{Module, Call, Storage, Event<T>},
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
    CreateOrganizations,
    ApplyAsOrganization(AccountId),
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

parameter_types! {
    pub const MaxVotes: u32 = 100;
    pub const MaxExecutors: u32 = 100;
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Serialize, Deserialize)]
pub enum MockVotingSystemId {
    WithResult(ProposalResult),
    FailInitiate,
    FailVeto,
    FailVote,
    FailClose,
}

pub struct MockVotingRouter;
impl VotingRouter for MockVotingRouter {
    type AccountId = AccountId;
    type VotingSystemId = MockVotingSystemId;
    type Parameters = ();
    type ProposalId = H256;
    type VoteData = ();

    fn initiate(
        voting_system: Self::VotingSystemId,
        _proposal: Self::ProposalId,
        _parameters: Self::Parameters,
    ) -> DispatchResult {
        match voting_system {
            MockVotingSystemId::FailInitiate => Err("fail".into()),
            _ => Ok(()),
        }
    }

    fn veto(voting_system: Self::VotingSystemId, _proposal: Self::ProposalId) -> DispatchResult {
        match voting_system {
            MockVotingSystemId::FailVeto => Err("fail".into()),
            _ => Ok(()),
        }
    }

    fn vote(
        voting_system: Self::VotingSystemId,
        _proposal: Self::ProposalId,
        _voter: &Self::AccountId,
        _data: Self::VoteData,
    ) -> DispatchResult {
        match voting_system {
            MockVotingSystemId::FailVote => Err("fail".into()),
            _ => Ok(()),
        }
    }

    fn close(
        voting_system: Self::VotingSystemId,
        _proposal: Self::ProposalId,
    ) -> Result<ProposalResult, DispatchError> {
        match voting_system {
            MockVotingSystemId::WithResult(res) => Ok(res),
            MockVotingSystemId::FailClose => Err("fail".into()),
            _ => Ok(ProposalResult::Passing),
        }
    }
}

impl governance_os_pallet_organizations::RoleBuilder for MockRoles {
    type OrganizationId = AccountId;
    type Role = MockRoles;

    fn create_organizations() -> Self::Role {
        MockRoles::CreateOrganizations
    }

    fn apply_as_organization(org_id: &Self::OrganizationId) -> Self::Role {
        MockRoles::ApplyAsOrganization(*org_id)
    }
}

impl governance_os_pallet_organizations::Config for Test {
    type Event = Event;
    type Call = Call;
    type RoleManager = Bylaws;
    type RoleBuilder = MockRoles;
    type VotingRouter = MockVotingRouter;
    type MaxVotes = MaxVotes;
    type MaxExecutors = MaxExecutors;
    type WeightInfo = ();
}

pub struct ExtBuilder {
    can_create: Vec<AccountId>,
    orgs: Vec<OrganizationDetailsOf<Test>>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            can_create: vec![],
            orgs: vec![],
        }
    }
}

impl ExtBuilder {
    pub fn alice_can_create_orgs(mut self) -> Self {
        self.can_create.push(ALICE);
        self
    }

    pub fn with_org(mut self, org: OrganizationDetailsOf<Test>) -> Self {
        self.orgs.push(org);
        self
    }

    pub fn with_default_org(mut self) -> Self {
        self.orgs.push(OrganizationDetails {
            executors: vec![],
            voting: (MockVotingSystemId::WithResult(ProposalResult::Passing), ()),
        });
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        governance_os_pallet_bylaws::GenesisConfig::<Test> {
            roles: self
                .can_create
                .into_iter()
                .map(|account| (MockRoles::CreateOrganizations, Some(account)))
                .collect::<Vec<_>>(),
        }
        .assimilate_storage(&mut t)
        .unwrap();

        governance_os_pallet_organizations::GenesisConfig::<Test> {
            organizations: self.orgs,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

pub fn make_proposal() -> Box<Call> {
    Box::new(Call::System(frame_system::Call::remark(vec![])))
}
