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

use super::mock::*;
use frame_support::assert_ok;
use frame_system::RawOrigin;
use governance_os_support::{acl::RoleManager, testing::ALICE};

#[test]
fn grant_role() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Bylaws::grant_role(
            RawOrigin::Root.into(),
            Some(ALICE),
            MockRoles::Root,
        ));
        assert_eq!(Bylaws::has_role(&ALICE, MockRoles::Root), true);
    })
}

#[test]
fn revoke_role() {
    ExtBuilder::default()
        .with_role(MockRoles::Root, Some(ALICE))
        .build()
        .execute_with(|| {
            assert_ok!(Bylaws::revoke_role(
                RawOrigin::Root.into(),
                Some(ALICE),
                MockRoles::Root,
            ));
            assert_eq!(Bylaws::has_role(&ALICE, MockRoles::Root), false);
        })
}
