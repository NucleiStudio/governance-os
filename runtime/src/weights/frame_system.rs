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

pub struct WeightInfo;
impl frame_system::WeightInfo for WeightInfo {
    // WARNING! Some components were not used: ["b"]
    fn remark() -> Weight {
        (2_677_000 as Weight)
    }
    fn set_heap_pages() -> Weight {
        (4_000_000 as Weight).saturating_add(DbWeight::get().writes(1 as Weight))
    }
    fn set_changes_trie_config(d: u32) -> Weight {
        (11_730_000 as Weight)
            .saturating_add((3_000 as Weight).saturating_mul(d as Weight))
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    fn set_storage(i: u32) -> Weight {
        (4_336_000 as Weight)
            .saturating_add((1_474_000 as Weight).saturating_mul(i as Weight))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
    }
    fn kill_storage(i: u32) -> Weight {
        (0 as Weight)
            .saturating_add((1_055_000 as Weight).saturating_mul(i as Weight))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
    }
    fn kill_prefix(p: u32) -> Weight {
        (0 as Weight)
            .saturating_add((1_851_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
    }
    fn suicide() -> Weight {
        (57_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(4 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
}
