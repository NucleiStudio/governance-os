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

use crate::{Bylaw, CallTagger, CallTags, Event, Runtime};
use frame_support::parameter_types;
use sp_std::prelude::*;

parameter_types! {
    pub DefaultBylaws: Vec<(CallTags, Bylaw)> = vec![(CallTags::Any, Bylaw::Allow)];
    pub const MaxBylaws: u32 = 1_000;
}

impl governance_os_pallet_bylaws::Trait for Runtime {
    type Event = Event;
    type Tag = CallTags;
    type Tagger = CallTagger;
    type Bylaw = Bylaw;
    type DefaultBylaws = DefaultBylaws;
    type MaxBylaws = MaxBylaws;
    type WeightInfo = ();
}
