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

use crate::ProposalMetadata;
use governance_os_support::testing::primitives::{AccountId, Balance, BlockNumber};

#[test]
fn default_sets_everything_to_zero() {
    let data: ProposalMetadata<AccountId, Balance, BlockNumber> = Default::default();
    assert_eq!(data.against, 0);
    assert_eq!(data.favorable, 0);
    assert_eq!(data.votes.len(), 0);
}
