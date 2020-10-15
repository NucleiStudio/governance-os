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
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Tokens::create(Origin::signed(TEST_TOKEN_OWNER), 42));
        assert_eq!(Tokens::details(42).owner, TEST_TOKEN_OWNER);
    })
}

#[test]
fn create_duplicate_currency_id_fails() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Tokens::create(Origin::signed(TEST_TOKEN_OWNER), TEST_TOKEN_ID),
            Error::<Test>::CurrencyAlreadyExists
        );
    })
}
