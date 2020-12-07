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

use super::mock::{ExtBuilder, MockVotingSystems, Tokens};
use crate::{CoinBasedVotingParameters, ProposalMetadata, VotingErrors};
use frame_support::assert_ok;
use governance_os_support::{
    testing::{
        primitives::{AccountId, Balance, BlockNumber, CurrencyId},
        ALICE, BOB, CHARLIE, TEST_TOKEN_ID,
    },
    traits::{Currencies, ReservableCurrencies, VotingHooks},
};

fn mock_system() -> MockVotingSystems {
    MockVotingSystems::CoinBased(
        CoinBasedVotingParameters::<Balance, CurrencyId, BlockNumber> {
            voting_currency: TEST_TOKEN_ID,
            creation_fee: 10,
            min_quorum: 50,        // 50% min approval
            min_participation: 20, // 20% participation rate
            ttl: 10,
        },
    )
}

#[test]
fn on_create_proposal_returns_correct_metadata_and_reserve_fee() {
    ExtBuilder::default()
        .hundred_for_alice()
        .build()
        .execute_with(|| {
            let (res, data) = MockVotingSystems::on_create_proposal(mock_system(), &ALICE, 0);

            assert_eq!(res, Ok(()));

            assert_eq!(data.creator, ALICE);
            assert_eq!(data.favorable, 10);
            assert_eq!(data.against, 0);
            assert_eq!(data.expiry, 10);
            assert_eq!(data.votes.len(), 1);
            assert!(data.votes.contains_key(&ALICE));
            assert_eq!(data.votes.get(&ALICE), Some(&(10, true)));

            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 10);
        })
}

#[test]
fn on_veto_proposal_free_all_funds() {
    ExtBuilder::default().build().execute_with(|| {
        // Create a specific structure with a few votes
        let mut metadata: ProposalMetadata<AccountId, Balance, BlockNumber> = Default::default();
        metadata.votes.insert(ALICE, (10, true));
        metadata.votes.insert(BOB, (5, false));
        metadata.votes.insert(CHARLIE, (10, true));
        metadata
            .votes
            .iter()
            .for_each(|(account, (val, _support))| {
                assert_ok!(<Tokens as Currencies<AccountId>>::mint(
                    TEST_TOKEN_ID,
                    account,
                    *val
                ));
                assert_ok!(Tokens::reserve(TEST_TOKEN_ID, account, *val));
            });

        assert_ok!(MockVotingSystems::on_veto_proposal(
            mock_system(),
            metadata.clone()
        ));

        metadata
            .votes
            .iter()
            .for_each(|(account, (val, _support))| {
                assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, account), 0);
                assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, account), *val);
            });
    })
}

macro_rules! test_on_decide_on_proposal_simple {
    ($test_name:ident, $support:tt) => {
        #[test]
        fn $test_name() {
            ExtBuilder::default()
                .hundred_for_alice()
                .build()
                .execute_with(|| {
                    let (res, data) = MockVotingSystems::on_decide_on_proposal(
                        mock_system(),
                        Default::default(),
                        &ALICE,
                        50,
                        $support,
                    );

                    assert_ok!(res);
                    assert_eq!(data.votes.get(&ALICE), Some(&(50, $support)));
                })
        }
    };
}

macro_rules! test_on_decide_on_proposal_updates {
    ($test_name:ident, $old_support:tt, $old_weight:tt, $new_support:tt, $new_weight:tt) => {
        #[test]
        fn $test_name() {
            ExtBuilder::default()
                .hundred_for_alice()
                .build()
                .execute_with(|| {
                    let (res, data) = MockVotingSystems::on_decide_on_proposal(
                        mock_system(),
                        Default::default(),
                        &ALICE,
                        $old_weight,
                        $old_support,
                    );

                    assert_ok!(res);
                    assert_eq!(data.votes.get(&ALICE), Some(&($old_weight, $old_support)));

                    let (res, data) = MockVotingSystems::on_decide_on_proposal(
                        mock_system(),
                        data,
                        &ALICE,
                        $new_weight,
                        $new_support,
                    );

                    assert_ok!(res);
                    assert_eq!(data.votes.get(&ALICE), Some(&($new_weight, $new_support)));
                })
        }
    };
}

test_on_decide_on_proposal_simple!(test_on_decide_on_proposal_favorable, true);
test_on_decide_on_proposal_simple!(test_on_decide_on_proposal_against, false);

test_on_decide_on_proposal_updates!(
    test_on_decide_on_proposal_increase_against,
    false,
    10,
    false,
    20
);
test_on_decide_on_proposal_updates!(
    test_on_decide_on_proposal_decrease_against,
    false,
    20,
    false,
    10
);
test_on_decide_on_proposal_updates!(
    test_on_decide_on_proposal_increase_favorable,
    true,
    10,
    true,
    20
);
test_on_decide_on_proposal_updates!(
    test_on_decide_on_proposal_decrease_favorable,
    true,
    20,
    true,
    10
);
test_on_decide_on_proposal_updates!(
    test_on_decide_on_proposal_change_against_to_favorable,
    false,
    10,
    true,
    20
);
test_on_decide_on_proposal_updates!(
    test_on_decide_on_proposal_change_favorable_to_against,
    true,
    10,
    false,
    20
);

#[test]
fn on_decide_on_proposal_creator_cannot_free_creation_fee_in_advance() {
    ExtBuilder::default()
        .hundred_for_alice()
        .build()
        .execute_with(|| {
            let mut metadata: ProposalMetadata<AccountId, Balance, BlockNumber> =
                Default::default();
            metadata.votes.insert(ALICE, (10, true));
            metadata.creator = ALICE;
            assert_ok!(Tokens::reserve(TEST_TOKEN_ID, &ALICE, 10));

            let (res, data) = MockVotingSystems::on_decide_on_proposal(
                mock_system(),
                metadata.clone(),
                &ALICE,
                5,
                true,
            );
            assert_eq!(res, Err(VotingErrors::UnderCreationFee.into()));
            assert_eq!(data, metadata);
        })
}

#[test]
fn passing_if_quorum_and_participation_criteria_met() {
    ExtBuilder::default()
        .hundred_for_alice()
        .build()
        .execute_with(|| {
            assert!(MockVotingSystems::passing(
                mock_system(),
                ProposalMetadata {
                    favorable: 50,
                    against: 10,
                    ..Default::default()
                }
            ))
        })
}

#[test]
fn not_passing_if_quorum_is_not_met() {
    ExtBuilder::default()
        .hundred_for_alice()
        .build()
        .execute_with(|| {
            // Note how the total number of votes is 21. This allows us to test cases where
            // the Perbill call is rounding things up or down.
            assert!(!MockVotingSystems::passing(
                mock_system(),
                ProposalMetadata {
                    favorable: 10,
                    against: 11,
                    ..Default::default()
                }
            ))
        })
}

#[test]
fn not_passing_if_participation_not_met() {
    ExtBuilder::default()
        .hundred_for_alice()
        .build()
        .execute_with(|| {
            // Min participation is 20 tokens, we have 11
            assert!(!MockVotingSystems::passing(
                mock_system(),
                ProposalMetadata {
                    favorable: 6,
                    against: 5,
                    ..Default::default()
                }
            ))
        })
}

#[test]
fn can_close_if_passing() {
    ExtBuilder::default()
        .hundred_for_alice()
        .build()
        .execute_with(|| {
            assert!(MockVotingSystems::can_close(
                mock_system(),
                ProposalMetadata {
                    favorable: 50,
                    against: 10,
                    expiry: 1, // Proposal not yet expired
                    ..Default::default()
                },
                0
            ))
        })
}

#[test]
fn can_close_if_failed_but_ttl_expired() {
    ExtBuilder::default()
        .hundred_for_alice()
        .build()
        .execute_with(|| {
            assert!(MockVotingSystems::can_close(
                mock_system(),
                ProposalMetadata {
                    favorable: 10,
                    against: 50,
                    expiry: 1,
                    ..Default::default()
                },
                10 // Late enough so that proposal expire
            ))
        })
}

#[test]
fn can_not_close_if_failing_but_not_ttl_expired() {
    ExtBuilder::default()
        .hundred_for_alice()
        .build()
        .execute_with(|| {
            assert!(!MockVotingSystems::can_close(
                mock_system(),
                ProposalMetadata {
                    favorable: 10,
                    against: 50,
                    expiry: 1,
                    ..Default::default()
                },
                0 // Too eearly for expiry
            ))
        })
}

#[test]
fn on_close_proposal_free_all_funds() {
    ExtBuilder::default().build().execute_with(|| {
        // Create a specific structure with a few votes
        let mut metadata: ProposalMetadata<AccountId, Balance, BlockNumber> = Default::default();
        metadata.votes.insert(ALICE, (10, true));
        metadata.votes.insert(BOB, (5, false));
        metadata.votes.insert(CHARLIE, (10, true));
        metadata
            .votes
            .iter()
            .for_each(|(account, (val, _support))| {
                assert_ok!(<Tokens as Currencies<AccountId>>::mint(
                    TEST_TOKEN_ID,
                    account,
                    *val
                ));
                assert_ok!(Tokens::reserve(TEST_TOKEN_ID, account, *val));
            });

        assert_ok!(MockVotingSystems::on_close_proposal(
            mock_system(),
            metadata.clone(),
            false
        ));

        metadata
            .votes
            .iter()
            .for_each(|(account, (val, _support))| {
                assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, account), 0);
                assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, account), *val);
            });
    })
}
