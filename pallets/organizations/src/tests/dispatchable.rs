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

use super::mock::{
    Bylaws, Call, ExtBuilder, MockRoles, MockVotingSystem, Organizations, Test, Tokens,
};
use crate::{Error, OrganizationDetails, Proposals, RoleBuilder};
use frame_support::{assert_noop, assert_ok, StorageMap};
use frame_system::RawOrigin;
use governance_os_support::{
    acl::{AclError, RoleManager},
    testing::{ALICE, BOB, CHARLIE, TEST_TOKEN_ID},
    ReservableCurrencies,
};

fn make_proposal() -> Box<Call> {
    Box::new(Call::System(frame_system::Call::remark(vec![])))
}

#[test]
fn create_increments_counter_and_save_details_and_configure_roles() {
    ExtBuilder::default()
        .alice_can_create_orgs()
        .build()
        .execute_with(|| {
            assert_ok!(Organizations::create(
                RawOrigin::Signed(ALICE).into(),
                OrganizationDetails {
                    // We intentionally make it unsorted for the test
                    executors: vec![CHARLIE, BOB],
                    voting: Default::default(),
                }
            ));
            assert_eq!(Organizations::counter(), 1);

            let org_id = Organizations::org_id_for(0);

            let params = Organizations::parameters(org_id);
            assert_eq!(params.executors.as_slice(), [BOB, CHARLIE]);

            assert!(Bylaws::has_role(
                &CHARLIE,
                MockRoles::apply_as_organization(&org_id)
            ));
            assert!(Bylaws::has_role(
                &BOB,
                MockRoles::apply_as_organization(&org_id)
            ));
        })
}

#[test]
fn apply_as() {
    ExtBuilder::default()
        .alice_can_create_orgs()
        .build()
        .execute_with(|| {
            assert_ok!(Organizations::create(
                RawOrigin::Signed(ALICE).into(),
                OrganizationDetails {
                    executors: vec![ALICE],
                    voting: Default::default(),
                }
            ));
            assert_ok!(Organizations::apply_as(
                RawOrigin::Signed(ALICE).into(),
                Organizations::org_id_for(0),
                make_proposal(),
            ));
        })
}

#[test]
fn mutate_save_details_and_update_roles() {
    ExtBuilder::default()
        .alice_can_create_orgs()
        .build()
        .execute_with(|| {
            assert_ok!(Organizations::create(
                RawOrigin::Signed(ALICE).into(),
                OrganizationDetails {
                    executors: vec![ALICE, BOB],
                    voting: Default::default(),
                }
            ));
            let org_id = Organizations::org_id_for(0);
            assert_ok!(Organizations::mutate(
                RawOrigin::Signed(org_id).into(),
                OrganizationDetails {
                    executors: vec![ALICE, CHARLIE],
                    voting: Default::default(),
                },
            ));

            let params = Organizations::parameters(org_id);
            assert_eq!(params.executors.as_slice(), [ALICE, CHARLIE]);

            // BOB lost permissions
            assert!(!Bylaws::has_role(
                &BOB,
                MockRoles::apply_as_organization(&org_id)
            ));

            // ALICE kept them
            assert!(Bylaws::has_role(
                &ALICE,
                MockRoles::apply_as_organization(&org_id)
            ));

            // CHARLIE won them
            assert!(Bylaws::has_role(
                &CHARLIE,
                MockRoles::apply_as_organization(&org_id)
            ));
        })
}

#[test]
fn create_fail_if_not_corect_role() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Organizations::create(
                RawOrigin::Signed(ALICE).into(),
                OrganizationDetails::default()
            ),
            AclError::MissingRole
        );
    })
}

#[test]
fn apply_as_fail_if_not_correct_role() {
    ExtBuilder::default()
        .with_default_orgs(1)
        .build()
        .execute_with(|| {
            assert_noop!(
                Organizations::apply_as(
                    RawOrigin::Signed(ALICE).into(),
                    Organizations::org_id_for(0),
                    make_proposal()
                ),
                AclError::MissingRole
            );
        })
}

#[test]
fn mutate_fail_if_not_the_org_itself() {
    ExtBuilder::default()
        .with_default_orgs(1)
        .build()
        .execute_with(|| {
            assert_noop!(
                Organizations::mutate(
                    RawOrigin::Signed(ALICE).into(),
                    OrganizationDetails::default()
                ),
                Error::<Test>::NotAnOrganization,
            );
        })
}

#[test]
fn create_proposal_with_hook() {
    ExtBuilder::default()
        .hundred_for_alice()
        .with_org(OrganizationDetails {
            executors: vec![],
            voting: MockVotingSystem::SimpleReserveWithCreationFee(TEST_TOKEN_ID, 2),
        })
        .build()
        .execute_with(|| {
            let org_id = Organizations::org_id_for(0);
            let proposal = make_proposal();
            let proposal_id = Organizations::proposal_id(&org_id, proposal.clone());

            assert_ok!(Organizations::create_proposal(
                RawOrigin::Signed(ALICE).into(),
                org_id,
                proposal
            ));

            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 2);
            assert!(Proposals::<Test>::contains_key(proposal_id));
            assert_eq!(Organizations::proposals(proposal_id).creator, ALICE);
        })
}

#[test]
fn create_same_proposal_later_works() {
    ExtBuilder::default()
        .hundred_for_alice()
        .with_org(OrganizationDetails {
            executors: vec![],
            voting: MockVotingSystem::SimpleReserveWithCreationFee(TEST_TOKEN_ID, 2),
        })
        .build()
        .execute_with(|| {
            let org_id = Organizations::org_id_for(0);
            let proposal = make_proposal();
            let first_proposal_id = Organizations::proposal_id(&org_id, proposal.clone());

            assert_ok!(Organizations::create_proposal(
                RawOrigin::Signed(ALICE).into(),
                org_id,
                proposal.clone()
            ));

            // Change block numbers so that proposal IDs are different
            frame_system::Module::<Test>::set_block_number(
                frame_system::Module::<Test>::block_number() + 1,
            );
            let second_proposal_id = Organizations::proposal_id(&org_id, proposal.clone());
            assert_ok!(Organizations::create_proposal(
                RawOrigin::Signed(ALICE).into(),
                org_id,
                proposal.clone()
            ));

            assert!(first_proposal_id != second_proposal_id);
            assert_eq!(
                Organizations::proposals(first_proposal_id),
                Organizations::proposals(second_proposal_id)
            );
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 4);
        })
}

#[test]
fn create_proposal_fail_if_not_org() {
    ExtBuilder::default()
        .hundred_for_alice()
        .with_org(OrganizationDetails {
            executors: vec![],
            voting: MockVotingSystem::SimpleReserveWithCreationFee(TEST_TOKEN_ID, 2),
        })
        .build()
        .execute_with(|| {
            let org_id = Organizations::org_id_for(0);
            let proposal = make_proposal();

            assert_ok!(Organizations::create_proposal(
                RawOrigin::Signed(ALICE).into(),
                org_id,
                proposal.clone()
            ));

            // Block number did not change so we'd end up generating the same hash
            assert_noop!(
                Organizations::create_proposal(RawOrigin::Signed(ALICE).into(), org_id, proposal),
                Error::<Test>::ProposalDuplicate
            );
        })
}

#[test]
fn create_proposal_fail_if_duplicate_at_same_block() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Organizations::create_proposal(
                RawOrigin::Signed(ALICE).into(),
                Organizations::org_id_for(0),
                make_proposal()
            ),
            Error::<Test>::NotAnOrganization
        );
    })
}

#[test]
fn create_proposal_fail_if_hook_fail() {
    // A none voting system will return an error in our mocks.

    ExtBuilder::default()
        .hundred_for_alice()
        .with_org(OrganizationDetails {
            executors: vec![],
            voting: MockVotingSystem::None,
        })
        .build()
        .execute_with(|| {
            let org_id = Organizations::org_id_for(0);
            let proposal = make_proposal();

            assert_noop!(
                Organizations::create_proposal(
                    RawOrigin::Signed(ALICE).into(),
                    org_id,
                    proposal.clone()
                ),
                "none voting system"
            );
        })
}
