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

use crate::Trait;
use governance_os_support::rules::Rule;
use sp_runtime::traits::DispatchInfoOf;

/// We use the enum to create a Domain Specific Language used to decide
/// wether an extrinsic should be accepted or not.
pub enum Bylaw<T: Trait> {
    /// Allows the extrinsic to go through, no other questions asked.
    Allow,
    /// Refuse the extrinsic.
    Deny,

    Mock(T::AccountId),
}

impl<T: Trait> Rule for Bylaw<T> {
    type AccountId = T::AccountId;
    type Call = T::Call;

    fn validate(
        &self,
        _who: &Self::AccountId,
        _call: &Self::Call,
        _info: &DispatchInfoOf<Self::Call>,
        _len: usize,
    ) -> bool {
        use Bylaw::*;

        match self {
            Allow => true,
            Deny => false,
            _ => todo!(),
        }
    }
}
