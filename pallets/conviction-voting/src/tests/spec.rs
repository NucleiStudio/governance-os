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
use crate::{Conviction, Error, Locks, Proposals};
use frame_support::{assert_noop, assert_ok, StorageMap};
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

        assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
            mock_hash,
            Default::default()
        ));
        assert!(Proposals::<Test>::contains_key(mock_hash));
    })
}

#[test]
fn initiate_logs_block_number() {
    ExtBuilder::default().build().execute_with(|| {
        let mock_hash = H256::default();

        assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
            mock_hash,
            Default::default()
        ));
        assert_eq!(
            ConvictionVoting::proposals(mock_hash).created_on,
            ConvictionVoting::now()
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

            assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_voting_parameters()
            ));

            assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                Conviction {
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

macro_rules! test_vote_edit {
    ($function:tt, $support:ident, $var:tt) => {
        #[test]
        fn $function() {
            ExtBuilder::default()
                .one_hundred_for_alice_n_bob()
                .build()
                .execute_with(|| {
                    let mock_hash = H256::default();

                    assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
                        mock_hash,
                        mock_voting_parameters()
                    ));

                    assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                        mock_hash,
                        &ALICE,
                        Conviction {
                            in_support: $support,
                            power: 10
                        }
                    ));
                    assert_eq!(ConvictionVoting::proposals(mock_hash).$var, 10);

                    assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                        mock_hash,
                        &ALICE,
                        Conviction {
                            in_support: $support,
                            power: 15
                        }
                    ));

                    assert_eq!(
                        ConvictionVoting::proposals(mock_hash).convictions,
                        vec![(
                            ALICE,
                            1,
                            Conviction {
                                in_support: $support,
                                power: 15
                            }
                        )]
                    );
                    assert_eq!(ConvictionVoting::proposals(mock_hash).$var, 15);

                    assert_eq!(
                        ConvictionVoting::locks((TEST_TOKEN_ID, &ALICE)),
                        vec![(mock_hash, $support, 15)]
                    );
                    assert_eq!(
                        <Tokens as LockableCurrencies<AccountId>>::locked_balance(
                            TEST_TOKEN_ID,
                            &ALICE
                        ),
                        15
                    );
                })
        }
    };
}

test_vote_edit!(vote_edit_favorable, true, conviction_for);
test_vote_edit!(vote_edit_against, false, conviction_against);

#[test]
fn vote_edit_change_our_mind() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_voting_parameters()
            ));

            assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                Conviction {
                    in_support: true,
                    power: 10
                }
            ));
            assert_eq!(ConvictionVoting::proposals(mock_hash).conviction_for, 10);
            assert_eq!(ConvictionVoting::proposals(mock_hash).conviction_against, 0);

            assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                Conviction {
                    in_support: false,
                    power: 15
                }
            ));
            assert_eq!(ConvictionVoting::proposals(mock_hash).conviction_for, 0);
            assert_eq!(
                ConvictionVoting::proposals(mock_hash).conviction_against,
                15
            );

            assert_eq!(
                ConvictionVoting::proposals(mock_hash).convictions,
                vec![(
                    ALICE,
                    1,
                    Conviction {
                        in_support: false,
                        power: 15
                    }
                )]
            );
            assert_eq!(
                ConvictionVoting::locks((TEST_TOKEN_ID, &ALICE)),
                vec![(mock_hash, false, 15)]
            );
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

            assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_voting_parameters()
            ));

            assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                Conviction {
                    in_support: true,
                    power: 10
                }
            ));
            assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                mock_hash,
                &BOB,
                Conviction {
                    in_support: false,
                    power: 15
                }
            ));

            assert_eq!(
                ConvictionVoting::locks((TEST_TOKEN_ID, &ALICE)),
                vec![(mock_hash, true, 10)]
            );
            assert_eq!(
                ConvictionVoting::locks((TEST_TOKEN_ID, &BOB)),
                vec![(mock_hash, false, 15)]
            );
            assert_eq!(
                ConvictionVoting::proposals(mock_hash).convictions,
                vec![
                    (
                        ALICE,
                        1,
                        Conviction {
                            in_support: true,
                            power: 10
                        }
                    ),
                    (
                        BOB,
                        1,
                        Conviction {
                            in_support: false,
                            power: 15
                        }
                    )
                ]
            );
            assert_eq!(ConvictionVoting::proposals(mock_hash).conviction_for, 10);
            assert_eq!(
                ConvictionVoting::proposals(mock_hash).conviction_against,
                15
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

            assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
                mock_hash_1,
                mock_voting_parameters()
            ));
            assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                mock_hash_1,
                &ALICE,
                Conviction {
                    in_support: true,
                    power: 10
                }
            ));

            assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
                mock_hash_2,
                mock_voting_parameters()
            ));
            assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                mock_hash_2,
                &ALICE,
                Conviction {
                    in_support: true,
                    power: 11
                }
            ));

            assert_eq!(
                ConvictionVoting::locks((TEST_TOKEN_ID, &ALICE)),
                vec![(mock_hash_1, true, 10), (mock_hash_2, true, 11)]
            );
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

        assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
            mock_hash,
            mock_voting_parameters()
        ));

        assert_noop!(
            <ConvictionVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                Conviction {
                    in_support: true,
                    power: 10
                }
            ),
            Error::<Test>::NotEnoughBalance
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

                    assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
                        mock_hash,
                        mock_voting_parameters()
                    ));

                    assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                        mock_hash,
                        &ALICE,
                        Conviction {
                            in_support: true,
                            power: 10
                        }
                    ));
                    assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                        mock_hash,
                        &BOB,
                        Conviction {
                            in_support: false,
                            power: 15
                        }
                    ));

                    System::set_block_number(ConvictionVoting::now() + 10_000);
                    assert_ok!(ConvictionVoting::$function(mock_hash));

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

                    assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
                        mock_hash,
                        mock_voting_parameters()
                    ));

                    assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                        mock_hash,
                        &ALICE,
                        Conviction {
                            in_support: true,
                            power: 10
                        }
                    ));

                    System::set_block_number(ConvictionVoting::now() + 10_000);
                    assert_ok!(ConvictionVoting::$function(mock_hash));

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

                    assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
                        mock_hash_1,
                        mock_voting_parameters()
                    ));
                    assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
                        mock_hash_2,
                        mock_voting_parameters()
                    ));

                    assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                        mock_hash_1,
                        &ALICE,
                        Conviction {
                            in_support: true,
                            power: 15
                        }
                    ));
                    assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                        mock_hash_2,
                        &ALICE,
                        Conviction {
                            in_support: true,
                            power: 10
                        }
                    ));

                    assert_eq!(
                        <Tokens as LockableCurrencies<AccountId>>::locked_balance(
                            TEST_TOKEN_ID,
                            &ALICE
                        ),
                        15
                    );

                    System::set_block_number(ConvictionVoting::now() + 10_000);
                    assert_ok!(ConvictionVoting::$function(mock_hash_1));

                    assert_eq!(
                        ConvictionVoting::locks((TEST_TOKEN_ID, &ALICE)),
                        vec![(mock_hash_2, true, 10)]
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

            assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_voting_parameters()
            ));

            assert_eq!(
                ConvictionVoting::close(mock_hash).expect_err("too early and not passing"),
                Error::<Test>::CannotClose.into()
            );

            // We did not advance blocks and particpation and quorum criteria are not met
            assert_noop!(
                ConvictionVoting::close(mock_hash),
                Error::<Test>::CannotClose
            );
        })
}

#[test]
fn close_failing() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_voting_parameters()
            ));

            System::set_block_number(ConvictionVoting::now() + 10_000);

            assert_eq!(
                ConvictionVoting::close(mock_hash).expect("proposal shall fail with no error"),
                ProposalResult::Failing
            );
        })
}

#[test]
fn close_passing_early() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mock_hash = H256::default();

            assert_ok!(<ConvictionVoting as StandardizedVoting>::initiate(
                mock_hash,
                mock_voting_parameters()
            ));

            assert_ok!(<ConvictionVoting as StandardizedVoting>::vote(
                mock_hash,
                &ALICE,
                Conviction {
                    in_support: true,
                    power: 99
                }
            ));

            // TTL is 1_000 so we are still early. We need a few blocks for
            // the conviction to accumulate.
            System::set_block_number(ConvictionVoting::now() + 500);

            assert_eq!(
                ConvictionVoting::close(mock_hash).expect("proposal shall pass"),
                ProposalResult::Passing
            );
        })
}
