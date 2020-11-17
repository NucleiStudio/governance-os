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
use crate::RoleBuilder;
use governance_os_support::{
    acl::RoleManager,
    testing::{ALICE, BOB, TEST_TOKEN_ID, TEST_TOKEN_OWNER},
    Currencies,
};

#[test]
fn set_storage_correctly() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 100);
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &BOB), 100);
            assert_eq!(Tokens::total_issuance(TEST_TOKEN_ID), 200);
        })
}

#[test]
fn does_not_set_balances_by_default() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 0);
        assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &BOB), 0);
        assert_eq!(Tokens::total_issuance(TEST_TOKEN_ID), 0);
    })
}

#[test]
fn set_test_token_roles_approprietaly() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(
            Bylaws::has_role(&TEST_TOKEN_OWNER, MockRoles::manage_currency(TEST_TOKEN_ID)),
            true
        );
        assert_eq!(
            Bylaws::has_role(&ALICE, MockRoles::transfer_currency(TEST_TOKEN_ID)),
            true
        );
    })
}
