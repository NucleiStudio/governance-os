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
use crate::{CurrencyDetails, Error, RoleBuilder};
use frame_support::{assert_noop, assert_ok};
use governance_os_support::{
    errors::AclError,
    testing::{ALICE, BOB, TEST_TOKEN_ID, TEST_TOKEN_OWNER},
    traits::{Currencies, RoleManager},
};

#[test]
fn create_transferrable_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Tokens::create(Origin::signed(TEST_TOKEN_OWNER), 42, true));
        assert_eq!(
            Bylaws::has_role(&TEST_TOKEN_OWNER, MockRoles::manage_currency(42)),
            true
        );
        assert_eq!(
            Bylaws::has_role(&ALICE, MockRoles::transfer_currency(42)),
            true
        );
    })
}

#[test]
fn create_non_transferable_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Tokens::create(Origin::signed(TEST_TOKEN_OWNER), 42, false));
        assert_eq!(
            Bylaws::has_role(&TEST_TOKEN_OWNER, MockRoles::manage_currency(42)),
            true
        );
        assert_eq!(
            Bylaws::has_role(&ALICE, MockRoles::transfer_currency(42)),
            false
        );
    })
}

#[test]
fn create_duplicate_currency_id_fails() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Tokens::create(Origin::signed(TEST_TOKEN_OWNER), TEST_TOKEN_ID, true),
            Error::<Test>::CurrencyAlreadyExists
        );
    })
}

#[test]
fn mint_fails_if_not_owner() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Tokens::mint(Origin::signed(42), TEST_TOKEN_ID, ALICE, 100),
            AclError::MissingRole
        );
    })
}

#[test]
fn mint_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Tokens::mint(
            Origin::signed(TEST_TOKEN_OWNER),
            TEST_TOKEN_ID,
            ALICE,
            100
        ));
        assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 100);
    })
}

#[test]
fn burn_fails_if_not_owner() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_noop!(
                Tokens::burn(Origin::signed(42), TEST_TOKEN_ID, ALICE, 100),
                AclError::MissingRole
            );
        })
}

#[test]
fn burn_works() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Tokens::burn(
                Origin::signed(TEST_TOKEN_OWNER),
                TEST_TOKEN_ID,
                ALICE,
                100
            ));
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 0); // Burned the balance
        })
}

#[test]
fn update_details_fails_if_not_owner() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_noop!(
                Tokens::update_details(
                    Origin::signed(42),
                    TEST_TOKEN_ID,
                    CurrencyDetails {
                        owner: ALICE,
                        transferable: true
                    }
                ),
                AclError::MissingRole
            );
        })
}

#[test]
fn update_details_works() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Tokens::update_details(
                Origin::signed(TEST_TOKEN_OWNER),
                TEST_TOKEN_ID,
                CurrencyDetails {
                    owner: ALICE,
                    transferable: false
                }
            ));
            assert_eq!(
                Bylaws::has_role(&TEST_TOKEN_OWNER, MockRoles::manage_currency(TEST_TOKEN_ID)),
                false
            );
            assert_eq!(
                Bylaws::has_role(&ALICE, MockRoles::manage_currency(TEST_TOKEN_ID)),
                true
            );
            assert_eq!(
                Bylaws::has_role(&ALICE, MockRoles::transfer_currency(TEST_TOKEN_ID)),
                false
            );
        })
}

#[test]
fn transfer_works() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(Tokens::transfer(
                Origin::signed(ALICE),
                TEST_TOKEN_ID,
                BOB,
                50
            ));
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 50);
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &BOB), 150);
        })
}
