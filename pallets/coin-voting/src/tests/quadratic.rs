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
use crate::types::{VoteCountingStrategy, VoteData};
use frame_support::assert_ok;
use governance_os_support::{
    testing::{primitives::AccountId, ALICE, TEST_TOKEN_ID},
    traits::{LockableCurrencies, StandardizedVoting},
};
use sp_core::H256;

#[test]
fn registers_the_correct_amount_of_coins_locked() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<CoinVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_quadratic_voting_parameters()
            ));

            assert_ok!(<CoinVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                VoteData {
                    in_support: true,
                    // closest square root is 16
                    power: 17
                }
            ));

            assert_eq!(
                CoinVoting::locks((TEST_TOKEN_ID, ALICE)),
                vec![(mock_hash, true, 17, VoteCountingStrategy::Quadratic)]
            );
            assert_eq!(
                <Tokens as LockableCurrencies<AccountId>>::locked_balance(TEST_TOKEN_ID, &ALICE),
                17
            );
        })
}

#[test]
fn increase_counters_quadratically() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<CoinVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_quadratic_voting_parameters()
            ));

            assert_ok!(<CoinVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                VoteData {
                    in_support: true,
                    // closest square root is 16
                    power: 17
                }
            ));

            // Because 4 ** 2 = 16 and we locked 17 coins
            assert_eq!(CoinVoting::proposals(mock_hash).total_favorable, 4);
        })
}

#[test]
fn decrease_counters_correctly() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<CoinVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_quadratic_voting_parameters()
            ));

            assert_ok!(<CoinVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                VoteData {
                    in_support: true,
                    // closest square root is 16
                    power: 17
                }
            ));

            assert_ok!(<CoinVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                VoteData {
                    in_support: true,
                    power: 9
                }
            ));

            // Because 3 ** 2 = 9 and we locked 9 coins
            assert_eq!(CoinVoting::proposals(mock_hash).total_favorable, 3);
        })
}

#[test]
fn unlock_the_right_amount_of_coins() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<CoinVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_quadratic_voting_parameters()
            ));

            assert_ok!(<CoinVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                VoteData {
                    in_support: true,
                    // closest square root is 16
                    power: 17
                }
            ));

            // Bypass the choice between veto or close and directly call unlock
            assert_ok!(CoinVoting::unlock(vec![(TEST_TOKEN_ID, ALICE)], mock_hash));
            assert_eq!(
                <Tokens as LockableCurrencies<AccountId>>::locked_balance(TEST_TOKEN_ID, &ALICE),
                0
            );
        })
}

#[test]
fn locks_not_shared_between_proposals() {
    // The whole point of quadratic voting is to force people to carefully
    // allocate their votes between proposals. This is why we expect the
    // pallet to not share quadratic locks between proposals.

    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mut mock_hash_1 = H256::default();
            let mut mock_hash_2 = H256::default();

            mock_hash_1.randomize();
            mock_hash_2.randomize();

            assert_ok!(<CoinVoting as StandardizedVoting>::initiate(
                mock_hash_1,
                mock_quadratic_voting_parameters()
            ));
            assert_ok!(<CoinVoting as StandardizedVoting>::initiate(
                mock_hash_2,
                mock_quadratic_voting_parameters()
            ));

            assert_ok!(<CoinVoting as StandardizedVoting>::vote(
                mock_hash_1,
                &ALICE,
                VoteData {
                    in_support: true,
                    // closest square root is 16
                    power: 17
                }
            ));

            assert_ok!(<CoinVoting as StandardizedVoting>::vote(
                mock_hash_2,
                &ALICE,
                VoteData {
                    in_support: true,
                    power: 4
                }
            ));

            // Since quadratic locks are not shared we expect to have a lock
            // of 17 + 4 = 21
            assert_eq!(
                <Tokens as LockableCurrencies<AccountId>>::locked_balance(TEST_TOKEN_ID, &ALICE),
                21
            );
        })
}
