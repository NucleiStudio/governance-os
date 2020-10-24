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
use crate::Bylaw::{self, *};
use frame_support::weights::GetDispatchInfo;
use governance_os_support::rules::Rule;
use governance_os_support::testing::ALICE;

macro_rules! test_bylaw {
    ($test_name:ident, $variant:expr, $result:expr) => {
        #[test]
        fn $test_name() {
            let bylaw: Bylaw<Test> = $variant;
            let call = Call::System(frame_system::Call::remark(vec![]));
            assert_eq!(
                bylaw.validate(&ALICE, &call, &call.get_dispatch_info(), 0),
                $result
            );
        }
    };
}

test_bylaw!(deny, Deny, false);
test_bylaw!(allow, Allow, true);
