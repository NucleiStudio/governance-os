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
    self as governance_os_pallet_plcr_voting, BalanceOf, CurrencyIdOf, VoteData, VotingParameters,
};
use codec::{Decode, Encode};
use frame_support::{construct_runtime, parameter_types, traits::GenesisBuild};
use governance_os_pallet_tokens::CurrencyDetails;
use governance_os_support::{
    impl_enum_default,
    testing::{
        primitives::{AccountId, Balance, BlockNumber, CurrencyId},
        ALICE, BOB, TEST_TOKEN_ID, TEST_TOKEN_OWNER,
    },
};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, Hash, IdentityLookup},
    RuntimeDebug,
};

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
        Tokens: governance_os_pallet_tokens::{Module, Call, Storage, Event<T>},
        PlcrVoting: governance_os_pallet_plcr_voting::{Module, Call, Storage, Event<T>},
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
    CreateCurrencies,
    TransferCurrency(CurrencyId),
    ManageCurrency(CurrencyId),
}
impl_enum_default!(MockRoles, Root);
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

impl governance_os_pallet_tokens::RoleBuilder for MockRoles {
    type CurrencyId = CurrencyId;
    type Role = Self;

    fn transfer_currency(id: CurrencyId) -> Self {
        Self::TransferCurrency(id)
    }

    fn manage_currency(id: CurrencyId) -> Self {
        Self::ManageCurrency(id)
    }

    fn create_currencies() -> Self {
        Self::CreateCurrencies
    }
}

impl governance_os_pallet_tokens::Config for Test {
    type Event = Event;
    type CurrencyId = CurrencyId;
    type Balance = Balance;
    type WeightInfo = ();
    type RoleManager = Bylaws;
    type RoleBuilder = MockRoles;
}

parameter_types! {
    pub const Decay: Balance = 1;
}

impl governance_os_pallet_plcr_voting::Config for Test {
    type Currencies = Tokens;
    type Event = Event;
}

pub struct ExtBuilder {
    endowed_accounts: Vec<(CurrencyId, AccountId, Balance)>,
    test_token_details: CurrencyDetails<AccountId>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            endowed_accounts: vec![],
            test_token_details: CurrencyDetails {
                owner: TEST_TOKEN_OWNER,
                transferable: true,
            },
        }
    }
}

impl ExtBuilder {
    pub fn balances(mut self, endowed_accounts: Vec<(CurrencyId, AccountId, Balance)>) -> Self {
        self.endowed_accounts = endowed_accounts;
        self
    }

    pub fn one_hundred_for_alice_n_bob(self) -> Self {
        self.balances(vec![(TEST_TOKEN_ID, ALICE, 100), (TEST_TOKEN_ID, BOB, 100)])
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        governance_os_pallet_bylaws::GenesisConfig::<Test> {
            roles: vec![(MockRoles::CreateCurrencies, None)], // Everybody can create currencies
        }
        .assimilate_storage(&mut t)
        .unwrap();

        governance_os_pallet_tokens::GenesisConfig::<Test> {
            endowed_accounts: self.endowed_accounts,
            currency_details: vec![(TEST_TOKEN_ID, self.test_token_details)],
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

pub fn mock_parameters() -> VotingParameters<BlockNumber, CurrencyIdOf<Test>> {
    VotingParameters {
        commit_duration: 10,
        reveal_duration: 10,
        voting_currency: TEST_TOKEN_ID,
        min_quorum: 50,
        min_participation: 33,
    }
}

pub fn mock_vote(
    power: BalanceOf<Test>,
    support: bool,
    salt: u64,
) -> (
    VoteData<BalanceOf<Test>, H256>,
    VoteData<BalanceOf<Test>, H256>,
) {
    let hashed = BlakeTwo256::hash_of(&(power, support, salt));
    (
        VoteData::Commit(hashed),
        VoteData::Reveal(power, support, salt),
    )
}

pub fn advance_blocks(blocks: BlockNumber) {
    let now = System::block_number();
    System::set_block_number(now.saturating_add(blocks));
}
