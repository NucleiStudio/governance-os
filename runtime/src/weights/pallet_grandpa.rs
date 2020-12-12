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

//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

// Pending on a fix for https://github.com/paritytech/substrate/issues/7383. For now
// we have to stick to the default implementation.
pub type WeightInfo = ();
// pub struct WeightInfo;
// impl pallet_grandpa::WeightInfo for WeightInfo {
// 	// WARNING! Some components were not used: ["x"]
// 	fn check_equivocation_proof() -> Weight {
// 		(135_500_000 as Weight)
// 	}
// 	fn note_stalled() -> Weight {
// 		(6_000_000 as Weight)
// 			.saturating_add(DbWeight::get().writes(1 as Weight))
// 	}
// }
