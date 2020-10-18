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

//! This module hosts the runtime for the governance OS, it bundles the other
//! pallets from the governance OS project. It aims at providing the following:
//! - a demonstration runtime for anybody willing to test the pallets
//! - a reference runtime for anybody trying to use the pallets in their own runtime

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use frame_support::construct_runtime;
use governance_os_primitives::{BlockNumber, Signature};
use sp_api::impl_runtime_apis;
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, Block as BlockT, StaticLookup},
};
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

mod constants;
mod pallets_consensus;
mod pallets_core;
mod pallets_economics;
mod version;

pub use pallets_consensus::SessionKeys;
pub use pallets_economics::NativeCurrency;
pub use version::VERSION;

construct_runtime!(
    pub enum Runtime
    where
        Block = Block,
        NodeBlock = governance_os_primitives::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // Core
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Indices: pallet_indices::{Module, Call, Storage, Config<T>, Event<T>},

        // Consensus
        Aura: pallet_aura::{Module, Config<T>, Inherent},
        Grandpa: pallet_grandpa::{Module, Call, Storage, Config, Event},
        Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},

        // Economics
        Tokens: governance_os_pallet_tokens::{Module, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Module, Storage},
    }
);

/// The address format for describing accounts.
pub type Address = <Indices as StaticLookup>::Source;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllModules,
>;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }
}
