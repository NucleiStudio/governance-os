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

use crate::{CurrencyDetails, GenesisConfig, Module, NativeCurrencyAdapter, Trait};
use frame_support::{impl_outer_origin, parameter_types};
pub use governance_os_support::{
    testing::{
        primitives::{AccountId, Balance, CurrencyId},
        AvailableBlockRatio, BlockHashCount, MaximumBlockLength, MaximumBlockWeight, ALICE, BOB,
        TEST_TOKEN_ID, TEST_TOKEN_OWNER,
    },
    Currencies, ReservableCurrencies,
};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

impl_outer_origin! {
    pub enum Origin for Test {}
}

#[derive(Clone, Eq, PartialEq)]
pub struct Test;

impl frame_system::Trait for Test {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Call = ();
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
    type AccountData = crate::AccountData<CurrencyId, Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
}

impl Trait for Test {
    type Event = ();
    type CurrencyId = CurrencyId;
    type Balance = Balance;
    type WeightInfo = ();
    type AccountStore = System;
}

parameter_types! {
    pub const GetTestTokenId: CurrencyId = TEST_TOKEN_ID;
}

pub type System = frame_system::Module<Test>;
pub type Tokens = Module<Test>;
pub type TokensCurrencyAdapter = NativeCurrencyAdapter<Test, GetTestTokenId>;

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

        GenesisConfig::<Test> {
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
