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

use crate::{
    Bylaw::{self, *},
    Call,
};
use frame_support::weights::GetDispatchInfo;
use governance_os_primitives::AccountId;
use governance_os_support::rules::Rule;
use sp_runtime::{traits::AccountIdConversion, ModuleId};

macro_rules! test_bylaw {
    ($test_name:ident, $variant:expr, $result:expr) => {
        #[test]
        fn $test_name() {
            let alice: AccountId = ModuleId(*b"tt/bylaw").into_account();
            let bylaw: Bylaw = $variant;
            let call = Call::System(frame_system::Call::remark(vec![]));
            assert_eq!(
                bylaw.validate(&alice, &call, &call.get_dispatch_info(), 0),
                $result
            );
        }
    };
}

macro_rules! bylaw_truth {
    ($variant:ident, $operand:tt, $left_op:expr, $right_op:expr, $left_result:expr, $right_result:expr) => {
        let alice: AccountId = ModuleId(*b"tt/bylaw").into_account();
        let bylaw: Bylaw = $variant($left_op, $right_op);
        let call = Call::System(frame_system::Call::remark(vec![]));
        assert_eq!(
            bylaw.validate(&alice, &call, &call.get_dispatch_info(), 0),
            $left_result $operand $right_result
        );
    };
}

macro_rules! test_bylaw_truth_table {
    ($test_name:ident, $variant:ident, $operand:tt) => {
        #[test]
        fn $test_name() {
            bylaw_truth!(
                $variant,
                $operand,
                Box::new(Allow),
                Box::new(Allow),
                true,
                true
            );
            bylaw_truth!(
                $variant,
                $operand,
                Box::new(Allow),
                Box::new(Deny),
                true,
                false
            );
            bylaw_truth!(
                $variant,
                $operand,
                Box::new(Deny),
                Box::new(Allow),
                false,
                true
            );
            bylaw_truth!(
                $variant,
                $operand,
                Box::new(Deny),
                Box::new(Deny),
                false,
                false
            );
        }
    };
}

test_bylaw!(allow, Allow, true);
test_bylaw!(deny, Deny, false);
test_bylaw_truth_table!(and, And, &);
test_bylaw_truth_table!(or, Or, |);
test_bylaw_truth_table!(xor, Xor, ^);
test_bylaw!(not_true, Not(Box::new(Allow)), false);
test_bylaw!(not_false, Not(Box::new(Deny)), true);
