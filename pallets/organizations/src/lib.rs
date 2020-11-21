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

//! This pallet bundles together the `bylaws` and `tokens` ones to create and
//! manage decentralized autonomous organizations.
//! Note that we give an account id to every organization created, this opens up
//! the possibility to send some funds to an organization and have it spend the funds
//! itself with no additional logic.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::{Dispatchable, Parameter},
    weights::GetDispatchInfo,
};
use governance_os_support::acl::RoleManager;
use sp_runtime::{traits::AccountIdConversion, DispatchResult, ModuleId};
use sp_std::boxed::Box;

mod details;
#[cfg(test)]
mod tests;
use details::OrganizationDetails;

pub trait RoleBuilder {
    type OrganizationId;
    type Role;

    /// Role for creating new organizations.
    fn create_organizations() -> Self::Role;

    /// This role gives the ability to execute calls as if they came
    /// from the organization address.
    fn apply_as_organization(org_id: Self::OrganizationId) -> Self::Role;

    /// This role give management access for a given organization. Typically
    /// this would imply the ability to add new executors (that get granted the
    /// `apply_as` role).
    fn manage_organization(org_id: Self::OrganizationId) -> Self::Role;
}

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// Calls triggered from an organization.
    type Call: Parameter + GetDispatchInfo + Dispatchable<Origin = Self::Origin>;

    /// Pallet that is in charge of managing the roles based ACL.
    type RoleManager: RoleManager<AccountId = Self::AccountId>;

    /// This pallet relies on roles associated to a specific metadata so we need the runtime
    /// to provide some helper functions to build those so that we can keep the role definition
    /// code modular.
    type RoleBuilder: RoleBuilder<
        OrganizationId = Self::AccountId,
        Role = <RoleManagerOf<Self> as RoleManager>::Role,
    >;
}

type RoleBuilderOf<T> = <T as Trait>::RoleBuilder;
type RoleManagerOf<T> = <T as Trait>::RoleManager;

const ORGS_MODULE_ID: ModuleId = ModuleId(*b"gos/orgs");

decl_storage! {
    trait Store for Module<T: Trait> as Organizations {
        pub CreatedOrganizations get(fn created_organizations): u32 = 0;
        pub OrganizationsParameters get(fn organizations_parameters): map hasher(blake2_128_concat) T::AccountId => OrganizationDetails<T::AccountId>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        OrganizationDetails = OrganizationDetails<<T as frame_system::Trait>::AccountId>,
    {
        /// An organization was created with the following parameters. \[org. address, details\]
        OrganizationCreated(AccountId, OrganizationDetails),
        /// An organization executed a call. \[org. address, result\]
        OrganizationExecuted(AccountId, DispatchResult),
        /// An organization parameters have been modified. \[org. address, old details, new details\]
        OrganizationMutated(AccountId, OrganizationDetails, OrganizationDetails),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// We have created the maximum number of organizations, a runtime upgrade may
        /// be necessary.
        CreatedOrganizationsOverflow,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Create an organization with the given parameters. An event will be triggered with
        /// the organization's address.
        #[weight = 0]
        fn create(origin, details: OrganizationDetails<T::AccountId>) {
            RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::create_organizations())?;

            let counter = Self::created_organizations();
            let new_counter = counter.checked_add(1).ok_or(Error::<T>::CreatedOrganizationsOverflow)?;
            let org_id: T::AccountId = ORGS_MODULE_ID.into_sub_account(counter);

            // Sorting details allows us to be more efficient when updating them later on.
            let mut details = details;
            details.sort();

            // We first write the counter so that even if the calls below fail we will always regenerate a new and
            // different organization id.
            CreatedOrganizations::put(new_counter);
            details.executors
                .iter()
                .for_each(|account| {
                    drop(RoleManagerOf::<T>::grant_role(Some(&account), RoleBuilderOf::<T>::apply_as_organization(org_id.clone())));
                });
            details.managers
                .iter()
                .for_each(|account| {
                    drop(RoleManagerOf::<T>::grant_role(Some(&account), RoleBuilderOf::<T>::manage_organization(org_id.clone())));
                });
            OrganizationsParameters::<T>::insert(org_id.clone(), details.clone());

            Self::deposit_event(RawEvent::OrganizationCreated(org_id, details));
        }

        /// Trigger a call as if it came from the organization itself.
        #[weight = 0]
        fn apply_as(origin, org_id: T::AccountId, call: Box<<T as Trait>::Call>) {
            RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::apply_as_organization(org_id.clone()))?;

            let res = call.dispatch(frame_system::RawOrigin::Signed(org_id.clone()).into());
            Self::deposit_event(RawEvent::OrganizationExecuted(org_id, res.map(|_| ()).map_err(|e| e.error)));
        }

        /// Mutate an organization to use the new parameters.
        #[weight = 0]
        fn mutate(origin, org_id: T::AccountId, new_details: OrganizationDetails<T::AccountId>) {
            RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::manage_organization(org_id.clone()))?;

            // Make sure everything is sorted for optimization purposes
            let mut new_details = new_details;
            new_details.sort();
            let old_details = OrganizationsParameters::<T>::get(org_id.clone());

            Self::run_on_changes(old_details.executors.as_slice(), new_details.executors.as_slice(), |old_account| {
                drop(RoleManagerOf::<T>::revoke_role(Some(old_account), RoleBuilderOf::<T>::apply_as_organization(org_id.clone())));
            }, |new_account| {
                drop(RoleManagerOf::<T>::grant_role(Some(new_account), RoleBuilderOf::<T>::apply_as_organization(org_id.clone())));
            });
            Self::run_on_changes(old_details.managers.as_slice(), new_details.managers.as_slice(), |old_account| {
                drop(RoleManagerOf::<T>::revoke_role(Some(old_account), RoleBuilderOf::<T>::manage_organization(org_id.clone())));
            }, |new_account| {
                drop(RoleManagerOf::<T>::grant_role(Some(new_account), RoleBuilderOf::<T>::manage_organization(org_id.clone())));
            });

            Self::deposit_event(RawEvent::OrganizationMutated(org_id, old_details, new_details));
        }
    }
}

impl<T: Trait> Module<T> {
    /// A handy helper functions to run two functions on what elements were removed from a
    /// slice and on the added elements. Should be pretty useful when trying to limit
    /// database read and writes since we execute the functions only on the changed elements.
    fn run_on_changes<Elem: Ord>(
        old_vec: &[Elem],
        new_vec: &[Elem],
        on_old: impl Fn(&Elem),
        on_new: impl Fn(&Elem),
    ) {
        Self::run_if_not_in_right(old_vec, new_vec, on_old);
        Self::run_if_not_in_right(new_vec, old_vec, on_new);
    }

    /// Run `to_run` on every elent that is in `left` but not in `right`.
    fn run_if_not_in_right<Elem: Ord>(left: &[Elem], right: &[Elem], to_run: impl Fn(&Elem)) {
        left.into_iter().for_each(|elem| {
            if right.binary_search(&elem).is_err() {
                to_run(elem);
            }
        });
    }
}
