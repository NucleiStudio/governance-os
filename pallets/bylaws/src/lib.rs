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

use frame_support::{weights::Weight, Parameter};
use governance_os_support::traits::RoleManager;
use sp_runtime::{
    traits::{MaybeSerializeDeserialize, Member, StaticLookup},
    DispatchResult,
};
use sp_std::prelude::Vec;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod default_weights;
#[cfg(test)]
mod tests;

pub use pallet::*;

pub trait WeightInfo {
    fn grant_role(b: u32) -> Weight;
    fn revoke_role(b: u32) -> Weight;
}

pub trait RoleBuilder {
    type Role;

    /// Give access to the functions `grant_role` and `revoke_role`.
    fn manage_roles() -> Self::Role;

    /// This role would be the equivalent of a super role. If an account is granted it it can submit
    /// any other calls.
    fn root() -> Self::Role;
}
type RoleBuilderOf<T> = <T as Config>::RoleBuilder;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Roles defines UNIX like roles that users must be granted before triggering certain calls.
        type Role: Parameter + Member + MaybeSerializeDeserialize + Ord + Default;

        /// The weights for this pallet.
        type WeightInfo: WeightInfo;

        /// Only used for weight calculations: this is the highest number of roles we expect one account
        /// to have.
        type MaxRoles: Get<u32>;

        /// Helper for the runtime to specify its custom roles.
        type RoleBuilder: RoleBuilder<Role = Self::Role>;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// We were unable to find the indicated role. It was likely not granted to the target.
        RoleNotFound,
        /// Target was already granted this role.
        RoleAlreadyExists,
    }

    #[pallet::storage]
    #[pallet::getter(fn roles)]
    /// Roles granted to the different accounts. If a role is granted to `None` this means
    /// it is granted to all accounts from the runtime.
    pub(super) type Roles<T: Config> =
        StorageMap<_, Blake2_128Concat, Option<T::AccountId>, Vec<T::Role>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub roles: Vec<(T::Role, Option<T::AccountId>)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                roles: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            self.roles.iter().for_each(|(role, target)| {
                drop(<Pallet<T> as RoleManager>::grant_role(
                    target.as_ref(),
                    role.clone(),
                ))
            });
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", T::Role = "Role")]
    pub enum Event<T: Config> {
        /// A role has been granted to an account. \[account, role\]
        RoleGranted(Option<T::AccountId>, T::Role),
        /// A role has been revoked from an account. \[account, role\]
        RoleRevoked(Option<T::AccountId>, T::Role),
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a `role` to a given account `who`. If `who` is set to `None` this
        /// means that the role is granted to all the accounts of the chain.
        #[pallet::weight(T::WeightInfo::grant_role(T::MaxRoles::get()))]
        pub(super) fn grant_role(
            origin: OriginFor<T>,
            who: Option<<T::Lookup as StaticLookup>::Source>,
            role: T::Role,
        ) -> DispatchResultWithPostInfo {
            Self::ensure_has_role(origin, RoleBuilderOf::<T>::manage_roles())?;

            let target = match who {
                Some(lookmeup) => Some(T::Lookup::lookup(lookmeup)?),
                None => None,
            };

            <Self as RoleManager>::grant_role(target.as_ref(), role)?;

            Ok(().into())
        }

        /// Remove a `role` from a given account `who`. If `who` is set to `None` this means
        /// that the role is revoked for all the accounts of the chain.
        #[pallet::weight(T::WeightInfo::revoke_role(T::MaxRoles::get()))]
        pub(super) fn revoke_role(
            origin: OriginFor<T>,
            who: Option<<T::Lookup as StaticLookup>::Source>,
            role: T::Role,
        ) -> DispatchResultWithPostInfo {
            Self::ensure_has_role(origin, RoleBuilderOf::<T>::manage_roles())?;

            let target = match who {
                Some(lookmeup) => Some(T::Lookup::lookup(lookmeup)?),
                None => None,
            };

            <Self as RoleManager>::revoke_role(target.as_ref(), role)?;

            Ok(().into())
        }
    }
}

impl<T: Config> RoleManager for Pallet<T> {
    type AccountId = T::AccountId;
    type Role = T::Role;

    fn grant_role(target: Option<&Self::AccountId>, role: Self::Role) -> DispatchResult {
        Roles::<T>::try_mutate(target, |v| match v.binary_search(&role.clone()) {
            Ok(_) => Err(Error::<T>::RoleAlreadyExists.into()),
            Err(index) => {
                v.insert(index, role.clone());
                Ok(())
            }
        })
        .map(|result| {
            Self::deposit_event(Event::RoleGranted(target.cloned(), role));
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
            Self::deposit_event(Event::RoleRevoked(target.cloned(), role));
            result
        })
    }

    fn has_role(target: &Self::AccountId, role: Self::Role) -> bool {
        Roles::<T>::get(Some(target))
            .into_iter()
            .chain(Roles::<T>::get(None as Option<T::AccountId>).into_iter())
            .find(|r| {
                return *r == role || *r == RoleBuilderOf::<T>::root();
            })
            .is_some()
    }
}
