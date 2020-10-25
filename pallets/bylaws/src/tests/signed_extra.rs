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
use frame_support::weights::GetDispatchInfo;
use governance_os_support::testing::ALICE;
use sp_runtime::traits::SignedExtension;

#[test]
fn default_bylaw_if_no_bylaw_registered() {
    ExtBuilder::default().build().execute_with(|| {
        let call = Call::System(frame_system::Call::remark(vec![]));

        // `call` is not linked to any bylaw, by default we `Mock` uses `Deny`, thus we
        // expect an error
        assert_eq!(
            MockCheckBylaws::default()
                .validate(&ALICE, &call, &call.get_dispatch_info(), 0)
                .is_err(),
            true
        );
    })
}
