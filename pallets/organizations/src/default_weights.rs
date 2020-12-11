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

impl crate::WeightInfo for () {
    fn create(b: u32) -> Weight {
        (91_504_000 as Weight)
            .saturating_add((27_695_000 as Weight).saturating_mul(b as Weight))
            .saturating_add(DbWeight::get().reads(7 as Weight))
            .saturating_add(DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
            .saturating_add(DbWeight::get().writes(4 as Weight))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
    }
    fn mutate(b: u32, c: u32) -> Weight {
        (37_750_000 as Weight)
            .saturating_add((28_579_000 as Weight).saturating_mul(b as Weight))
            .saturating_add((30_467_000 as Weight).saturating_mul(c as Weight))
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
            .saturating_add(DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
            .saturating_add(DbWeight::get().writes(3 as Weight))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
    }
    fn create_proposal() -> Weight {
        (118_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(8 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    fn veto_proposal(b: u32, c: u32) -> Weight {
        (0 as Weight)
            .saturating_add((26_245_000 as Weight).saturating_mul(b as Weight))
            .saturating_add((25_230_000 as Weight).saturating_mul(c as Weight))
            .saturating_add(DbWeight::get().reads(6 as Weight))
            .saturating_add(DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
            .saturating_add(DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
            .saturating_add(DbWeight::get().writes(3 as Weight))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
    }
    fn decide_on_proposal_favorable(b: u32, c: u32) -> Weight {
        (126_842_000 as Weight)
            .saturating_add((1_272_000 as Weight).saturating_mul(b as Weight))
            .saturating_add((1_280_000 as Weight).saturating_mul(c as Weight))
            .saturating_add(DbWeight::get().reads(7 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    fn decide_on_proposal_against(b: u32, c: u32) -> Weight {
        (115_653_000 as Weight)
            .saturating_add((1_259_000 as Weight).saturating_mul(b as Weight))
            .saturating_add((1_436_000 as Weight).saturating_mul(c as Weight))
            .saturating_add(DbWeight::get().reads(7 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    fn close_proposal(b: u32, c: u32) -> Weight {
        (173_777_000 as Weight)
            .saturating_add((25_685_000 as Weight).saturating_mul(b as Weight))
            .saturating_add((26_287_000 as Weight).saturating_mul(c as Weight))
            .saturating_add(DbWeight::get().reads(6 as Weight))
            .saturating_add(DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
            .saturating_add(DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
            .saturating_add(DbWeight::get().writes(3 as Weight))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
    }
}
