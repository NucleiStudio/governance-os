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

use sp_runtime::traits::{DispatchInfoOf, Dispatchable};

/// A common trait that can be implemented to filter calls.
pub trait Rule<AccountId, Call>
where
    Call: Dispatchable,
{
    fn validate(
        &self,
        who: &AccountId,
        call: &Call,
        info: &DispatchInfoOf<Call>,
        len: usize,
    ) -> bool;
}

/// This trait can be used to "tag" incoming calls, this is typically
/// used to identify them and then match them with some set of rules.
pub trait CallTagger<AccountId, Call, Tag> {
    fn tag(&self, who: &AccountId, call: &Call) -> Tag;
}
