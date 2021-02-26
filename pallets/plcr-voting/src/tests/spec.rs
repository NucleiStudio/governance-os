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

use super::mock::{
    advance_blocks, mock_parameters, mock_vote, ExtBuilder, PlcrVoting, Test, Tokens,
};
use crate::{Error, Locks, Proposals, Votes};
use frame_support::{assert_noop, assert_ok, IterableStorageDoubleMap, StorageMap};
use governance_os_support::{
    testing::{primitives::AccountId, ALICE, BOB, TEST_TOKEN_ID},
    traits::{LockableCurrencies, ProposalResult, StandardizedVoting},
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

            let (commit, reveal) = mock_vote(10, true, 42);
            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, commit,
            ));
            advance_blocks(mock_parameters().commit_duration + 1);

            assert_eq!(PlcrVoting::votes(mock_hash, ALICE), commit);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, reveal
            ));
            assert_eq!(PlcrVoting::votes(mock_hash, ALICE), reveal);

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
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_parameters()
            ));

            let (_commit, reveal) = mock_vote(10, true, 42);

            assert_noop!(
                <PlcrVoting as StandardizedVoting>::vote(mock_hash, &ALICE, reveal),
                Error::<Test>::Phase
            );
        })
}

#[test]
fn vote_cannot_commit_late() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_parameters()
            ));

            let (commit, _reveal) = mock_vote(10, true, 42);

            advance_blocks(mock_parameters().commit_duration + 1);
            assert_noop!(
                <PlcrVoting as StandardizedVoting>::vote(mock_hash, &ALICE, commit,),
                Error::<Test>::Phase
            );
        })
}

#[test]
fn vote_cannot_reveal_uncommitted() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_parameters()
            ));

            let (_commit, reveal) = mock_vote(10, true, 42);

            advance_blocks(mock_parameters().commit_duration + 1);
            assert_noop!(
                <PlcrVoting as StandardizedVoting>::vote(mock_hash, &ALICE, reveal),
                Error::<Test>::RevealCommitMismatch
            );
        })
}

#[test]
fn cannot_reveal_twice() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_parameters()
            ));

            let (commit, reveal) = mock_vote(10, true, 42);
            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, commit,
            ));
            advance_blocks(mock_parameters().commit_duration + 1);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, reveal
            ));

            assert_noop!(
                <PlcrVoting as StandardizedVoting>::vote(mock_hash, &ALICE, reveal),
                Error::<Test>::NoCommitFound
            );
        })
}

#[test]
fn vote_cannot_commit_after_reveal() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_parameters()
            ));

            let (commit, reveal) = mock_vote(10, true, 42);
            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, commit,
            ));
            advance_blocks(mock_parameters().commit_duration + 1);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, reveal
            ));

            assert_noop!(
                <PlcrVoting as StandardizedVoting>::vote(mock_hash, &ALICE, commit),
                Error::<Test>::Revealed
            );
        })
}

#[test]
fn vote_cannot_reveal_wrong_hash() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_parameters()
            ));

            // Salts are different
            let (commit, _reveal) = mock_vote(10, true, 42);
            let (_commit, reveal) = mock_vote(10, true, 43);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, commit,
            ));
            advance_blocks(mock_parameters().commit_duration + 1);

            assert_noop!(
                <PlcrVoting as StandardizedVoting>::vote(mock_hash, &ALICE, reveal),
                Error::<Test>::RevealCommitMismatch
            );
        })
}

#[test]
fn vote_update_commit_change_locks() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_parameters()
            ));

            let (initial_commit, _reveal) = mock_vote(10, true, 42);
            let (commit, _reveal) = mock_vote(15, false, 42);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                initial_commit,
            ));
            assert_eq!(PlcrVoting::votes(mock_hash, ALICE), initial_commit);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, commit,
            ));
            assert_eq!(PlcrVoting::votes(mock_hash, ALICE), commit);
        })
}

#[test]
fn vote_multiple_locks_pick_highest_lock() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mut mock_hash_1 = H256::default();
            let mut mock_hash_2 = H256::default();

            mock_hash_1.randomize();
            mock_hash_2.randomize();

            assert_ok!(PlcrVoting::lock(mock_hash_1, TEST_TOKEN_ID, &ALICE, 100));
            assert_ok!(PlcrVoting::lock(mock_hash_2, TEST_TOKEN_ID, &ALICE, 120));

            assert_eq!(
                <Tokens as LockableCurrencies<AccountId>>::locked_balance(TEST_TOKEN_ID, &ALICE),
                120
            );

            assert_ok!(PlcrVoting::unlock(mock_hash_2, TEST_TOKEN_ID, &ALICE));

            assert_eq!(
                <Tokens as LockableCurrencies<AccountId>>::locked_balance(TEST_TOKEN_ID, &ALICE),
                100
            );
        })
}

macro_rules! test_close_or_veto {
    ($function:tt, $t1:ident, $t2:ident, $t3:ident) => {
        #[test]
        fn $t1() {
            ExtBuilder::default()
                .one_hundred_for_alice_n_bob()
                .build()
                .execute_with(|| {
                    let mock_hash = H256::default();

                    assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                        mock_hash,
                        mock_parameters()
                    ));

                    let (alice_commit, alice_reveal) = mock_vote(10, true, 42);
                    let (bob_commit, bob_reveal) = mock_vote(15, false, 42);

                    assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                        mock_hash,
                        &ALICE,
                        alice_commit,
                    ));
                    assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                        mock_hash, &BOB, bob_commit,
                    ));

                    advance_blocks(mock_parameters().commit_duration + 1);

                    assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                        mock_hash,
                        &ALICE,
                        alice_reveal
                    ));
                    assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                        mock_hash, &BOB, bob_reveal
                    ));

                    advance_blocks(mock_parameters().reveal_duration + 1);

                    assert_ok!(PlcrVoting::$function(mock_hash));

                    assert_eq!(
                        <Tokens as LockableCurrencies<AccountId>>::locked_balance(
                            TEST_TOKEN_ID,
                            &ALICE
                        ),
                        0
                    );
                    assert_eq!(
                        <Tokens as LockableCurrencies<AccountId>>::locked_balance(
                            TEST_TOKEN_ID,
                            &BOB
                        ),
                        0
                    );
                })
        }

        #[test]
        fn $t2() {
            ExtBuilder::default()
                .one_hundred_for_alice_n_bob()
                .build()
                .execute_with(|| {
                    let mock_hash = H256::default();

                    assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                        mock_hash,
                        mock_parameters()
                    ));

                    let (alice_commit, alice_reveal) = mock_vote(10, true, 42);

                    assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                        mock_hash,
                        &ALICE,
                        alice_commit,
                    ));

                    advance_blocks(mock_parameters().commit_duration + 1);

                    assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                        mock_hash,
                        &ALICE,
                        alice_reveal
                    ));

                    advance_blocks(mock_parameters().reveal_duration + 1);

                    assert_ok!(PlcrVoting::$function(mock_hash));

                    // Empty!
                    assert_eq!(Votes::<Test>::iter_prefix(mock_hash).next(), None);
                    assert!(!Locks::<Test>::contains_key((TEST_TOKEN_ID, &ALICE)));
                    assert!(!Proposals::<Test>::contains_key(mock_hash));
                    assert_eq!(
                        <Tokens as LockableCurrencies<AccountId>>::locked_balance(
                            TEST_TOKEN_ID,
                            &ALICE
                        ),
                        0
                    );
                })
        }

        #[test]
        fn $t3() {
            ExtBuilder::default()
                .one_hundred_for_alice_n_bob()
                .build()
                .execute_with(|| {
                    let mut mock_hash_1 = H256::default();
                    let mut mock_hash_2 = H256::default();

                    mock_hash_1.randomize();
                    mock_hash_2.randomize();

                    assert!(mock_hash_1 != mock_hash_2);

                    assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                        mock_hash_1,
                        mock_parameters()
                    ));
                    assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                        mock_hash_2,
                        mock_parameters()
                    ));

                    let (commit_1, reveal_1) = mock_vote(15, true, 42);
                    let (commit_2, reveal_2) = mock_vote(10, true, 42);

                    assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                        mock_hash_1,
                        &ALICE,
                        commit_1,
                    ));
                    assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                        mock_hash_2,
                        &ALICE,
                        commit_2,
                    ));

                    advance_blocks(mock_parameters().commit_duration + 1);

                    assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                        mock_hash_1,
                        &ALICE,
                        reveal_1
                    ));
                    assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                        mock_hash_2,
                        &ALICE,
                        reveal_2
                    ));

                    assert_eq!(
                        <Tokens as LockableCurrencies<AccountId>>::locked_balance(
                            TEST_TOKEN_ID,
                            &ALICE
                        ),
                        15
                    );

                    advance_blocks(mock_parameters().reveal_duration + 1);
                    assert_ok!(PlcrVoting::$function(mock_hash_1));

                    // Empty!
                    assert_eq!(Votes::<Test>::iter_prefix(mock_hash_1).next(), None);
                    assert_eq!(
                        PlcrVoting::locks((TEST_TOKEN_ID, &ALICE)),
                        vec![(mock_hash_2, 10)]
                    );
                    assert!(!Proposals::<Test>::contains_key(mock_hash_1));
                    assert_eq!(
                        <Tokens as LockableCurrencies<AccountId>>::locked_balance(
                            TEST_TOKEN_ID,
                            &ALICE
                        ),
                        10
                    );
                })
        }
    };
}

test_close_or_veto!(
    veto,
    veto_unlocks_coins,
    veto_free_storage_if_last_proposal,
    veto_cleans_storage
);

test_close_or_veto!(
    close,
    close_unlocks_coins,
    close_free_storage_if_last_proposal,
    close_cleans_storage
);

#[test]
fn close_error_if_early() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_parameters()
            ));

            let (alice_commit, alice_reveal) = mock_vote(10, false, 42);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                alice_commit,
            ));

            advance_blocks(mock_parameters().commit_duration + 1);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                alice_reveal
            ));

            assert_noop!(PlcrVoting::close(mock_hash), Error::<Test>::TooEarly);
        })
}

#[test]
fn close_passing() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_parameters()
            ));

            let (commit, reveal) = mock_vote(90, true, 42);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, commit,
            ));

            advance_blocks(mock_parameters().commit_duration + 1);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, reveal
            ));

            advance_blocks(mock_parameters().reveal_duration + 1);

            assert_eq!(PlcrVoting::close(mock_hash), Ok(ProposalResult::Passing));
        })
}

#[test]
fn close_failing() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_parameters()
            ));

            let (commit, reveal) = mock_vote(90, false, 42);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, commit,
            ));

            advance_blocks(mock_parameters().commit_duration + 1);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, reveal
            ));

            advance_blocks(mock_parameters().reveal_duration + 1);

            assert_eq!(PlcrVoting::close(mock_hash), Ok(ProposalResult::Failing));
        })
}

#[test]
fn close_passing_early() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<PlcrVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_parameters()
            ));

            let (commit, reveal) = mock_vote(90, true, 42);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, commit,
            ));

            advance_blocks(mock_parameters().commit_duration + 1);

            assert_ok!(<PlcrVoting as StandardizedVoting>::vote(
                mock_hash, &ALICE, reveal
            ));

            assert_eq!(PlcrVoting::close(mock_hash), Ok(ProposalResult::Passing));
        })
}
