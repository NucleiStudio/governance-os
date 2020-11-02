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

#[macro_export]
/// This macro makes it easy to prepare a mock testing runtime. It also avoid
/// the developer from duplicating mocking code. However, the developer is still
/// expected to write the correct imports for now.
/// The mock runtime is composed of the `system` and `bylaws` pallets as all the
/// other pallets of the Governance OS project are supposed to depend on those; so
/// better not write the same code twice.
macro_rules! mock_runtime {
    ($runtime:tt) => {
        mock_runtime!($runtime, ());
    };

    ($runtime:tt, $account_data:ty) => {
        #[derive(Clone, Eq, PartialEq, RuntimeDebug)]
        pub struct $runtime;

        impl_outer_origin! {
            pub enum Origin for $runtime {}
        }

        impl_outer_dispatch! {
            pub enum Call for $runtime where origin: Origin {
                frame_system::System,
            }
        }

        impl frame_system::Trait for $runtime {
            type BaseCallFilter = ();
            type Origin = Origin;
            type Call = Call;
            type Index = u64;
            type BlockNumber = u64;
            type Hash = H256;
            type Hashing = BlakeTwo256;
            type AccountId = AccountId;
            type Lookup = IdentityLookup<Self::AccountId>;
            type Header = Header;
            type Event = ();
            type BlockHashCount = BlockHashCount;
            type MaximumBlockWeight = MaximumBlockWeight;
            type DbWeight = ();
            type BlockExecutionWeight = ();
            type ExtrinsicBaseWeight = ();
            type MaximumExtrinsicWeight = MaximumBlockWeight;
            type MaximumBlockLength = MaximumBlockLength;
            type AvailableBlockRatio = AvailableBlockRatio;
            type Version = ();
            type PalletInfo = ();
            type AccountData = $account_data;
            type OnNewAccount = ();
            type OnKilledAccount = ();
            type SystemWeightInfo = ();
        }

        #[derive(
            Eq,
            PartialEq,
            RuntimeDebug,
            Encode,
            Decode,
            Copy,
            Clone,
            Serialize,
            Deserialize,
            Ord,
            PartialOrd,
        )]
        pub enum MockRoles {
            Root,
            RemarkOnly,
            CreateCurrencies,
            TransferCurrency(CurrencyId),
            ManageCurrency(CurrencyId),
        }
        impl Role for MockRoles {}
        impl_enum_default!(MockRoles, RemarkOnly);

        parameter_types! {
            pub const RootRole: MockRoles = MockRoles::Root;
            pub const MaxRoles: u32 = 5;
        }

        impl governance_os_pallet_bylaws::Trait for $runtime {
            type Event = ();
            type Role = MockRoles;
            type RootRole = RootRole;
            type WeightInfo = ();
            type MaxRoles = MaxRoles;
        }

        pub type Bylaws = governance_os_pallet_bylaws::Module<Test>;
        pub type System = frame_system::Module<Test>;
    };
}
