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
    GenesisConfig, Module, OrganizationDetails, OrganizationDetailsOf, RoleBuilder, Trait,
};
use governance_os_pallet_tokens::CurrencyDetails;
use governance_os_support::{
    mock_runtime_with_currencies,
    testing::{ALICE, TEST_TOKEN_ID, TEST_TOKEN_OWNER},
};
use governance_os_voting::{ProposalMetadata, VotingSystems};

mock_runtime_with_currencies!(Test);

impl Trait for Test {
    type Event = ();
    type Call = Call;
    type RoleManager = Bylaws;
    type RoleBuilder = MockRoles;
    type Currencies = Tokens;
    type VotingSystem =
        VotingSystems<Balance, CurrencyId, BlockNumber, Self::Currencies, AccountId>;
    type ProposalMetadata = ProposalMetadata<AccountId, Balance, BlockNumber>;
    type VotingHooks = VotingSystems<Balance, CurrencyId, BlockNumber, Self::Currencies, AccountId>;
}

impl RoleBuilder for MockRoles {
    type OrganizationId = AccountId;
    type Role = MockRoles;

    fn create_organizations() -> Self::Role {
        MockRoles::CreateOrganizations
    }

    fn apply_as_organization(org_id: &Self::OrganizationId) -> Self::Role {
        MockRoles::ApplyAsOrganization(*org_id)
    }
}

pub type Organizations = Module<Test>;

pub struct ExtBuilder {
    can_create: Vec<AccountId>,
    orgs: Vec<OrganizationDetailsOf<Test>>,
    endowed_accounts: Vec<(CurrencyId, AccountId, Balance)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            can_create: vec![],
            orgs: vec![],
            endowed_accounts: vec![],
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

    pub fn with_default_orgs(mut self, nb: u32) -> Self {
        for _ in 0..nb {
            self.orgs.push(OrganizationDetails {
                executors: vec![],
                voting: VotingSystems::None,
            });
        }
        self
    }

    pub fn hundred_for_alice(mut self) -> Self {
        self.endowed_accounts.push((TEST_TOKEN_ID, ALICE, 100));
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

        GenesisConfig::<Test> {
            organizations: self.orgs,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
