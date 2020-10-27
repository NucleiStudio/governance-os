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

use crate::{CheckBylaws, Module, Trait};
use codec::{Decode, Encode};
use frame_support::{impl_outer_origin, parameter_types};
pub use governance_os_runtime::Call;
use governance_os_support::{
    rules::{CallTagger, Rule, SuperSetter},
    testing::{
        primitives::AccountId, AvailableBlockRatio, BlockHashCount, MaximumBlockLength,
        MaximumBlockWeight,
    },
};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, DispatchInfoOf, IdentityLookup},
    RuntimeDebug,
};
use sp_std::{fmt::Debug, marker};

impl_outer_origin! {
    pub enum Origin for Test {}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Test;

impl frame_system::Trait for Test {
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
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
}

#[derive(Eq, PartialEq, RuntimeDebug, Encode, Decode, Copy, Clone, Serialize, Deserialize)]
pub enum MockTags {
    Test,
    Misc,
}

impl SuperSetter for MockTags {
    fn is_superset(&self, other: &Self) -> bool {
        self == other
    }
}

impl Default for MockTags {
    fn default() -> Self {
        MockTags::Misc
    }
}

pub struct MockTagger<T>(marker::PhantomData<T>);
impl<T: Trait> CallTagger<AccountId, Call, MockTags> for MockTagger<T> {
    fn tag(_who: &AccountId, call: &Call) -> MockTags {
        match matches!(call, Call::System(frame_system::Call::remark(..))) {
            true => MockTags::Test,
            false => MockTags::Misc,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, Serialize, Deserialize)]
pub enum Bylaw {
    Allow,
    Deny,
}

impl Rule<AccountId, Call> for Bylaw {
    fn validate(
        &self,
        _who: &AccountId,
        _call: &Call,
        _info: &DispatchInfoOf<Call>,
        _len: usize,
    ) -> bool {
        use Bylaw::*;

        match self {
            Allow => true,
            Deny => false,
        }
    }
}

impl Default for Bylaw {
    fn default() -> Self {
        Bylaw::Allow
    }
}

parameter_types! {
    pub DefaultBylaws: Vec<(MockTags, Bylaw)> = vec![(MockTags::Test, Bylaw::Allow), (MockTags::Misc, Bylaw::Deny)];
    // NOTE: we use a small number so that tests gives results in a short enough time.
    // In production you'd probably want a higher value.
    pub const MaxBylaws: u32 = 10;
}

impl Trait for Test {
    type Event = ();
    type Tag = MockTags;
    type Tagger = MockTagger<Test>;
    type DefaultBylaws = DefaultBylaws;
    type Bylaw = Bylaw;
    type MaxBylaws = MaxBylaws;
    type WeightInfo = ();
}

pub type System = frame_system::Module<Test>;
pub type Bylaws = Module<Test>;
pub type MockCheckBylaws = CheckBylaws<Test>;

pub struct ExtBuilder;

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
