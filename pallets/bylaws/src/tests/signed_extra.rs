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
use frame_support::{assert_noop, assert_ok, weights::GetDispatchInfo};
use governance_os_support::testing::ALICE;
use sp_runtime::{traits::SignedExtension, transaction_validity::InvalidTransaction};

#[test]
fn refuse_if_no_role_granted() {
    ExtBuilder::default().build().execute_with(|| {
        let call = Call::System(frame_system::Call::remark(vec![]));
        assert_noop!(
            MockCheckRole::default().validate(&ALICE, &call, &call.get_dispatch_info(), 0),
            InvalidTransaction::Call
        );
    })
}

#[test]
fn allow_if_no_roles_attached_to_call() {
    ExtBuilder::default().build().execute_with(|| {
        let call = Call::System(frame_system::Call::suicide());
        assert_ok!(MockCheckRole::default().validate(&ALICE, &call, &call.get_dispatch_info(), 0));
    })
}

#[test]
fn allow_if_role_granted_to_account() {
    ExtBuilder::default()
        .with_role(MockRoles::RemarkOnly, Some(ALICE))
        .build()
        .execute_with(|| {
            let call = Call::System(frame_system::Call::remark(vec![]));
            assert_ok!(MockCheckRole::default().validate(
                &ALICE,
                &call,
                &call.get_dispatch_info(),
                0
            ));
        })
}

#[test]
fn allow_if_role_granted_everybody() {
    ExtBuilder::default()
        .with_role(MockRoles::RemarkOnly, None)
        .build()
        .execute_with(|| {
            let call = Call::System(frame_system::Call::remark(vec![]));
            assert_ok!(MockCheckRole::default().validate(
                &ALICE,
                &call,
                &call.get_dispatch_info(),
                0
            ));
        })
}

#[test]
fn allow_if_root() {
    ExtBuilder::default()
        .alice_as_root()
        .build()
        .execute_with(|| {
            let call = Call::System(frame_system::Call::remark(vec![]));
            assert_ok!(MockCheckRole::default().validate(
                &ALICE,
                &call,
                &call.get_dispatch_info(),
                0
            ));
        })
}
