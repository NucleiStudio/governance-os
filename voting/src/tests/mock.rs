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

use crate::VotingSystems;
use governance_os_pallet_tokens::CurrencyDetails;
use governance_os_support::{
    mock_runtime_with_currencies,
    testing::{ALICE, TEST_TOKEN_ID, TEST_TOKEN_OWNER},
};

mock_runtime_with_currencies!(Test);

pub type MockVotingSystems = VotingSystems<Balance, CurrencyId, BlockNumber, Tokens, AccountId>;

pub struct ExtBuilder {
    endowed_accounts: Vec<(CurrencyId, AccountId, Balance)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            endowed_accounts: vec![],
        }
    }
}

impl ExtBuilder {
    pub fn hundred_for_alice(mut self) -> Self {
        self.endowed_accounts.push((TEST_TOKEN_ID, ALICE, 100));
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        governance_os_pallet_tokens::GenesisConfig::<Test> {
            endowed_accounts: self.endowed_accounts,
            currency_details: vec![(
                TEST_TOKEN_ID,
                CurrencyDetails {
                    owner: TEST_TOKEN_OWNER,
                    transferable: true,
                },
            )],
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
