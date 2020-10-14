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

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn transfer_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Tokens::transfer(TEST_TOKEN_ID, &ALICE, &BOB, 50));
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 50);
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &BOB), 150);
        })
}

#[test]
fn mint_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Tokens::mint(TEST_TOKEN_ID, &ALICE, 100));
        assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 100);
    })
}

#[test]
fn burn_should_work() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Tokens::burn(TEST_TOKEN_ID, &ALICE, 10));
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
