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
    types::{VoteData, VotingParameters},
    BalanceOf, CurrencyIdOf, Module, Trait,
};
use governance_os_pallet_tokens::CurrencyDetails;
use governance_os_support::{
    mock_runtime_with_currencies,
    testing::{ALICE, BOB, TEST_TOKEN_ID, TEST_TOKEN_OWNER},
};
use sp_runtime::{traits::Hash, Perbill};

mock_runtime_with_currencies!(Test);

impl Trait for Test {
    type Event = ();
    type Currencies = Tokens;
}

pub type PlcrVoting = Module<Test>;

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
        min_quorum: Perbill::from_percent(0),
        min_participation: Perbill::from_percent(0),
    }
}

pub fn mock_vote(
    decoy_power: BalanceOf<Test>,
    actual_power: BalanceOf<Test>,
    support: bool,
    salt: u64,
) -> (
    VoteData<BalanceOf<Test>, H256>,
    VoteData<BalanceOf<Test>, H256>,
) {
    let hashed = BlakeTwo256::hash_of(&(actual_power, support, salt));
    (
        VoteData::Commit(hashed, decoy_power),
        VoteData::Reveal(actual_power, support, salt),
    )
}
