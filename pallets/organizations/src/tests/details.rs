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

use super::mock::Tokens;
use crate::OrganizationDetails;
use governance_os_support::testing::{
    primitives::{AccountId, Balance, BlockNumber, CurrencyId},
    ALICE, BOB,
};
use governance_os_voting::VotingSystems;

#[test]
fn sort() {
    let mut details = OrganizationDetails {
        executors: vec![BOB, ALICE],
        voting: VotingSystems::<Balance, CurrencyId, BlockNumber, Tokens, AccountId>::None,
    };
    details.sort();
    assert_eq!(details.executors, vec![ALICE, BOB]);
}
