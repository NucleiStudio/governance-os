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

//! This pallets creates and manages a set of organizations. An organization is linked
//! to a set of executors that can call its `apply_as` function to execute calls as if
//! it came from it.
//! For instance, a voting contract could be deployed and registered as an executor.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::{Dispatchable, Parameter},
    ensure,
    weights::GetDispatchInfo,
};
use frame_system::ensure_signed;
use governance_os_support::{
    acl::RoleManager,
    voting::{VotingHooks, VotingSystem},
    ReservableCurrencies,
};
use sp_runtime::{
    traits::{AccountIdConversion, Hash, StaticLookup},
    DispatchError, DispatchResult, ModuleId,
};
use sp_std::{boxed::Box, prelude::Vec};

mod details;
#[cfg(test)]
mod tests;

use details::{OrganizationDetails, Proposal};

pub trait RoleBuilder {
    type OrganizationId;
    type Role;

    /// Role for creating new organizations.
    fn create_organizations() -> Self::Role;

    /// This role gives the ability to execute calls as if they came
    /// from the organization address.
    fn apply_as_organization(org_id: &Self::OrganizationId) -> Self::Role;
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

    /// Pallet handling currencies. Used to represent voting weights.
    type Currencies: ReservableCurrencies<Self::AccountId>;

    /// The different kinds of voting system present inside the runtime.
    /// **NOTE**: The `Default` voting system will be the one used during benchmarks,
    /// thus you should probably set the most expensive one as the default.
    type VotingSystem: VotingSystem + Default;

    /// Some arbitrary data that can be added to proposals
    type ProposalMetadata: Parameter + Default;

    /// Various hooks as implemented for each voting system.
    type VotingHooks: VotingHooks<
        VotingSystem = Self::VotingSystem,
        AccountId = Self::AccountId,
        OrganizationId = Self::AccountId,
        Currencies = Self::Currencies,
        Data = Self::ProposalMetadata,
    >;
}

type OrganizationDetailsOf<T> =
    OrganizationDetails<<T as frame_system::Trait>::AccountId, <T as Trait>::VotingSystem>;
type ProposalIdOf<T> = <T as frame_system::Trait>::Hash;
type ProposalOf<T> = Proposal<
    Vec<u8>,
    <T as Trait>::ProposalMetadata,
    <T as frame_system::Trait>::AccountId,
    <T as Trait>::VotingSystem,
>;
type RoleBuilderOf<T> = <T as Trait>::RoleBuilder;
type RoleManagerOf<T> = <T as Trait>::RoleManager;

const ORGS_MODULE_ID: ModuleId = ModuleId(*b"gos/orgs");

decl_storage! {
    trait Store for Module<T: Trait> as Organizations {
        pub Counter get(fn counter): u32 = 0;
        pub Parameters get(fn parameters): map hasher(blake2_128_concat) T::AccountId => OrganizationDetailsOf<T>;
        pub Proposals get(fn proposals): map hasher(blake2_128_concat) ProposalIdOf<T> => ProposalOf<T>;
    }
    add_extra_genesis {
        config(organizations): Vec<OrganizationDetailsOf<T>>;
        build(|config: &GenesisConfig<T>| {
            config.organizations.iter().cloned().for_each(|params| drop(Module::<T>::do_create(params)))
        })
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        OrganizationDetails = OrganizationDetailsOf<T>,
        ProposalId = ProposalIdOf<T>,
    {
        /// An organization was created with the following parameters. \[org. address, details\]
        OrganizationCreated(AccountId, OrganizationDetails),
        /// An organization executed a call. \[org. address, result\]
        OrganizationExecuted(AccountId, DispatchResult),
        /// An organization parameters have been modified. \[org. address, old details, new details\]
        OrganizationMutated(AccountId, OrganizationDetails, OrganizationDetails),
        /// A proposal has been submitted to an organization. \[org. address, proposal id\]
        ProposalSubmitted(AccountId, ProposalId),
        /// A proposal has been vetoed and removed from the queue of open proposals. \[org. address, proposal id\]
        ProposalVetoed(AccountId, ProposalId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// We have created the maximum number of organizations, a runtime upgrade may
        /// be necessary.
        CounterOverflow,
        /// This call can only be executed by an organization.
        NotAnOrganization,
        /// A similar proposal already exists.
        ProposalDuplicate,
        /// The proposal is not linked to this organization.
        ProposalNotForOrganization,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Create an organization with the given parameters. An event will be triggered with
        /// the organization's address.
        #[weight = 0]
        fn create(origin, details: OrganizationDetailsOf<T>) {
            RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::create_organizations())?;
            Self::do_create(details)?;
        }

        /// Trigger a call as if it came from the organization itself.
        #[weight = 0]
        fn apply_as(origin, org_id: <T::Lookup as StaticLookup>::Source, call: Box<<T as Trait>::Call>) {
            let target_org_id = T::Lookup::lookup(org_id)?;
            RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::apply_as_organization(&target_org_id))?;

            let res = call.dispatch(frame_system::RawOrigin::Signed(target_org_id.clone()).into());
            Self::deposit_event(RawEvent::OrganizationExecuted(target_org_id, res.map(|_| ()).map_err(|e| e.error)));
        }

        /// Mutate an organization to use the new parameters. Only an organization can call this on itself.
        #[weight = 0]
        fn mutate(origin, new_details: OrganizationDetailsOf<T>) {
            let org_id = Self::ensure_org(origin)?;

            // Make sure everything is sorted for optimization purposes
            let mut new_details = new_details;
            new_details.sort();
            let old_details = Parameters::<T>::take(&org_id);

            Self::run_on_changes(old_details.executors.as_slice(), new_details.executors.as_slice(), |old_account| {
                drop(RoleManagerOf::<T>::revoke_role(Some(old_account), RoleBuilderOf::<T>::apply_as_organization(&org_id)));
            }, |new_account| {
                drop(RoleManagerOf::<T>::grant_role(Some(new_account), RoleBuilderOf::<T>::apply_as_organization(&org_id)));
            });
            Parameters::<T>::insert(&org_id, new_details.clone());

            Self::deposit_event(RawEvent::OrganizationMutated(org_id, old_details, new_details));
        }

        /// Create a proposal for a given organization
        #[weight = 0]
        fn create_proposal(origin, org_id: <T::Lookup as StaticLookup>::Source, call: Box<<T as Trait>::Call>) {
            let who = ensure_signed(origin)?;
            let target_org_id = T::Lookup::lookup(org_id)?;
            ensure!(Parameters::<T>::contains_key(&target_org_id), Error::<T>::NotAnOrganization);

            let proposal_id = Self::proposal_id(&target_org_id, call.clone());
            if Proposals::<T>::contains_key(proposal_id) {
                return Err(Error::<T>::ProposalDuplicate.into());
            }

            let details = Self::parameters(&target_org_id);
            let (maybe_hook_sucessful, additional_data) = T::VotingHooks::on_creating_proposal(details.voting, &who);
            // If the hook returns an err this should stop the flow here
            maybe_hook_sucessful.and_then(|_| {
                Proposals::<T>::insert(&proposal_id, Proposal{
                    org: target_org_id.clone(),
                    call: call.encode(),
                    metadata: additional_data,
                    // Not only does this save us future read weights but it also cover
                    // the case where an org change voting systems but still has pending
                    // proposals.
                    voting: Self::parameters(&target_org_id).voting,
                });

                Ok(())
            })?;

            Self::deposit_event(RawEvent::ProposalSubmitted(target_org_id, proposal_id));
        }

        /// Remove a proposal from the batch of active ones. Has to be called by the organization itself,
        /// typically this could come from an 'apply_as' or a separate vote.
        #[weight = 0]
        fn veto_proposal(origin, proposal_id: ProposalIdOf<T>) {
            let org_id = Self::ensure_org(origin)?;
            let proposal = Self::proposals(proposal_id);
            ensure!(proposal.org == org_id, Error::<T>::ProposalNotForOrganization);

            T::VotingHooks::on_veto_proposal(proposal.voting, proposal.metadata)?;
            Proposals::<T>::remove(proposal_id);

            Self::deposit_event(RawEvent::ProposalVetoed(org_id, proposal_id));
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

    /// Given any counter return the associated organization id
    fn org_id_for(counter: u32) -> T::AccountId {
        ORGS_MODULE_ID.into_sub_account(counter)
    }

    /// Makes sure that the `origin` is a registered organization
    fn ensure_org(origin: T::Origin) -> Result<T::AccountId, DispatchError> {
        match ensure_signed(origin) {
            Err(e) => Err(e.into()),
            Ok(maybe_org_id) if Parameters::<T>::contains_key(&maybe_org_id) => Ok(maybe_org_id),
            _ => Err(Error::<T>::NotAnOrganization.into()),
        }
    }

    /// Hash the block number, org id and proposal together to generate its id
    fn proposal_id(org_id: &T::AccountId, proposal: Box<<T as Trait>::Call>) -> ProposalIdOf<T> {
        T::Hashing::hash_of(&[
            frame_system::Module::<T>::block_number().encode(),
            org_id.encode(),
            proposal.encode(),
        ])
    }

    fn do_create(details: OrganizationDetailsOf<T>) -> DispatchResult {
        let counter = Self::counter();
        let new_counter = counter.checked_add(1).ok_or(Error::<T>::CounterOverflow)?;
        let org_id = Self::org_id_for(counter);

        // Sorting details allows us to be more efficient when updating them later on.
        let mut details = details;
        details.sort();

        // We first write the counter so that even if the calls below fail we will always regenerate a new and
        // different organization id.
        Counter::put(new_counter);
        details.executors.iter().for_each(|account| {
            drop(RoleManagerOf::<T>::grant_role(
                Some(&account),
                RoleBuilderOf::<T>::apply_as_organization(&org_id),
            ));
        });
        Parameters::<T>::insert(&org_id, details.clone());

        Self::deposit_event(RawEvent::OrganizationCreated(org_id, details));
        Ok(())
    }
}
