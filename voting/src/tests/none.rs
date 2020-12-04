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

use super::mock::{ExtBuilder, MockVotingSystems};
use crate::{ProposalMetadata, VotingErrors};
use frame_support::assert_noop;
use governance_os_support::{
    testing::{
        primitives::{AccountId, Balance, BlockNumber},
        ALICE,
    },
    voting::VotingHooks,
};

#[test]
fn on_create_proposal_error_with_empty_metadata() {
    ExtBuilder::default().build().execute_with(|| {
        let (res, data) = MockVotingSystems::on_create_proposal(MockVotingSystems::None, &ALICE, 0);
        assert_eq!(res, Err(VotingErrors::NotAVotingSystem.into()));
        assert_eq!(data, Default::default());
    })
}

#[test]
fn on_veto_proposal_error() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            MockVotingSystems::on_veto_proposal(MockVotingSystems::None, Default::default()),
            VotingErrors::NotAVotingSystem
        );
    })
}

#[test]
fn on_decide_on_proposal_error_no_data_change() {
    ExtBuilder::default().build().execute_with(|| {
        let metadata: ProposalMetadata<AccountId, Balance, BlockNumber> = Default::default();
        let (res, data) = MockVotingSystems::on_decide_on_proposal(
            MockVotingSystems::None,
            metadata.clone(),
            &ALICE,
            0,
            true,
        );
        assert_eq!(data, metadata); // No changes
        assert_eq!(res, Err(VotingErrors::NotAVotingSystem.into()));
    })
}

#[test]
fn can_close_is_false() {
    ExtBuilder::default().build().execute_with(|| {
        assert!(!MockVotingSystems::can_close(
            MockVotingSystems::None,
            Default::default(),
            0,
        ));
    })
}

#[test]
fn passing_is_false() {
    ExtBuilder::default().build().execute_with(|| {
        assert!(!MockVotingSystems::passing(
            MockVotingSystems::None,
            Default::default()
        ));
    })
}

#[test]
fn on_close_proposal_error() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            MockVotingSystems::on_close_proposal(
                MockVotingSystems::None,
                Default::default(),
                false
            ),
            VotingErrors::NotAVotingSystem
        );
    })
}
