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
use crate::Roles;
use frame_support::storage::StorageDoubleMap;
use governance_os_support::testing::{primitives::AccountId, ALICE, BOB};

#[test]
fn register_role() {
    ExtBuilder::default()
        .with_role(MockRoles::RemarkOnly, None)
        .with_role(MockRoles::Root, Some(ALICE))
        .build()
        .execute_with(|| {
            assert_eq!(
                Roles::<Test>::get(MockRoles::RemarkOnly, None as Option<AccountId>),
                true
            );
            assert_eq!(Roles::<Test>::get(MockRoles::Root, Some(ALICE)), true);
            assert_eq!(Bylaws::has_role(&ALICE, MockRoles::Root), true);
            assert_eq!(Bylaws::has_role(&BOB, MockRoles::Root), false);
            assert_eq!(Bylaws::has_role(&ALICE, MockRoles::RemarkOnly), true);
            assert_eq!(Bylaws::has_role(&BOB, MockRoles::RemarkOnly), true);
        })
}
