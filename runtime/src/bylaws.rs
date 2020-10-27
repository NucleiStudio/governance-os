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

use crate::Call;
use codec::{Decode, Encode};
use governance_os_primitives::AccountId;
use governance_os_support::rules::Rule;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{traits::DispatchInfoOf, RuntimeDebug};
use sp_std::boxed::Box;

/// We use the enum to create a Domain Specific Language used to decide
/// wether an extrinsic should be accepted or not.
#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Bylaw {
    /// Allows the extrinsic to go through, no other questions asked.
    Allow,
    /// Refuse the extrinsic.
    Deny,
    /// Combine the parameters via an AND logic operation.
    And(Box<Bylaw>, Box<Bylaw>),
    /// Combine the parameters via an OR logic operation.
    Or(Box<Bylaw>, Box<Bylaw>),
    /// Combine the parameters via an XOR logic operation.
    Xor(Box<Bylaw>, Box<Bylaw>),
    /// Invert the result of its parameter.
    Not(Box<Bylaw>),
}

macro_rules! impl_combination {
    ($left:ident, $right: ident, $op:tt, $who:expr, $call:expr, $info:expr, $len:expr) => {
        $left.validate($who, $call, $info, $len) $op $right.validate($who, $call, $info, $len)
    };
}

impl Rule<AccountId, Call> for Bylaw {
    fn validate(
        &self,
        who: &AccountId,
        call: &Call,
        info: &DispatchInfoOf<Call>,
        len: usize,
    ) -> bool {
        match self {
            Self::Allow => true,
            Self::Deny => false,
            Self::And(left, right) => impl_combination!(left, right, &, who, call, info, len),
            Self::Or(left, right) => impl_combination!(left, right, |, who, call, info, len),
            Self::Xor(left, right) => impl_combination!(left, right, ^, who, call, info, len),
            Self::Not(bylaw) => !bylaw.validate(who, call, info, len),
        }
    }
}

impl Default for Bylaw {
    fn default() -> Self {
        Self::Allow
    }
}
