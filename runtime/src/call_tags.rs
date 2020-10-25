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
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;

/// Determine a tag for every kind of call.
#[derive(Encode, Decode, RuntimeDebug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CallTags {
    Any,
    System,
    Economic,
    Bylaws,
}

impl governance_os_support::rules::SuperSetter for CallTags {
    fn is_superset(&self, other: &Self) -> bool {
        match (self, other) {
            (x, y) if x == y => true,
            (CallTags::Any, _) => true,
            _ => false,
        }
    }
}

pub struct CallTagger;
impl governance_os_support::rules::CallTagger<AccountId, Call, CallTags> for CallTagger {
    fn tag(&self, _who: &AccountId, call: &Call) -> CallTags {
        match call {
            Call::System(..)
            | Call::Timestamp(..)
            | Call::Grandpa(..)
            | Call::RandomnessCollectiveFlip(..) => CallTags::System,
            Call::Tokens(..) => CallTags::Economic,
            Call::Bylaws(..) => CallTags::Bylaws,
        }
    }
}
