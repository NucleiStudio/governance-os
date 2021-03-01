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
    make_proposal, Bylaws, ExtBuilder, MockRoles, MockVotingSystemId, Organizations, Test,
};
use crate::{Error, OrganizationDetails, Proposal, Proposals, RoleBuilder};
use codec::Encode;
use frame_support::{assert_noop, assert_ok, weights::GetDispatchInfo, StorageMap};
use frame_system::RawOrigin;
use governance_os_support::{
    errors::AclError,
    testing::{ALICE, BOB, CHARLIE},
    traits::{ProposalResult, RoleManager},
};

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
                    voting: (MockVotingSystemId::WithResult(ProposalResult::Passing), ()),
                }
            ));
            assert_eq!(Organizations::counter(), 1);

            let org_id = Organizations::org_id_for(0);

            let params = Organizations::parameters(org_id).unwrap();
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
                    voting: (MockVotingSystemId::WithResult(ProposalResult::Passing), ()),
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
                    voting: (MockVotingSystemId::WithResult(ProposalResult::Passing), ()),
                }
            ));
            let org_id = Organizations::org_id_for(0);
            assert_ok!(Organizations::mutate(
                RawOrigin::Signed(org_id).into(),
                OrganizationDetails {
                    executors: vec![ALICE, CHARLIE],
                    voting: (MockVotingSystemId::WithResult(ProposalResult::Passing), ()),
                },
            ));

            let params = Organizations::parameters(org_id).unwrap();
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
                OrganizationDetails {
                    executors: vec![],
                    voting: (MockVotingSystemId::WithResult(ProposalResult::Passing), ()),
                }
            ),
            AclError::MissingRole
        );
    })
}

#[test]
fn apply_as_fail_if_not_correct_role() {
    ExtBuilder::default()
        .with_default_org()
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
        .with_default_org()
        .build()
        .execute_with(|| {
            assert_noop!(
                Organizations::mutate(
                    RawOrigin::Signed(ALICE).into(),
                    OrganizationDetails {
                        executors: vec![],
                        voting: (MockVotingSystemId::WithResult(ProposalResult::Passing), ()),
                    }
                ),
                Error::<Test>::NotAnOrganization,
            );
        })
}

#[test]
fn create_proposal() {
    ExtBuilder::default()
        .with_default_org()
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

            assert!(Proposals::<Test>::contains_key(proposal_id));
            let proposal = Organizations::proposals(proposal_id).unwrap();
            assert_eq!(proposal.org, org_id);
            assert_eq!(
                proposal.voting,
                MockVotingSystemId::WithResult(ProposalResult::Passing)
            );
        })
}

#[test]
fn create_proposal_fail_if_duplicate() {
    ExtBuilder::default()
        .with_default_org()
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
fn create_proposal_fail_if_target_is_not_an_org() {
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
    ExtBuilder::default()
        .with_org(OrganizationDetails {
            executors: vec![],
            voting: (MockVotingSystemId::FailInitiate, ()),
        })
        .build()
        .execute_with(|| {
            let org_id = Organizations::org_id_for(0);
            assert_noop!(
                Organizations::create_proposal(
                    RawOrigin::Signed(ALICE).into(),
                    org_id,
                    make_proposal()
                ),
                "fail"
            );
        })
}

// veto_proposal
#[test]
fn veto_proposal() {
    ExtBuilder::default()
        .with_default_org()
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

            assert_ok!(Organizations::veto_proposal(
                RawOrigin::Signed(org_id).into(),
                proposal_id
            ));

            assert!(!Proposals::<Test>::contains_key(proposal_id)); // Proposal was deleted
        })
}

#[test]
fn veto_proposal_fail_if_not_called_by_an_org() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Organizations::veto_proposal(
                RawOrigin::Signed(ALICE).into(),
                Organizations::proposal_id(&Organizations::org_id_for(0), make_proposal()),
            ),
            Error::<Test>::NotAnOrganization,
        );
    })
}

#[test]
fn veto_proposal_fail_if_called_on_wrong_proposal() {
    ExtBuilder::default()
        .with_default_org()
        // Need to create test organization to avoid the "NotAnOrganization" error
        .with_default_org()
        .build()
        .execute_with(|| {
            let org_id = Organizations::org_id_for(0);
            let proposal = make_proposal();
            let proposal_id = Organizations::proposal_id(&org_id, proposal.clone());

            assert_ok!(Organizations::create_proposal(
                RawOrigin::Signed(ALICE).into(),
                org_id,
                proposal.clone()
            ));

            // Not the right org id
            assert_noop!(
                Organizations::veto_proposal(
                    RawOrigin::Signed(Organizations::org_id_for(1)).into(),
                    proposal_id
                ),
                Error::<Test>::ProposalNotForOrganization
            );

            assert_noop!(
                Organizations::veto_proposal(
                    RawOrigin::Signed(org_id).into(),
                    Organizations::proposal_id(&Organizations::org_id_for(1), proposal)
                ),
                Error::<Test>::ProposalNotFound
            );
        })
}

#[test]
fn veto_proposal_fail_if_hook_fail() {
    ExtBuilder::default()
        .with_default_org()
        .build()
        .execute_with(|| {
            let org_id = Organizations::org_id_for(0);
            let proposal = make_proposal();
            let proposal_id = Organizations::proposal_id(&org_id, proposal);

            // We have to insert a fake proposal and bypass the hook for the `None` voting system
            Proposals::<Test>::insert(
                &proposal_id,
                Proposal {
                    org: org_id,
                    call: vec![],
                    voting: MockVotingSystemId::FailVeto,
                },
            );

            assert_noop!(
                Organizations::veto_proposal(RawOrigin::Signed(org_id).into(), proposal_id),
                "fail"
            );
        })
}

#[test]
fn decide_on_proposal() {
    ExtBuilder::default()
        .with_default_org()
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

            assert_ok!(Organizations::decide_on_proposal(
                RawOrigin::Signed(ALICE).into(),
                proposal_id,
                ()
            ));
        })
}

#[test]
fn decide_on_proposal_fails_if_does_not_exists() {
    ExtBuilder::default().build().execute_with(|| {
        let proposal_id =
            Organizations::proposal_id(&Organizations::org_id_for(0), make_proposal());

        assert_noop!(
            Organizations::decide_on_proposal(RawOrigin::Signed(ALICE).into(), proposal_id, ()),
            Error::<Test>::ProposalNotFound
        );
    })
}

#[test]
fn decide_on_proposal_fails_if_hook_fails() {
    ExtBuilder::default()
        .with_default_org()
        .build()
        .execute_with(|| {
            let org_id = Organizations::org_id_for(0);
            let proposal = make_proposal();
            let proposal_id = Organizations::proposal_id(&org_id, proposal);

            // We have to insert a fake proposal and bypass the hook for the `None` voting system
            Proposals::<Test>::insert(
                &proposal_id,
                Proposal {
                    org: org_id,
                    call: vec![],
                    voting: MockVotingSystemId::FailVote,
                },
            );

            assert_noop!(
                Organizations::decide_on_proposal(RawOrigin::Signed(ALICE).into(), proposal_id, ()),
                "fail"
            );
        })
}

// DRY principle for testing the closing of proposals.
// Basically generate two tests based on wether a proposal
// should pass or not.
// Also just wanted to write some cool macro code.
macro_rules! test_close_proposal {
    ($test_name:ident, $proposal_result:tt) => {
        #[test]
        fn $test_name() {
            ExtBuilder::default()
                .with_org(OrganizationDetails {
                    executors: vec![],
                    voting: (
                        MockVotingSystemId::WithResult(ProposalResult::$proposal_result),
                        (),
                    ),
                })
                .build()
                .execute_with(|| {
                    let org_id = Organizations::org_id_for(0);
                    let proposal = make_proposal();
                    let proposal_id = Organizations::proposal_id(&org_id, proposal.clone());

                    assert_ok!(Organizations::create_proposal(
                        RawOrigin::Signed(ALICE).into(),
                        org_id,
                        proposal.clone()
                    ));

                    assert_ok!(Organizations::close_proposal(
                        RawOrigin::Signed(ALICE).into(),
                        proposal_id,
                        proposal.get_dispatch_info().weight
                    ));

                    // Deleted the proposal
                    assert!(!Proposals::<Test>::contains_key(proposal_id));
                })
        }
    };
}

test_close_proposal!(close_passing_proposal, Passing);
test_close_proposal!(close_failing_proposal, Failing);

#[test]
fn close_fails_if_proposal_weight_bound_is_too_small() {
    ExtBuilder::default()
        .with_default_org()
        .build()
        .execute_with(|| {
            let org_id = Organizations::org_id_for(0);
            let proposal = make_proposal();
            let proposal_id = Organizations::proposal_id(&org_id, proposal.clone());

            assert_ok!(Organizations::create_proposal(
                RawOrigin::Signed(ALICE).into(),
                org_id,
                proposal.clone()
            ));

            assert_noop!(
                Organizations::close_proposal(
                    RawOrigin::Signed(ALICE).into(),
                    proposal_id,
                    proposal.get_dispatch_info().weight - 1
                ),
                Error::<Test>::TooSmallWeightBound
            );
        })
}

#[test]
fn close_fails_if_proposal_does_not_exists() {
    ExtBuilder::default().build().execute_with(|| {
        let proposal_id =
            Organizations::proposal_id(&Organizations::org_id_for(0), make_proposal());

        assert_noop!(
            Organizations::close_proposal(RawOrigin::Signed(ALICE).into(), proposal_id, 0),
            Error::<Test>::ProposalNotFound
        );
    })
}

#[test]
fn close_fails_if_hook_fails() {
    ExtBuilder::default().build().execute_with(|| {
        let org_id = Organizations::org_id_for(0);
        let proposal = make_proposal();
        let proposal_id = Organizations::proposal_id(&org_id, proposal.clone());

        // We have to insert a fake proposal and bypass the hook for the `None` voting system
        Proposals::<Test>::insert(
            &proposal_id,
            Proposal {
                org: org_id,
                call: make_proposal().encode(),
                voting: MockVotingSystemId::FailClose,
            },
        );

        assert_noop!(
            Organizations::close_proposal(
                RawOrigin::Signed(ALICE).into(),
                proposal_id,
                proposal.get_dispatch_info().weight
            ),
            "fail"
        );
    })
}
