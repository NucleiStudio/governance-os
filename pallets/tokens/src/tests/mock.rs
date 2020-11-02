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

use crate::{CurrencyDetails, GenesisConfig, Module, NativeCurrencyAdapter, RoleBuilder, Trait};
use codec::{Decode, Encode};
use frame_support::{impl_outer_dispatch, impl_outer_origin, parameter_types};
pub use governance_os_support::{
    acl::Role,
    impl_enum_default, mock_runtime,
    testing::{
        primitives::{AccountId, Balance, CurrencyId},
        AvailableBlockRatio, BlockHashCount, MaximumBlockLength, MaximumBlockWeight, ALICE, BOB,
        TEST_TOKEN_ID, TEST_TOKEN_OWNER,
    },
    Currencies, ReservableCurrencies,
};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    RuntimeDebug,
};

mock_runtime!(Test, crate::AccountData<CurrencyId, Balance>);

impl RoleBuilder for MockRoles {
    type CurrencyId = CurrencyId;
    type Role = Self;

    fn transfer_currency(id: CurrencyId) -> Self {
        Self::TransferCurrency(id)
    }

    fn manage_currency(id: CurrencyId) -> Self {
        Self::ManageCurrency(id)
    }
}

impl Trait for Test {
    type Event = ();
    type CurrencyId = CurrencyId;
    type Balance = Balance;
    type WeightInfo = ();
    type AccountStore = System;
    type RoleManager = Bylaws;
    type RoleBuilder = MockRoles;
}

parameter_types! {
    pub const GetTestTokenId: CurrencyId = TEST_TOKEN_ID;
}

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
