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
use crate::Error;
use frame_support::{
    traits::BalanceStatus,
    {assert_noop, assert_ok},
};

#[test]
fn transfer_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as Currencies<AccountId>>::transfer(
                TEST_TOKEN_ID,
                &ALICE,
                &BOB,
                50
            ));
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 50);
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &BOB), 150);
        })
}

#[test]
fn mint_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(<Tokens as Currencies<AccountId>>::mint(
            TEST_TOKEN_ID,
            &ALICE,
            100
        ));
        assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 100);
    })
}

#[test]
fn burn_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as Currencies<AccountId>>::burn(
                TEST_TOKEN_ID,
                &ALICE,
                10
            ));
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 90);
        })
}

#[test]
fn ensure_can_withdraw_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Tokens::ensure_can_withdraw(TEST_TOKEN_ID, &ALICE, 50));
            assert_noop!(
                Tokens::ensure_can_withdraw(TEST_TOKEN_ID, &ALICE, 150),
                Error::<Test>::BalanceTooLow
            );
        })
}

#[test]
fn total_balance_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_eq!(Tokens::total_balance(TEST_TOKEN_ID, &ALICE), 100);
            assert_ok!(Tokens::reserve(TEST_TOKEN_ID, &ALICE, 100));
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 0);
            assert_eq!(Tokens::total_balance(TEST_TOKEN_ID, &ALICE), 100);
        })
}

#[test]
fn reserve_fails_if_not_enough_funds() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Tokens::reserve(TEST_TOKEN_ID, &ALICE, 100),
            Error::<Test>::BalanceTooLow
        );
    })
}

#[test]
fn reserve_works() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_eq!(Tokens::can_reserve(TEST_TOKEN_ID, &ALICE, 55), true);
            assert_ok!(Tokens::reserve(TEST_TOKEN_ID, &ALICE, 55));
            assert_eq!(Tokens::can_reserve(TEST_TOKEN_ID, &ALICE, 55), false);
            assert_eq!(Tokens::can_reserve(TEST_TOKEN_ID, &ALICE, 45), true);
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 45);
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 55);
            assert_eq!(Tokens::total_balance(TEST_TOKEN_ID, &ALICE), 100);
        })
}

#[test]
fn unreserve_works() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Tokens::reserve(TEST_TOKEN_ID, &ALICE, 100));
            assert_eq!(Tokens::unreserve(TEST_TOKEN_ID, &ALICE, 55), 0);
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 45);
            // Returns the amount of extra coins that couldn't be unreserved since they were not even reserved
            assert_eq!(Tokens::unreserve(TEST_TOKEN_ID, &ALICE, 55), 10);
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 0);

            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 100);
            assert_eq!(Tokens::total_balance(TEST_TOKEN_ID, &ALICE), 100);
        })
}

#[test]
fn slash_reserved_works() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Tokens::reserve(TEST_TOKEN_ID, &ALICE, 100));
            assert_eq!(Tokens::slash_reserved(TEST_TOKEN_ID, &ALICE, 55), 0);
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 45);
            // Returns the amount of extra coins that couldn't be unreserved since they were not even reserved
            assert_eq!(Tokens::slash_reserved(TEST_TOKEN_ID, &ALICE, 55), 10);
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 0);

            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 0);
            assert_eq!(Tokens::total_balance(TEST_TOKEN_ID, &ALICE), 0);
        })
}

#[test]
fn repatriate_reserved_slashed_is_beneficiary_free_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Tokens::reserve(TEST_TOKEN_ID, &ALICE, 50));

            // Basically will unreserve the stuff
            assert_eq!(Tokens::repatriate_reserved(
                TEST_TOKEN_ID,
                &ALICE,
                &ALICE,
                30,
                BalanceStatus::Free,
            ), Ok(0));
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 20);
            assert_eq!(Tokens::repatriate_reserved(
                TEST_TOKEN_ID,
                &ALICE,
                &ALICE,
                30,
                BalanceStatus::Free,
            ), Ok(10));
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 0);
        })
}

#[test]
fn repatriate_reserved_slashed_is_beneficiary_reserved_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Tokens::reserve(TEST_TOKEN_ID, &ALICE, 50));

            // Doesn't really do anything but return slashed - reserved_balance
            assert_eq!(
                Tokens::repatriate_reserved(
                    TEST_TOKEN_ID,
                    &ALICE,
                    &ALICE,
                    30,
                    BalanceStatus::Reserved,
                ),
                Ok(0)
            );
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 50);
            assert_eq!(
                Tokens::repatriate_reserved(
                    TEST_TOKEN_ID,
                    &ALICE,
                    &ALICE,
                    60,
                    BalanceStatus::Reserved,
                ),
                Ok(10)
            );
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 50);
        })
}

#[test]
fn repatriate_reserved_free_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Tokens::reserve(TEST_TOKEN_ID, &ALICE, 50));

            // Basically will unreserve the stuff
            assert_eq!(
                Tokens::repatriate_reserved(TEST_TOKEN_ID, &ALICE, &BOB, 30, BalanceStatus::Free,),
                Ok(0)
            );
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 20);
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &BOB), 130);
            assert_eq!(
                Tokens::repatriate_reserved(TEST_TOKEN_ID, &ALICE, &BOB, 30, BalanceStatus::Free,),
                Ok(10)
            );
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 0);
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &BOB), 150);
        })
}

#[test]
fn repatriate_reserved_reserved_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Tokens::reserve(TEST_TOKEN_ID, &ALICE, 50));

            // Basically will unreserve the stuff
            assert_eq!(
                Tokens::repatriate_reserved(
                    TEST_TOKEN_ID,
                    &ALICE,
                    &BOB,
                    30,
                    BalanceStatus::Reserved,
                ),
                Ok(0)
            );
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 20);
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &BOB), 30);
            assert_eq!(
                Tokens::repatriate_reserved(
                    TEST_TOKEN_ID,
                    &ALICE,
                    &BOB,
                    30,
                    BalanceStatus::Reserved,
                ),
                Ok(10)
            );
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &ALICE), 0);
            assert_eq!(Tokens::reserved_balance(TEST_TOKEN_ID, &BOB), 50);
        })
}
