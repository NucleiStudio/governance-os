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

use frame_support::{parameter_types, weights::Weight};
use sp_runtime::Perbill;

pub const ALICE: primitives::AccountId = 1;
pub const BOB: primitives::AccountId = 2;
pub const TEST_TOKEN_ID: primitives::CurrencyId = 3;
pub const TEST_TOKEN_OWNER: primitives::AccountId = 4;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

pub mod primitives {
    pub type AccountId = u64;
    pub type Balance = u64;
    pub type CurrencyId = u8;
}
