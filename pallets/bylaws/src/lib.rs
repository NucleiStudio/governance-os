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

//! This pallet implements an ACL system for account level permissioning.
//! A role is the equivalent of a UNIX role, it can be granted either to
//! one or many account or even to all the accounts within the system.
//! Pallets can then use it to define custom role requirements.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, traits::Get, weights::Weight,
};
use governance_os_support::acl::{Role, RoleManager};
use sp_runtime::{traits::StaticLookup, DispatchResult};
use sp_std::prelude::Vec;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod default_weights;
#[cfg(test)]
mod tests;

pub trait WeightInfo {
    fn grant_role() -> Weight;
    fn revoke_role() -> Weight;
}

pub trait RoleBuilder {
    type Role;

    /// Give access to the functions `grant_role` and `revoke_role`.
    fn manage_roles() -> Self::Role;

    /// This role would be the equivalent of a super role. If an account is granted it it can submit
    /// any other calls.
    fn root() -> Self::Role;
}

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// Roles defines UNIX like roles that users must be granted before triggering certain calls.
    type Role: Role + Default;

    /// The weights for this pallet.
    type WeightInfo: WeightInfo;

    /// Only used for weight calculations: this is the highest number of roles we expect one account
    /// to have.
    type MaxRoles: Get<u32>;

    /// Helper for the runtime to specify its custom roles.
    type RoleBuilder: RoleBuilder<Role = Self::Role>;
}

type RoleBuilderOf<T> = <T as Trait>::RoleBuilder;

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// We were unable to find the indicated role. It was likely not granted to the target.
        RoleNotFound,
        /// Target was already granted this role.
        RoleAlreadyExists,
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as Bylaws {
        /// Roles granted to the different accounts. If a role is granted to `None` this means
        /// it is granted to all accounts from the runtime.
        pub Roles get(fn roles): map hasher(blake2_128_concat) Option<T::AccountId> => Vec<T::Role>;
    }
    add_extra_genesis {
        config(roles): Vec<(T::Role, Option<T::AccountId>)>;
        build(|config: &GenesisConfig<T>| {
            config.roles.iter().for_each(|(role, target)| drop(<Module<T> as RoleManager>::grant_role(target.as_ref(), *role)));
        })
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Role = <T as Trait>::Role,
    {
        RoleGranted(Option<AccountId>, Role),
        RoleRevoked(Option<AccountId>, Role),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Add a `role` to a given account `who`. If `who` is set to `None` this
        /// means that the role is granted to all the accounts of the chain.
        #[weight = T::WeightInfo::grant_role()]
        fn grant_role(origin, who: Option<<T::Lookup as StaticLookup>::Source>, role: T::Role) {
            Self::ensure_has_role(origin, RoleBuilderOf::<T>::manage_roles())?;

            let target = match who {
                Some(lookmeup) => Some(T::Lookup::lookup(lookmeup)?),
                None => None,
            };

            <Self as RoleManager>::grant_role(target.as_ref(), role)?;
        }

        /// Remove a `role` from a given account `who`. If `who` is set to `None` this means
        /// that the role is revoked for all the accounts of the chain.
        #[weight = T::WeightInfo::revoke_role()]
        fn revoke_role(origin, who: Option<<T::Lookup as StaticLookup>::Source>, role: T::Role) {
            Self::ensure_has_role(origin, RoleBuilderOf::<T>::manage_roles())?;

            let target = match who {
                Some(lookmeup) => Some(T::Lookup::lookup(lookmeup)?),
                None => None,
            };

            <Self as RoleManager>::revoke_role(target.as_ref(), role)?;
        }
    }
}

impl<T: Trait> RoleManager for Module<T> {
    type AccountId = T::AccountId;
    type Role = T::Role;

    fn grant_role(target: Option<&Self::AccountId>, role: Self::Role) -> DispatchResult {
        Roles::<T>::try_mutate(target, |v| match v.binary_search(&role) {
            Ok(_) => Err(Error::<T>::RoleAlreadyExists.into()),
            Err(index) => {
                v.insert(index, role);
                Ok(())
            }
        })
        .map(|result| {
            Self::deposit_event(RawEvent::RoleGranted(target.cloned(), role));
            result
        })
    }

    fn revoke_role(target: Option<&Self::AccountId>, role: Self::Role) -> DispatchResult {
        Roles::<T>::try_mutate_exists(target, |v| {
            let mut vec = v.take().unwrap_or_default();
            match vec.binary_search(&role) {
                Ok(index) => {
                    vec.remove(index);
                    if vec.is_empty() {
                        *v = None;
                    } else {
                        *v = Some(vec);
                    }
                    Ok(())
                }
                Err(_) => Err(Error::<T>::RoleNotFound.into()),
            }
        })
        .map(|result| {
            Self::deposit_event(RawEvent::RoleRevoked(target.cloned(), role));
            result
        })
    }

    fn has_role(target: &Self::AccountId, role: Self::Role) -> bool {
        Roles::<T>::get(Some(target))
            .iter()
            .chain(Roles::<T>::get(None as Option<T::AccountId>).iter())
            .cloned()
            .find(|r| {
                return *r == role || *r == RoleBuilderOf::<T>::root();
            })
            .is_some()
    }
}
