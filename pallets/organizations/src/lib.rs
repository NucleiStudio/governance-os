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

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_error, decl_event, decl_module, decl_storage};
use governance_os_support::{acl::RoleManager, Currencies};
use sp_runtime::{traits::AccountIdConversion, ModuleId};

mod details;
use details::OrganizationDetails;

pub trait RoleBuilder {
    type Role;

    /// Role for creating new organizations.
    fn create_organizations() -> Self::Role;
}

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// Since orgnizations can use different tokens to represent voting shares we need this
    /// type to be specified so that we can access all of the runtime's currencies.
    type Currencies: Currencies<Self::AccountId>;

    /// Pallet that is in charge of managing the roles based ACL.
    type RoleManager: RoleManager<AccountId = Self::AccountId>;

    /// This pallet relies on roles associated to a specific metadata so we need the runtime
    /// to provide some helper functions to build those so that we can keep the role definition
    /// code modular.
    type RoleBuilder: RoleBuilder<Role = <RoleManagerOf<Self> as RoleManager>::Role>;
}

type CurrencyIdOf<T> =
    <<T as Trait>::Currencies as Currencies<<T as frame_system::Trait>::AccountId>>::CurrencyId;
type RoleBuilderOf<T> = <T as Trait>::RoleBuilder;
type RoleManagerOf<T> = <T as Trait>::RoleManager;

const ORGS_MODULE_ID: ModuleId = ModuleId(*b"gos/orgs");

decl_storage! {
    trait Store for Module<T: Trait> as Organizations {
        pub CreatedOrganizations get(fn created_organizations): u32 = 0;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        OrganizationDetails = OrganizationDetails<CurrencyIdOf<T>>,
    {
        /// An organization was created with the following parameters. \[org. address, details\]
        OrganizationCreated(AccountId, OrganizationDetails),
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
        fn create(origin, details: OrganizationDetails<CurrencyIdOf<T>>) {
            RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::create_organizations())?;

            let counter = Self::created_organizations();
            let new_counter = counter.checked_add(1).ok_or(Error::<T>::CreatedOrganizationsOverflow)?;
            let org_id: T::AccountId = ORGS_MODULE_ID.into_sub_account(counter);

            // TODO: actually create the stuff
            CreatedOrganizations::put(new_counter);

            Self::deposit_event(RawEvent::OrganizationCreated(org_id, details));
        }
    }
}
