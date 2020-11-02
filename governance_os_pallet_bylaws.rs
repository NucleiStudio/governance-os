//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

pub struct WeightInfo;
impl governance_os_pallet_bylaws::WeightInfo for WeightInfo {
	fn grant_role(b: u32, ) -> Weight {
		(74_416_000 as Weight)
			.saturating_add((22_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn revoke_role(b: u32, ) -> Weight {
		(75_222_000 as Weight)
			.saturating_add((850_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
}
