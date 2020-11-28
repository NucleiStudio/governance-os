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
    assert_noop, assert_ok,
    traits::{
        BalanceStatus, Currency, ExistenceRequirement, Imbalance, ReservableCurrency,
        SignedImbalance, WithdrawReason,
    },
};
use governance_os_support::{
    testing::{ALICE, BOB, TEST_TOKEN_ID},
    Currencies, ReservableCurrencies,
};

#[test]
fn total_issuance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_eq!(
                Tokens::total_issuance(TEST_TOKEN_ID),
                TokensCurrencyAdapter::total_issuance()
            );
        })
}

#[test]
fn balances() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_eq!(
                Tokens::free_balance(TEST_TOKEN_ID, &ALICE),
                TokensCurrencyAdapter::total_balance(&ALICE)
            );

            assert_eq!(
                Tokens::free_balance(TEST_TOKEN_ID, &ALICE),
                TokensCurrencyAdapter::free_balance(&ALICE)
            );

            assert_eq!(TokensCurrencyAdapter::minimum_balance(), 0);
        })
}

#[test]
fn make_free_balance_be_positive_imbalence() -> Result<(), String> {
    ExtBuilder::default().build().execute_with(|| {
        match TokensCurrencyAdapter::make_free_balance_be(&ALICE, 100) {
            SignedImbalance::Positive(imbalance) => assert_eq!(imbalance.peek(), 100),
            _ => return Err("SignedImbalance is not positive".into()),
        };

        assert_eq!(TokensCurrencyAdapter::free_balance(&ALICE), 100);
        Ok(())
    })
}

#[test]
fn make_free_balance_be_negative_imbalance() -> Result<(), String> {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            match TokensCurrencyAdapter::make_free_balance_be(&ALICE, 45) {
                SignedImbalance::Negative(imbalance) => assert_eq!(imbalance.peek(), 55),
                _ => return Err("SignedImbalance is not negative".into()),
            };

            assert_eq!(TokensCurrencyAdapter::free_balance(&ALICE), 45);
            Ok(())
        })
}

#[test]
fn issue() {
    ExtBuilder::default().build().execute_with(|| {
        let imbalance = TokensCurrencyAdapter::issue(100);
        assert_eq!(imbalance.peek(), 100);
        assert_eq!(TokensCurrencyAdapter::total_issuance(), 100);

        // Dropping the imbalance basically revoke it
        drop(imbalance);
        assert_eq!(TokensCurrencyAdapter::total_issuance(), 0);
    })
}

#[test]
fn burn() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let imbalance = TokensCurrencyAdapter::burn(100);
            assert_eq!(imbalance.peek(), 100);
            assert_eq!(TokensCurrencyAdapter::total_issuance(), 100); // Reduced the supply
            assert_eq!(TokensCurrencyAdapter::free_balance(&ALICE), 100); // But didn't touch the balances
            assert_eq!(TokensCurrencyAdapter::free_balance(&BOB), 100);

            // Dropping the imbalance basically revoke it
            drop(imbalance);
            assert_eq!(TokensCurrencyAdapter::total_issuance(), 200);
        })
}

// Was lazy and didn't want to type two times the same test so we just make
// a macro to do the job for us
macro_rules! deposit_test {
    ($test_name:ident, $deposit_fn:ident) => {
        #[test]
        fn $test_name() {
            ExtBuilder::default().build().execute_with(|| {
                let imbalance = TokensCurrencyAdapter::$deposit_fn(&ALICE, 100).unwrap();
                assert_eq!(imbalance.peek(), 100);
                assert_eq!(TokensCurrencyAdapter::free_balance(&ALICE), 100);
                assert_eq!(TokensCurrencyAdapter::total_issuance(), 100);
            })
        }
    };
}

deposit_test!(deposit_into_existing, deposit_into_existing);
deposit_test!(deposit_into_creating, deposit_into_existing);

#[test]
fn transfer() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(TokensCurrencyAdapter::transfer(
                &ALICE,
                &BOB,
                50,
                ExistenceRequirement::KeepAlive
            ));
            assert_eq!(TokensCurrencyAdapter::free_balance(&ALICE), 50);
            assert_eq!(TokensCurrencyAdapter::free_balance(&BOB), 150);
        })
}

#[test]
fn withdraw() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let imbalance = TokensCurrencyAdapter::withdraw(
                &ALICE,
                45,
                WithdrawReason::Transfer.into(),
                ExistenceRequirement::KeepAlive,
            )
            .unwrap();

            assert_eq!(imbalance.peek(), 45);
            assert_eq!(TokensCurrencyAdapter::free_balance(&ALICE), 55);

            // Didn't touch the total issuance
            assert_eq!(TokensCurrencyAdapter::total_issuance(), 200);
        })
}

#[test]
fn ensure_can_withdraw() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(TokensCurrencyAdapter::ensure_can_withdraw(
                &ALICE,
                10,
                WithdrawReason::Transfer.into(),
                0
            ));
            assert_noop!(
                TokensCurrencyAdapter::ensure_can_withdraw(
                    &ALICE,
                    101,
                    WithdrawReason::Transfer.into(),
                    0
                ),
                Error::<Test>::BalanceTooLow
            );
        })
}

#[test]
fn can_slash() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_eq!(TokensCurrencyAdapter::can_slash(&ALICE, 1), true);
            assert_eq!(TokensCurrencyAdapter::can_slash(&ALICE, 101), false);
        })
}

#[test]
fn slash() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let alice_slash = TokensCurrencyAdapter::slash(&ALICE, 50);
            assert_eq!(alice_slash.0.peek(), 50);
            assert_eq!(alice_slash.1, 0);
            assert_eq!(TokensCurrencyAdapter::free_balance(&ALICE), 50);

            let bob_slash = TokensCurrencyAdapter::slash(&BOB, 150);
            assert_eq!(bob_slash.0.peek(), 100);
            assert_eq!(bob_slash.1, 50);
            assert_eq!(TokensCurrencyAdapter::free_balance(&BOB), 0);

            // No issuance changes until the imbalances are consumed
            assert_eq!(TokensCurrencyAdapter::total_issuance(), 200);
        })
}

#[test]
fn total_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_eq!(TokensCurrencyAdapter::total_balance(&ALICE), 100);
            assert_ok!(Tokens::reserve(TEST_TOKEN_ID, &ALICE, 100));
            assert_eq!(TokensCurrencyAdapter::free_balance(&ALICE), 0);
            assert_eq!(TokensCurrencyAdapter::total_balance(&ALICE), 100);
        })
}

#[test]
fn reserve_fails_if_not_enough_funds() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            TokensCurrencyAdapter::reserve(&ALICE, 100),
            Error::<Test>::BalanceTooLow
        );
    })
}

#[test]
fn reserve() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_eq!(TokensCurrencyAdapter::can_reserve(&ALICE, 55), true);
            assert_ok!(TokensCurrencyAdapter::reserve(&ALICE, 55));
            assert_eq!(TokensCurrencyAdapter::can_reserve(&ALICE, 55), false);
            assert_eq!(TokensCurrencyAdapter::can_reserve(&ALICE, 45), true);
            assert_eq!(TokensCurrencyAdapter::free_balance(&ALICE), 45);
            assert_eq!(TokensCurrencyAdapter::reserved_balance(&ALICE), 55);
            assert_eq!(TokensCurrencyAdapter::total_balance(&ALICE), 100);
        })
}

#[test]
fn unreserve() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(TokensCurrencyAdapter::reserve(&ALICE, 100));
            assert_eq!(TokensCurrencyAdapter::unreserve(&ALICE, 55), 0);
            assert_eq!(TokensCurrencyAdapter::reserved_balance(&ALICE), 45);
            // Returns the amount of extra coins that couldn't be unreserved since they were not even reserved
            assert_eq!(TokensCurrencyAdapter::unreserve(&ALICE, 55), 10);
            assert_eq!(TokensCurrencyAdapter::reserved_balance(&ALICE), 0);

            assert_eq!(TokensCurrencyAdapter::free_balance(&ALICE), 100);
            assert_eq!(TokensCurrencyAdapter::total_balance(&ALICE), 100);
        })
}

#[test]
fn repatriate_reserved_slashed_is_beneficiary_free_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(TokensCurrencyAdapter::reserve(&ALICE, 50));

            // Basically will unreserve the stuff
            assert_eq!(
                TokensCurrencyAdapter::repatriate_reserved(
                    &ALICE,
                    &ALICE,
                    30,
                    BalanceStatus::Free,
                ),
                Ok(0)
            );
            assert_eq!(TokensCurrencyAdapter::reserved_balance(&ALICE), 20);
            assert_eq!(
                TokensCurrencyAdapter::repatriate_reserved(
                    &ALICE,
                    &ALICE,
                    30,
                    BalanceStatus::Free,
                ),
                Ok(10)
            );
            assert_eq!(TokensCurrencyAdapter::reserved_balance(&ALICE), 0);
        })
}

#[test]
fn repatriate_reserved_slashed_is_beneficiary_reserved_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(TokensCurrencyAdapter::reserve(&ALICE, 50));

            // Doesn't really do anything but return slashed - reserved_balance
            assert_eq!(
                TokensCurrencyAdapter::repatriate_reserved(
                    &ALICE,
                    &ALICE,
                    30,
                    BalanceStatus::Reserved,
                ),
                Ok(0)
            );
            assert_eq!(TokensCurrencyAdapter::reserved_balance(&ALICE), 50);
            assert_eq!(
                TokensCurrencyAdapter::repatriate_reserved(
                    &ALICE,
                    &ALICE,
                    60,
                    BalanceStatus::Reserved,
                ),
                Ok(10)
            );
            assert_eq!(TokensCurrencyAdapter::reserved_balance(&ALICE), 50);
        })
}

#[test]
fn repatriate_reserved_free_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(TokensCurrencyAdapter::reserve(&ALICE, 50));

            // Basically will unreserve the stuff
            assert_eq!(
                TokensCurrencyAdapter::repatriate_reserved(&ALICE, &BOB, 30, BalanceStatus::Free,),
                Ok(0)
            );
            assert_eq!(TokensCurrencyAdapter::reserved_balance(&ALICE), 20);
            assert_eq!(TokensCurrencyAdapter::free_balance(&BOB), 130);
            assert_eq!(
                TokensCurrencyAdapter::repatriate_reserved(&ALICE, &BOB, 30, BalanceStatus::Free,),
                Ok(10)
            );
            assert_eq!(TokensCurrencyAdapter::reserved_balance(&ALICE), 0);
            assert_eq!(TokensCurrencyAdapter::free_balance(&BOB), 150);
        })
}

#[test]
fn repatriate_reserved_reserved_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(TokensCurrencyAdapter::reserve(&ALICE, 50));

            // Basically will unreserve the stuff
            assert_eq!(
                TokensCurrencyAdapter::repatriate_reserved(
                    &ALICE,
                    &BOB,
                    30,
                    BalanceStatus::Reserved,
                ),
                Ok(0)
            );
            assert_eq!(TokensCurrencyAdapter::reserved_balance(&ALICE), 20);
            assert_eq!(TokensCurrencyAdapter::reserved_balance(&BOB), 30);
            assert_eq!(
                TokensCurrencyAdapter::repatriate_reserved(
                    &ALICE,
                    &BOB,
                    30,
                    BalanceStatus::Reserved,
                ),
                Ok(10)
            );
            assert_eq!(TokensCurrencyAdapter::reserved_balance(&ALICE), 0);
            assert_eq!(TokensCurrencyAdapter::reserved_balance(&BOB), 50);
        })
}

#[test]
fn slash_reserved() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(TokensCurrencyAdapter::reserve(&ALICE, 25));

            let (imbalance, unslashed) = TokensCurrencyAdapter::slash_reserved(&ALICE, 50);
            assert_eq!(imbalance.peek(), 25);
            assert_eq!(unslashed, 25);
        })
}
