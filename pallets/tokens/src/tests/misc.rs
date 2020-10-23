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
use frame_support::{assert_ok, StorageMap};
use frame_system::Account;

#[test]
fn kill_currency_if_balance_down_to_zero() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .build()
        .execute_with(|| {
            assert_ok!(<Tokens as Currencies<AccountId>>::transfer(
                TEST_TOKEN_ID,
                &ALICE,
                &BOB,
                Tokens::free_balance(TEST_TOKEN_ID, &ALICE),
            ));

            // We can still view the balance
            assert_eq!(Tokens::free_balance(TEST_TOKEN_ID, &ALICE), 0);
            // Deleted the entry
            assert_eq!(
                <Account<Test>>::get(&ALICE)
                    .data
                    .contains_key(&TEST_TOKEN_ID),
                false
            );
        })
}
