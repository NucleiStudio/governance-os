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
use governance_os_support::{
    testing::{primitives::AccountId, ALICE, BOB, TEST_TOKEN_ID, TEST_TOKEN_OWNER},
    traits::{Currencies, LockableCurrencies, ReservableCurrencies},
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
fn transfer_to_self_is_noop() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as Currencies<AccountId>>::transfer(
                TEST_TOKEN_ID,
                &ALICE,
                &ALICE,
                50
            ));
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 100);
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &BOB), 100);
            assert_eq!(Tokens::total_issuance(TEST_TOKEN_ID), 200);
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

#[test]
fn ensure_can_withdraw_refuse_if_non_transferable_currency() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Tokens::create(Origin::signed(TEST_TOKEN_OWNER), 42, false));
        assert_noop!(
            <Tokens as Currencies<AccountId>>::transfer(42, &ALICE, &BOB, 50),
            Error::<Test>::UnTransferableCurrency
        );
    })
}

#[test]
fn creating_a_new_account_inc_system_ref() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(<Tokens as Currencies<AccountId>>::mint(
            TEST_TOKEN_ID,
            &ALICE,
            100
        ));
        assert_ok!(<Tokens as Currencies<AccountId>>::transfer(
            TEST_TOKEN_ID,
            &ALICE,
            &BOB,
            50
        ));

        assert_eq!(frame_system::Module::<Test>::refs(&ALICE), 1);
        assert_eq!(frame_system::Module::<Test>::refs(&BOB), 1);
    });
}

#[test]
fn deleting_an_account_dec_system_ref() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(<Tokens as Currencies<AccountId>>::mint(
            TEST_TOKEN_ID,
            &ALICE,
            100
        ));
        assert_ok!(<Tokens as Currencies<AccountId>>::transfer(
            TEST_TOKEN_ID,
            &ALICE,
            &BOB,
            50
        ));

        assert_ok!(<Tokens as Currencies<AccountId>>::transfer(
            TEST_TOKEN_ID,
            &BOB,
            &ALICE,
            50
        ));
        assert_ok!(<Tokens as Currencies<AccountId>>::burn(
            TEST_TOKEN_ID,
            &ALICE,
            100
        ));

        assert_eq!(frame_system::Module::<Test>::refs(&ALICE), 0);
        assert_eq!(frame_system::Module::<Test>::refs(&BOB), 0);
    });
}

#[test]
fn set_lock_effectively_freeze_part_of_the_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                30
            ));
            assert_eq!(
                Tokens::get_currency_account(TEST_TOKEN_ID, &ALICE).frozen,
                30
            );
            assert_noop!(
                <Tokens as Currencies<AccountId>>::transfer(TEST_TOKEN_ID, &ALICE, &BOB, 100),
                Error::<Test>::BalanceLockTriggered
            );
        })
}

#[test]
fn set_lock_with_same_id_overwrite_existing_lock() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            // Initial setup
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                30
            ));
            assert_eq!(
                Tokens::get_currency_account(TEST_TOKEN_ID, &ALICE).frozen,
                30
            );

            // Reduce lock
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                20
            ));
            assert_eq!(
                Tokens::get_currency_account(TEST_TOKEN_ID, &ALICE).frozen,
                20
            );

            // Increase lock
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                40
            ));
            assert_eq!(
                Tokens::get_currency_account(TEST_TOKEN_ID, &ALICE).frozen,
                40
            );
        })
}

#[test]
fn can_lock_more_than_free_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                120
            ));
            assert_eq!(
                Tokens::get_currency_account(TEST_TOKEN_ID, &ALICE).frozen,
                120
            );

            // We locked more than free balance and thus can't transfer anything
            assert_noop!(
                <Tokens as Currencies<AccountId>>::transfer(TEST_TOKEN_ID, &ALICE, &BOB, 1),
                Error::<Test>::BalanceLockTriggered
            );
        })
}

#[test]
fn set_lock_with_different_id_extend_frozen_balance_if_needed() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                30
            ));
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"deadbeef",
                &ALICE,
                30
            ));
            // No lock increase
            assert_eq!(
                Tokens::get_currency_account(TEST_TOKEN_ID, &ALICE).frozen,
                30
            );

            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"deadbeef",
                &ALICE,
                50
            ));
            // Lock increase
            assert_eq!(
                Tokens::get_currency_account(TEST_TOKEN_ID, &ALICE).frozen,
                50
            );
        })
}

#[test]
fn extend_lock_increases_lock() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                30
            ));
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::extend_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                60
            ));

            assert_eq!(
                Tokens::get_currency_account(TEST_TOKEN_ID, &ALICE).frozen,
                60
            );
        })
}

#[test]
fn extend_lock_does_not_decrease_lock() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                30
            ));
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::extend_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                20
            ));

            assert_eq!(
                Tokens::get_currency_account(TEST_TOKEN_ID, &ALICE).frozen,
                30
            );
        })
}

#[test]
fn extend_lock_creates_new_lock_if_needed() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::extend_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                60
            ));

            assert_eq!(
                Tokens::get_currency_account(TEST_TOKEN_ID, &ALICE).frozen,
                60
            );
        })
}

#[test]
fn remove_lock_may_not_decrease_frozen_amount_if_other_and_higher_locks_are_in_place() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                60
            ));
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"deadbeef",
                &ALICE,
                100
            ));
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::remove_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE
            ));

            // Still has the deadbeef lock in place
            assert_eq!(
                Tokens::get_currency_account(TEST_TOKEN_ID, &ALICE).frozen,
                100
            );
        })
}

#[test]
fn remove_lock_clears_frozen_balance_when_removing_last_lock() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                60
            ));
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::remove_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE
            ));

            assert_eq!(
                Tokens::get_currency_account(TEST_TOKEN_ID, &ALICE).frozen,
                0
            );
        })
}

#[test]
fn remove_lock_clears_frozen_balance_when_removing_higher_lock() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                60
            ));
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"deadbeef",
                &ALICE,
                100
            ));
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::remove_lock(
                TEST_TOKEN_ID,
                *b"deadbeef",
                &ALICE
            ));

            // Still has the testtest lock in place
            assert_eq!(
                Tokens::get_currency_account(TEST_TOKEN_ID, &ALICE).frozen,
                60
            );
        })
}

#[test]
fn can_not_withdraw_locked_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                60
            ));

            assert_noop!(
                <Tokens as Currencies<AccountId>>::ensure_can_withdraw(TEST_TOKEN_ID, &ALICE, 50),
                Error::<Test>::BalanceLockTriggered
            );
        })
}

#[test]
fn can_not_reserve_locked_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                60
            ));

            assert_eq!(
                <Tokens as ReservableCurrencies<AccountId>>::can_reserve(TEST_TOKEN_ID, &ALICE, 50),
                false
            );
            assert_noop!(
                <Tokens as ReservableCurrencies<AccountId>>::reserve(TEST_TOKEN_ID, &ALICE, 50),
                Error::<Test>::BalanceLockTriggered
            );
        })
}

#[test]
fn can_not_transfer_locked_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                60
            ));

            assert_noop!(
                <Tokens as Currencies<AccountId>>::transfer(TEST_TOKEN_ID, &ALICE, &BOB, 50),
                Error::<Test>::BalanceLockTriggered
            );
        })
}

#[test]
fn can_not_burn_locked_balance() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                60
            ));

            assert_noop!(
                <Tokens as Currencies<AccountId>>::burn(TEST_TOKEN_ID, &ALICE, 50),
                Error::<Test>::BalanceLockTriggered
            );
        })
}

#[test]
fn set_new_lock_inc_ref() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                60
            ));

            assert_eq!(frame_system::Module::<Test>::refs(&ALICE), 2);
        })
}
#[test]
fn set_existing_lock_does_not_inc_ref() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                60
            ));
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                50
            ));

            assert_eq!(frame_system::Module::<Test>::refs(&ALICE), 2);
        })
}

#[test]
fn extend_new_lock_inc_ref() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::extend_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                60
            ));

            assert_eq!(frame_system::Module::<Test>::refs(&ALICE), 2);
        })
}

#[test]
fn extend_existing_lock_does_not_inc_ref() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                50
            ));
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::extend_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                60
            ));

            assert_eq!(frame_system::Module::<Test>::refs(&ALICE), 2);
        })
}

#[test]
fn remove_lock_dec_ref() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::set_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
                60
            ));
            assert_ok!(<Tokens as LockableCurrencies<AccountId>>::remove_lock(
                TEST_TOKEN_ID,
                *b"testtest",
                &ALICE,
            ));

            assert_eq!(frame_system::Module::<Test>::refs(&ALICE), 1);
        })
}
