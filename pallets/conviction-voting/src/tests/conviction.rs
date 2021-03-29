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

use super::mock::{mock_voting_parameters, Decay};
use crate::ProposalState;
use frame_support::assert_ok;
use governance_os_support::testing::primitives::{AccountId, Balance, BlockNumber, CurrencyId};

fn mock_state() -> ProposalState<AccountId, Balance, BlockNumber, CurrencyId> {
    ProposalState {
        parameters: mock_voting_parameters(),
        conviction_for: 1e18 as u128,
        conviction_against: 1e18 as u128,
        ..Default::default()
    }
}

#[test]
fn more_decay() {
    let mut state = mock_state();
    assert_ok!(state.mutate_conviction_snapshot(1_000, Decay::get()));

    // Saturates
    assert_eq!(state.snapshot.favorable, 1111111111111111111);
    assert_eq!(state.snapshot.against, 1111111111111111111);
}

#[test]
fn full_decay() {
    let mut state = mock_state();
    assert_ok!(state.mutate_conviction_snapshot(190, Decay::get()));

    assert_eq!(state.snapshot.favorable, 1111111111111111111);
    assert_eq!(state.snapshot.against, 1111111111111111111);
}

#[test]
fn partial_decay() {
    let mut state = mock_state();
    assert_ok!(state.mutate_conviction_snapshot(50, Decay::get()));

    assert_eq!(state.snapshot.favorable, 1111100000000000000);
    assert_eq!(state.snapshot.against, 1111100000000000000);
}

#[test]
fn no_decay() {
    let mut state = mock_state();
    assert_ok!(state.mutate_conviction_snapshot(0, Decay::get()));

    assert_eq!(state.snapshot.favorable, 0);
    assert_eq!(state.snapshot.against, 0);
}
