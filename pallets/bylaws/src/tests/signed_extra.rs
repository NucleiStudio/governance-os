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
fn default_bylaw_if_no_bylaw_registered() {
    ExtBuilder::default().build().execute_with(|| {
        let allowed_call = Call::System(frame_system::Call::remark(vec![]));
        let denied_call = Call::System(frame_system::Call::suicide());

        // By default only `remark` is allowed
        assert_ok!(MockCheckBylaws::default().validate(
            &ALICE,
            &allowed_call,
            &allowed_call.get_dispatch_info(),
            0
        ));
        assert_noop!(
            MockCheckBylaws::default().validate(
                &ALICE,
                &denied_call,
                &denied_call.get_dispatch_info(),
                0
            ),
            InvalidTransaction::Call
        );
    })
}

#[test]
fn apply_bylaw_as_expected() {
    // We are going to make the system deny calls to `remark`
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Bylaws::add_bylaw(
            Origin::signed(ALICE),
            ALICE,
            MockTags::Test,
            Bylaw::Deny
        ));
        let call = Call::System(frame_system::Call::remark(vec![]));

        assert_eq!(
            MockCheckBylaws::default()
                .validate(&ALICE, &call, &call.get_dispatch_info(), 0)
                .is_err(),
            true
        );
    })
}
