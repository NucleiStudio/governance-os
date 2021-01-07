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
use crate::{mutations::Mutation, Error};
use frame_support::{assert_noop, assert_ok};
use governance_os_support::testing::{ALICE, TEST_TOKEN_ID};

#[test]
fn sub_up_to_free_balance_honors_frozen() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mut mutation = Mutation::<Test>::new_for_currency(TEST_TOKEN_ID);
            assert_ok!(mutation.add_frozen(&ALICE, 40));
            assert_eq!(mutation.sub_up_to_free_balance(&ALICE, 100), 60);
        })
}

#[test]
fn sub_free_balance_honors_frozen() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let mut mutation = Mutation::<Test>::new_for_currency(TEST_TOKEN_ID);
            assert_ok!(mutation.add_frozen(&ALICE, 40));
            assert_noop!(
                mutation.sub_free_balance(&ALICE, 100),
                Error::<Test>::BalanceLockTriggered
            );
        })
}
