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

use super::mock::{Bylaws, Call, ExtBuilder, MockRoles, Organizations};
use crate::{OrganizationDetails, RoleBuilder};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use governance_os_support::{
    acl::{AclError, RoleManager},
    testing::{ALICE, BOB, CHARLIE, EVE},
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
                    managers: vec![EVE, ALICE],
                }
            ));
            assert_eq!(Organizations::counter(), 1);

            let org_id = Organizations::org_id_for(0);

            let params = Organizations::parameters(org_id);
            assert_eq!(params.executors.as_slice(), [BOB, CHARLIE]);
            assert_eq!(params.managers.as_slice(), [ALICE, EVE]);

            assert!(Bylaws::has_role(
                &CHARLIE,
                MockRoles::apply_as_organization(&org_id)
            ));
            assert!(Bylaws::has_role(
                &BOB,
                MockRoles::apply_as_organization(&org_id)
            ));
            assert!(Bylaws::has_role(
                &ALICE,
                MockRoles::manage_organization(&org_id)
            ));
            assert!(Bylaws::has_role(
                &EVE,
                MockRoles::manage_organization(&org_id)
            ));

            // Org ID is granted roles as well
            assert!(Bylaws::has_role(
                &org_id,
                MockRoles::apply_as_organization(&org_id)
            ));
            assert!(Bylaws::has_role(
                &org_id,
                MockRoles::manage_organization(&org_id)
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
                    managers: vec![],
                }
            ));
            assert_ok!(Organizations::apply_as(
                RawOrigin::Signed(ALICE).into(),
                Organizations::org_id_for(0),
                Box::new(Call::System(frame_system::Call::remark(vec![]))),
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
                    managers: vec![ALICE, BOB],
                }
            ));
            let org_id = Organizations::org_id_for(0);
            assert_ok!(Organizations::mutate(
                RawOrigin::Signed(ALICE).into(),
                org_id,
                OrganizationDetails {
                    executors: vec![ALICE, CHARLIE],
                    managers: vec![ALICE, CHARLIE]
                },
            ));

            let params = Organizations::parameters(org_id);
            assert_eq!(params.executors.as_slice(), [ALICE, CHARLIE]);
            assert_eq!(params.managers.as_slice(), [ALICE, CHARLIE]);

            // BOB lost permissions
            assert!(!Bylaws::has_role(
                &BOB,
                MockRoles::apply_as_organization(&org_id)
            ));
            assert!(!Bylaws::has_role(
                &BOB,
                MockRoles::manage_organization(&org_id)
            ));

            // ALICE kept them
            assert!(Bylaws::has_role(
                &ALICE,
                MockRoles::apply_as_organization(&org_id)
            ));
            assert!(Bylaws::has_role(
                &ALICE,
                MockRoles::manage_organization(&org_id)
            ));

            // CHARLIE won them
            assert!(Bylaws::has_role(
                &CHARLIE,
                MockRoles::apply_as_organization(&org_id)
            ));
            assert!(Bylaws::has_role(
                &CHARLIE,
                MockRoles::manage_organization(&org_id)
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
                    Box::new(Call::System(frame_system::Call::remark(vec![])))
                ),
                AclError::MissingRole
            );
        })
}

#[test]
fn mutate_fail_if_not_correct_role() {
    ExtBuilder::default()
        .with_default_orgs(1)
        .build()
        .execute_with(|| {
            assert_noop!(
                Organizations::mutate(
                    RawOrigin::Signed(ALICE).into(),
                    Organizations::org_id_for(0),
                    OrganizationDetails::default()
                ),
                AclError::MissingRole
            );
        })
}