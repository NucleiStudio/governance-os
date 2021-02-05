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

use super::mock::*;
use crate::{
    types::{VoteData, VotingParameters},
    Error, Proposals,
};
use frame_support::{assert_noop, assert_ok, StorageMap};
use governance_os_support::{
    testing::{primitives::AccountId, ALICE, BOB, TEST_TOKEN_ID},
    traits::{LockableCurrencies, StandardizedVoting},
};
use sp_core::H256;

#[test]
fn initialize_registers_proposal_hash() {
    ExtBuilder::default().build().execute_with(|| {
        let mock_hash = H256::default();

        // Make sure it does not exists by default
        assert!(!Proposals::<Test>::contains_key(mock_hash));

        assert_ok!(<CoinVoting as StandardizedVoting>::initiate(
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

        assert_ok!(<CoinVoting as StandardizedVoting>::initiate(
            mock_hash,
            Default::default()
        ));
        assert_noop!(
            <CoinVoting as StandardizedVoting>::initiate(mock_hash, Default::default()),
            Error::<Test>::DuplicatedProposal
        );
    })
}

#[test]
fn vote_lock_tokens() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<CoinVoting as StandardizedVoting>::initiate(
                mock_hash,
                VotingParameters {
                    voting_currency: TEST_TOKEN_ID,
                }
            ));

            assert_ok!(<CoinVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                VoteData {
                    in_support: true,
                    power: 10
                }
            ));

            assert_eq!(
                <Tokens as LockableCurrencies<AccountId>>::locked_balance(TEST_TOKEN_ID, &ALICE),
                10
            );
        })
}

#[test]
fn vote_edit_previous_vote() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<CoinVoting as StandardizedVoting>::initiate(
                mock_hash,
                VotingParameters {
                    voting_currency: TEST_TOKEN_ID,
                }
            ));

            assert_ok!(<CoinVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                VoteData {
                    in_support: true,
                    power: 10
                }
            ));
            assert_ok!(<CoinVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                VoteData {
                    in_support: false,
                    power: 15
                }
            ));

            assert_eq!(
                CoinVoting::votes((TEST_TOKEN_ID, &ALICE)),
                vec![(mock_hash, false, 15)]
            );
            assert_eq!(CoinVoting::proposals(mock_hash).total_against, 15);
            assert_eq!(
                <Tokens as LockableCurrencies<AccountId>>::locked_balance(TEST_TOKEN_ID, &ALICE),
                15
            );
        })
}

#[test]
fn votes_saved_correctly() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<CoinVoting as StandardizedVoting>::initiate(
                mock_hash,
                VotingParameters {
                    voting_currency: TEST_TOKEN_ID,
                }
            ));

            assert_ok!(<CoinVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                VoteData {
                    in_support: true,
                    power: 10
                }
            ));
            assert_ok!(<CoinVoting as StandardizedVoting>::vote(
                mock_hash,
                &BOB,
                VoteData {
                    in_support: false,
                    power: 15
                }
            ));

            assert_eq!(CoinVoting::proposals(mock_hash).total_favorable, 10);
            assert_eq!(CoinVoting::proposals(mock_hash).total_against, 15);

            assert_eq!(
                CoinVoting::votes((TEST_TOKEN_ID, &ALICE)),
                vec![(mock_hash, true, 10)]
            );
            assert_eq!(
                CoinVoting::votes((TEST_TOKEN_ID, &BOB)),
                vec![(mock_hash, false, 15)]
            );
        })
}

#[test]
fn vote_other_proposals_extend_locks() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mut mock_hash_1 = H256::default();
            let mut mock_hash_2 = H256::default();

            mock_hash_1.randomize();
            mock_hash_2.randomize();

            assert!(mock_hash_1 != mock_hash_2);

            assert_ok!(<CoinVoting as StandardizedVoting>::initiate(
                mock_hash_1,
                VotingParameters {
                    voting_currency: TEST_TOKEN_ID,
                }
            ));
            assert_ok!(<CoinVoting as StandardizedVoting>::vote(
                mock_hash_1,
                &ALICE,
                VoteData {
                    in_support: true,
                    power: 10
                }
            ));

            assert_ok!(<CoinVoting as StandardizedVoting>::initiate(
                mock_hash_2,
                VotingParameters {
                    voting_currency: TEST_TOKEN_ID,
                }
            ));
            assert_ok!(<CoinVoting as StandardizedVoting>::vote(
                mock_hash_2,
                &ALICE,
                VoteData {
                    in_support: true,
                    power: 11
                }
            ));

            // Locked the max of both
            assert_eq!(
                <Tokens as LockableCurrencies<AccountId>>::locked_balance(TEST_TOKEN_ID, &ALICE),
                11
            );
        })
}

#[test]
fn vote_fail_if_not_enough_tokens() {
    ExtBuilder::default().build().execute_with(|| {
        let mock_hash = H256::default();

        assert_ok!(<CoinVoting as StandardizedVoting>::initiate(
            mock_hash,
            VotingParameters {
                voting_currency: TEST_TOKEN_ID,
            }
        ));

        assert_noop!(
            <CoinVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                VoteData {
                    in_support: true,
                    power: 10
                }
            ),
            Error::<Test>::NotEnoughBalance
        );
    })
}

#[test]
fn vote_fail_if_proposal_does_not_exists() {
    ExtBuilder::default().build().execute_with(|| {
        let mock_hash = H256::default();

        assert_noop!(
            <CoinVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                VoteData {
                    in_support: true,
                    power: 10
                }
            ),
            Error::<Test>::ProposalNotInitialized
        );
    })
}
