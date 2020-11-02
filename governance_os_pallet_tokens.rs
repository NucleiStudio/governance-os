//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

pub struct WeightInfo;
impl governance_os_pallet_tokens::WeightInfo for WeightInfo {
	fn create() -> Weight {
		(123_000_000 as Weight)
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(5 as Weight))
	}
	fn mint() -> Weight {
		(118_000_000 as Weight)
			.saturating_add(DbWeight::get().reads(8 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	fn burn() -> Weight {
		(118_000_000 as Weight)
			.saturating_add(DbWeight::get().reads(8 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	fn update_details() -> Weight {
		(125_000_000 as Weight)
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	fn transfer() -> Weight {
		(150_000_000 as Weight)
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
}
