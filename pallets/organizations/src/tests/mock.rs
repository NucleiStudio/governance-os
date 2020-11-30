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

use crate::{GenesisConfig, Module, OrganizationDetailsOf, RoleBuilder, Trait};
use governance_os_pallet_tokens::CurrencyDetails;
use governance_os_support::{
    mock_runtime_with_currencies,
    testing::{ALICE, TEST_TOKEN_ID, TEST_TOKEN_OWNER},
    voting::{VotingHooks, VotingSystem},
    ReservableCurrencies,
};
use sp_runtime::DispatchResult;

mock_runtime_with_currencies!(Test);

#[derive(Eq, PartialEq, RuntimeDebug, Encode, Decode, Copy, Clone, Serialize, Deserialize)]
pub enum MockVotingSystem {
    None,
    SimpleReserveWithCreationFee(CurrencyId, Balance),
}
impl_enum_default!(MockVotingSystem, None);
impl VotingSystem for MockVotingSystem {}

#[derive(Default, Eq, PartialEq, RuntimeDebug, Encode, Decode, Clone, Serialize, Deserialize)]
pub struct VotingSystemMetadata {
    // A more efficient ways to store it would probably be a BTree
    pub coins_locked: Vec<(AccountId, Balance)>,
}

impl VotingHooks for MockVotingSystem {
    type AccountId = AccountId;
    type OrganizationId = AccountId;
    type VotingSystem = Self;
    type Currencies = Tokens;
    type Data = VotingSystemMetadata;

    fn on_creating_proposal(
        voting_system: Self::VotingSystem,
        creator: &Self::AccountId,
    ) -> (DispatchResult, Self::Data) {
        match voting_system {
            Self::SimpleReserveWithCreationFee(currency_id, creation_fee) => (
                Self::Currencies::reserve(currency_id, creator, creation_fee),
                VotingSystemMetadata {
                    coins_locked: vec![(*creator, creation_fee)],
                },
            ),
            _ => (Err("none voting system".into()), Default::default()),
        }
    }

    fn on_veto_proposal(voting_system: Self::VotingSystem, data: Self::Data) -> DispatchResult {
        match voting_system {
            Self::SimpleReserveWithCreationFee(currency_id, _creation_fee) => {
                data.coins_locked.iter().for_each(|(account, balance)| {
                    drop(Self::Currencies::unreserve(currency_id, account, *balance));
                });
                Ok(())
            }
            _ => Err("none voting system".into()),
        }
    }
}

impl Trait for Test {
    type Event = ();
    type Call = Call;
    type RoleManager = Bylaws;
    type RoleBuilder = MockRoles;
    type Currencies = Tokens;
    type VotingSystem = MockVotingSystem;
    type ProposalMetadata = VotingSystemMetadata;
    type VotingHooks = MockVotingSystem;
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
            self.orgs.push(OrganizationDetailsOf::<Test>::default());
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
