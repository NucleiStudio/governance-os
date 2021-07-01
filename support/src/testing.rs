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

//! A series of helpers and common values used for tests.

pub const ROOT: primitives::AccountId = 0;
pub const TEST_TOKEN_ID: primitives::CurrencyId = 1;
pub const TEST_TOKEN_OWNER: primitives::AccountId = 2;
pub const ALICE: primitives::AccountId = 3;
pub const BOB: primitives::AccountId = 4;
pub const CHARLIE: primitives::AccountId = 5;
pub const EVE: primitives::AccountId = 6;

pub mod primitives {
    pub type AccountId = u128;
    pub type Balance = u128;
    pub type CurrencyId = u8;
    pub type BlockNumber = u64;
}
