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
//! We then link the pallet to a `CallFilter` that is in charge or associating
//! incoming calls to expected roles.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_event, decl_module, decl_storage, traits::Get};
use frame_system::ensure_root;
use governance_os_support::acl::{CallFilter, Role};
use sp_runtime::traits::StaticLookup;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
mod signed_extra;
#[cfg(test)]
mod tests;

pub use signed_extra::CheckRole;

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// Roles defines UNIX like roles that users must be granted before triggering certain calls.
    type Role: Role;

    /// This role would be the equivalent of a super role. If an account is granted it it can submit
    /// any other calls.
    type RootRole: Get<Self::Role>;

    /// The call filter is in charge of tagging incoming calls with roles that are needed.
    type CallFilter: CallFilter<Self::AccountId, Self::Call, Self::Role>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Bylaws {
        /// Roles granted to the different accounts. If a role is granted to `None` this means
        /// it is granted to all accounts from the runtime.
        pub Roles get(fn roles): double_map hasher(blake2_128_concat) T::Role, hasher(blake2_128_concat) Option<T::AccountId> => bool;
    }
    add_extra_genesis {
        config(roles): Vec<(T::Role, Option<T::AccountId>)>;
        build(|config: &GenesisConfig<T>| {
            config.roles.iter().for_each(|(role, target)| Module::<T>::set_role(target.as_ref(), *role));
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
        #[weight = 0]
        fn grant_role(origin, who: Option<<T::Lookup as StaticLookup>::Source>, role: T::Role) {
            ensure_root(origin)?;
            let target = match who {
                Some(lookmeup) => Some(T::Lookup::lookup(lookmeup)?),
                None => None,
            };

            Self::set_role(target.as_ref(), role);
            Self::deposit_event(RawEvent::RoleGranted(target, role));
        }

        /// Remove a `role` from a given account `who`. If `who` is set to `None` this means
        /// that the role is revoked for all the accounts of the chain.
        #[weight = 0]
        fn revoke_role(origin, who: Option<<T::Lookup as StaticLookup>::Source>, role: T::Role) {
            ensure_root(origin)?;
            let target = match who {
                Some(lookmeup) => Some(T::Lookup::lookup(lookmeup)?),
                None => None,
            };

            Self::unset_role(target.as_ref(), role);
            Self::deposit_event(RawEvent::RoleRevoked(target, role));
        }
    }
}

impl<T: Trait> Module<T> {
    fn set_role(target: Option<&T::AccountId>, role: T::Role) {
        Roles::<T>::mutate(role, target, |d| *d = true)
    }

    fn unset_role(target: Option<&T::AccountId>, role: T::Role) {
        Roles::<T>::remove(role, target)
    }

    pub fn has_role(target: &T::AccountId, role: T::Role) -> bool {
        Roles::<T>::get(role, Some(target)) || Roles::<T>::get(role, None as Option<&T::AccountId>)
    }
}
