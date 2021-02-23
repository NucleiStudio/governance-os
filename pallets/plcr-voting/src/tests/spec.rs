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

use super::mock::{mock_parameters, mock_vote, ExtBuilder, PlcrVoting, Test, Tokens};
use crate::{Error, Proposals};
use frame_support::{assert_noop, assert_ok, StorageMap};
use governance_os_support::{
    testing::{primitives::AccountId, ALICE, TEST_TOKEN_ID},
    traits::{LockableCurrencies, StandardizedVoting},
};
use sp_core::H256;

#[test]
fn initialize_registers_proposal_hash() {
    ExtBuilder::default().build().execute_with(|| {
        let mock_hash = H256::default();

        // Make sure it does not exists by default
        assert!(!Proposals::<Test>::contains_key(mock_hash));

        assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
            mock_hash,
            Default::default()
        ));
        assert!(Proposals::<Test>::contains_key(mock_hash));
    })
}

#[test]
fn initialize_refuses_duplicate() {
    ExtBuilder::default().build().execute_with(|| {
        let mock_hash = H256::default();

        assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
            mock_hash,
            Default::default()
        ));
        assert_noop!(
            <PlcrVoting as StandardizedVoting>::initiate(mock_hash, Default::default()),
            Error::<Test>::DuplicatedProposal
        );
    })
}

#[test]
fn vote_normal_flow() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_parameters()
            ));

            let (commit, reveal) = mock_vote(50, 10, true, 42);
            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, commit,
            ));
            // Locked the decoy
            assert_eq!(
                <Tokens as LockableCurrencies<AccountId>>::locked_balance(TEST_TOKEN_ID, &ALICE),
                50
            );
            assert_eq!(
                PlcrVoting::locks((TEST_TOKEN_ID, ALICE)),
                vec![(mock_hash, 50)]
            );
            assert_eq!(PlcrVoting::votes(ALICE, mock_hash), commit);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, reveal
            ));
            assert_eq!(PlcrVoting::votes(ALICE, mock_hash), reveal);

            assert_eq!(PlcrVoting::proposals(mock_hash).revealed_against, 0);
            assert_eq!(PlcrVoting::proposals(mock_hash).revealed_favorable, 10);

            // Freed the decoy balance
            assert_eq!(
                PlcrVoting::locks((TEST_TOKEN_ID, ALICE)),
                vec![(mock_hash, 10)]
            );
            assert_eq!(
                <Tokens as LockableCurrencies<AccountId>>::locked_balance(TEST_TOKEN_ID, &ALICE),
                10
            );
        })
}

#[test]
fn vote_cannot_reveal_early() {
    unimplemented!()
}

#[test]
fn vote_cannot_commit_late() {
    unimplemented!()
}

#[test]
fn vote_cannot_reveal_uncommitted() {
    unimplemented!()
}

#[test]
fn vote_multiple_proposals_pick_highest_lock() {
    unimplemented!()
}

#[test]
fn vote_update_commit_change_locks() {
    unimplemented!()
}
