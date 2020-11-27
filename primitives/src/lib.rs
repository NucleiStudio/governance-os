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

//! Low level primitives for the governance OS runtime and node.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use governance_os_support::impl_enum_default;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature, OpaqueExtrinsic, RuntimeDebug,
};

/// How we represent currencies in the runtime.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CurrencyId {
    Native,
    Custom(u32),
}
impl Default for CurrencyId {
    fn default() -> Self {
        Self::Native
    }
}

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Type used for expressing timestamp.
pub type Moment = u64;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type.
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

/// The different roles supported by the runtime.
#[derive(Encode, Decode, RuntimeDebug, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Role {
    ApplyAsOrganization(AccountId),
    CreateCurrencies,
    CreateOrganizations,
    ManageCurrency(CurrencyId),
    ManageOrganization(AccountId),
    ManageRoles,
    Root,
    TransferCurrency(CurrencyId),
}
impl governance_os_support::acl::Role for Role {}
// `Default` is used for benchmarks. We have to make sure the default role is not
// root though.
impl_enum_default!(Role, CreateCurrencies);
impl governance_os_pallet_tokens::RoleBuilder for Role {
    type CurrencyId = CurrencyId;
    type Role = Role;

    fn transfer_currency(id: CurrencyId) -> Role {
        Role::TransferCurrency(id)
    }

    fn manage_currency(id: CurrencyId) -> Role {
        Role::ManageCurrency(id)
    }

    fn create_currencies() -> Role {
        Role::CreateCurrencies
    }
}
impl governance_os_pallet_bylaws::RoleBuilder for Role {
    type Role = Role;

    fn manage_roles() -> Role {
        Role::ManageRoles
    }

    fn root() -> Role {
        Role::Root
    }
}
impl governance_os_pallet_organizations::RoleBuilder for Role {
    type OrganizationId = AccountId;
    type Role = Role;

    fn create_organizations() -> Role {
        Role::CreateOrganizations
    }

    fn apply_as_organization(org_id: &AccountId) -> Role {
        Role::ApplyAsOrganization(org_id.clone())
    }

    fn manage_organization(org_id: &AccountId) -> Role {
        Role::ManageOrganization(org_id.clone())
    }
}
