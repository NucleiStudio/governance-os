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

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::{DispatchResultWithPostInfo, Dispatchable, Parameter, PostDispatchInfo},
    ensure,
    traits::Get,
    weights::{GetDispatchInfo, Weight},
};
use frame_system::ensure_signed;
use governance_os_support::traits::{ProposalResult, RoleManager, VotingRouter};
use sp_runtime::{
    traits::{AccountIdConversion, Hash, StaticLookup},
    DispatchError, DispatchResult, ModuleId,
};
use sp_std::{boxed::Box, prelude::Vec};

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod default_weights;
mod details;
#[cfg(test)]
mod tests;

pub use details::{OrganizationDetails, Proposal};

pub trait RoleBuilder {
    type OrganizationId;
    type Role;

    /// Role for creating new organizations.
    fn create_organizations() -> Self::Role;

    /// This role gives the ability to execute calls as if they came
    /// from the organization address.
    fn apply_as_organization(org_id: &Self::OrganizationId) -> Self::Role;
}

pub trait WeightInfo {
    fn create(b: u32) -> Weight;
    fn mutate(b: u32, c: u32) -> Weight;
    fn create_proposal() -> Weight;
    fn veto_proposal(b: u32, c: u32) -> Weight;
    fn decide_on_proposal(b: u32) -> Weight;
    fn close_proposal(b: u32, c: u32) -> Weight;
}

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// Calls triggered from an organization.
    type Call: Parameter
        + GetDispatchInfo
        + From<frame_system::Call<Self>>
        + Dispatchable<Origin = Self::Origin, PostInfo = PostDispatchInfo>;

    /// Pallet that is in charge of managing the roles based ACL.
    type RoleManager: RoleManager<AccountId = Self::AccountId>;

    /// This pallet relies on roles associated to a specific metadata so we need the runtime
    /// to provide some helper functions to build those so that we can keep the role definition
    /// code modular.
    type RoleBuilder: RoleBuilder<
        OrganizationId = Self::AccountId,
        Role = <RoleManagerOf<Self> as RoleManager>::Role,
    >;

    /// VotingRouter implementation to choose between voting systems and route those
    /// to the right pallets.
    type VotingRouter: VotingRouter<AccountId = Self::AccountId, ProposalId = ProposalIdOf<Self>>;

    /// Mostly used for weight computations and not actually enforced. The maximum number
    /// of votes in favor or against we can expect a proposal to have.
    type MaxVotes: Get<u32>;

    /// Mostly used for weight computations and not actually enforced. Maximum numbers of
    /// executors we expect to be configured for an organization.
    type MaxExecutors: Get<u32>;

    /// Weight values for this pallet
    type WeightInfo: WeightInfo;
}

type OrganizationDetailsOf<T> = OrganizationDetails<
    <T as frame_system::Trait>::AccountId,
    (VotingSystemIdOf<T>, VotingParametersOf<T>),
>;
pub type OrganizationsCounter = u32;
type ProposalIdOf<T> = <T as frame_system::Trait>::Hash;
type ProposalOf<T> = Proposal<Vec<u8>, <T as frame_system::Trait>::AccountId, VotingSystemIdOf<T>>;
type RoleBuilderOf<T> = <T as Trait>::RoleBuilder;
type RoleManagerOf<T> = <T as Trait>::RoleManager;
type VoteDataOf<T> = <<T as Trait>::VotingRouter as VotingRouter>::VoteData;
type VotingParametersOf<T> = <<T as Trait>::VotingRouter as VotingRouter>::Parameters;
type VotingSystemIdOf<T> = <<T as Trait>::VotingRouter as VotingRouter>::VotingSystemId;

const ORGS_MODULE_ID: ModuleId = ModuleId(*b"gos/orgs");

decl_storage! {
    trait Store for Module<T: Trait> as Organizations {
        pub Counter get(fn counter): OrganizationsCounter = 0;
        pub Parameters get(fn parameters): map hasher(blake2_128_concat) T::AccountId => Option<OrganizationDetailsOf<T>>;
        pub Proposals get(fn proposals): map hasher(blake2_128_concat) ProposalIdOf<T> => Option<ProposalOf<T>>;
    }
    add_extra_genesis {
        config(organizations): Vec<OrganizationDetailsOf<T>>;
        build(|config: &GenesisConfig<T>| {
            config.organizations.iter().cloned().for_each(|params| {
                Module::<T>::do_create(params)
                    .expect("org creation in genesis block shall not fail")
            })
        })
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        OrganizationDetails = OrganizationDetailsOf<T>,
        ProposalId = ProposalIdOf<T>,
        VoteData = VoteDataOf<T>,
    {
        /// An organization was created with the following parameters. \[org. address, details\]
        OrganizationCreated(AccountId, OrganizationDetails),
        /// An organization executed a call. \[org. address, result\]
        OrganizationExecuted(AccountId, DispatchResult),
        /// An organization parameters have been modified. \[org. address, old details, new details\]
        OrganizationMutated(AccountId, OrganizationDetails, OrganizationDetails),
        /// A proposal has been submitted to an organization. \[org. address, proposal id\]
        ProposalSubmitted(AccountId, ProposalId),
        /// A proposal has been vetoed and removed from the queue of open proposals. \[proposal id\]
        ProposalVetoed(ProposalId),
        /// Somebody just voted on a proposal. \[proposal id, voter, vote data\]
        ProposalVoteCasted(ProposalId, AccountId, VoteData),
        /// A proposal has been executed with the following result. \[proposal id, result\]
        ProposalExecuted(ProposalId, DispatchResult),
        /// A proposal was closed. \[proposal id, wether it passed or not\]
        ProposalClosed(ProposalId, ProposalResult),
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
        /// The proposal does not exists, maybe it was already closed.
        ProposalNotFound,
        /// This proposal can not be closed yet. It probably needs to wait for more votes.
        ProposalCanNotBeClosed,
        /// The proposal code couldn't be decoded for some reason. This isn't expected to ever
        /// happen and thus should be reported upstream.
        ProposalDecodingFailure,
        /// The weight passed to the function is too small.
        TooSmallWeightBound,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Create an organization with the given parameters. An event will be triggered with
        /// the organization's address.
        #[weight = T::WeightInfo::create(details.executors.len() as u32)]
        fn create(origin, details: OrganizationDetailsOf<T>) {
            RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::create_organizations())?;
            Self::do_create(details)?;
        }

        /// Trigger a call as if it came from the organization itself.
        #[weight =
            call.get_dispatch_info().weight
                .saturating_add(10_000)
                // AccountData for inner call origin accountdata.
                .saturating_add(T::DbWeight::get().reads_writes(1, 1))
                // Two read performed by ensure_has_role
                .saturating_add(T::DbWeight::get().reads(2))
        ]
        fn apply_as(origin, org_id: <T::Lookup as StaticLookup>::Source, call: Box<<T as Trait>::Call>) {
            let target_org_id = T::Lookup::lookup(org_id)?;
            RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::apply_as_organization(&target_org_id))?;

            let res = call.dispatch(frame_system::RawOrigin::Signed(target_org_id.clone()).into());
            Self::deposit_event(RawEvent::OrganizationExecuted(target_org_id, res.map(|_| ()).map_err(|e| e.error)));
        }

        /// Mutate an organization to use the new parameters. Only an organization can call this on itself.
        #[weight = T::WeightInfo::mutate(new_details.executors.len() as u32, T::MaxExecutors::get())]
        fn mutate(origin, new_details: OrganizationDetailsOf<T>) -> DispatchResultWithPostInfo {
            let (org_id, old_details) = Self::ensure_org(origin)?;

            // Make sure everything is sorted for optimization purposes
            let mut new_details = new_details;
            new_details.sort();

            let mut roles_granted: u32 = 0;
            let mut roles_revoked: u32 = 0;
            Self::try_run_on_changes(old_details.executors.as_slice(), new_details.executors.as_slice(), |old_account| {
                roles_revoked = roles_revoked.saturating_add(1);
                RoleManagerOf::<T>::revoke_role(Some(old_account), RoleBuilderOf::<T>::apply_as_organization(&org_id))
            }, |new_account| {
                roles_granted = roles_granted.saturating_add(1);
                RoleManagerOf::<T>::grant_role(Some(new_account), RoleBuilderOf::<T>::apply_as_organization(&org_id))
            })?;
            Parameters::<T>::insert(&org_id, new_details.clone());

            Self::deposit_event(RawEvent::OrganizationMutated(org_id, old_details, new_details));

            Ok(Some(T::WeightInfo::mutate(roles_granted, roles_revoked)).into())
        }

        /// Create a proposal for a given organization
        #[weight = T::WeightInfo::create_proposal()]
        fn create_proposal(origin, org_id: <T::Lookup as StaticLookup>::Source, call: Box<<T as Trait>::Call>) {
            let _who = ensure_signed(origin)?;
            let target_org_id = T::Lookup::lookup(org_id)?;
            let details = Self::try_get_parameters(&target_org_id)?;
            let proposal_id = Self::proposal_id(&target_org_id, call.clone());
            if Proposals::<T>::contains_key(proposal_id) {
                return Err(Error::<T>::ProposalDuplicate.into());
            }

            T::VotingRouter::initiate(details.voting.0.clone(), proposal_id, details.voting.1)?;

            Proposals::<T>::insert(&proposal_id, Proposal{
                org: target_org_id.clone(),
                call: call.encode(),
                // Not only does this save us future read weights but it also cover
                // the case where an org change voting systems but still has pending
                // proposals.
                voting: details.voting.0,
            });

            Self::deposit_event(RawEvent::ProposalSubmitted(target_org_id, proposal_id));
        }

        /// Remove a proposal from the batch of active ones. Has to be called by the organization itself,
        /// typically this could come from an 'apply_as' or a separate vote.
        #[weight = T::WeightInfo::veto_proposal(T::MaxVotes::get(), T::MaxVotes::get())]
        fn veto_proposal(origin, proposal_id: ProposalIdOf<T>) {
            let (org_id, _details) = Self::ensure_org(origin)?;
            let proposal = Self::try_get_proposal(proposal_id)?;
            ensure!(proposal.org == org_id, Error::<T>::ProposalNotForOrganization);

            T::VotingRouter::veto(proposal.voting, proposal_id)?;
            Proposals::<T>::remove(proposal_id);

            Self::deposit_event(RawEvent::ProposalVetoed(proposal_id));
        }

        /// Vote for or against a given proposal. The caller can choose how much voting power is dedicated
        /// to it via the `power` parameter.
        #[weight = T::WeightInfo::decide_on_proposal(T::MaxVotes::get())]
        fn decide_on_proposal(origin, proposal_id: ProposalIdOf<T>, vote_data: VoteDataOf<T>) {
            let voter = ensure_signed(origin)?;
            let proposal = Self::try_get_proposal(proposal_id)?;

            T::VotingRouter::vote(proposal.voting, proposal_id, &voter, vote_data.clone())?;
            Self::deposit_event(RawEvent::ProposalVoteCasted(proposal_id, voter, vote_data));
        }

        /// If a proposal passed or failed but is not longer awaiting or waiting for votes it can be closed. Closing
        /// a proposal means executing it if it passed, freeing all funds locked and erasing it from the local storage.
        /// `proposal_weight_bound` has to be at least equal to the weight of the call that will be executed would the
        /// proposal pass.
        #[weight = T::WeightInfo::close_proposal(T::MaxVotes::get(), T::MaxVotes::get()).saturating_add(*proposal_weight_bound)]
        fn close_proposal(origin, proposal_id: ProposalIdOf<T>, proposal_weight_bound: Weight) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            let proposal = Self::try_get_proposal(proposal_id)?;
            let decoded_call = <T as Trait>::Call::decode(&mut &proposal.clone().call[..]).map_err(|_| Error::<T>::ProposalDecodingFailure)?;
            let decoded_call_weight = decoded_call.get_dispatch_info().weight;
            ensure!(proposal_weight_bound >= decoded_call_weight, Error::<T>::TooSmallWeightBound);

            let proposal_result = T::VotingRouter::close(proposal.voting.clone(), proposal_id)?;

            let mut external_weight: Weight = 0;
            if proposal_result == ProposalResult::Passing {
                let res = decoded_call.dispatch(frame_system::RawOrigin::Signed(proposal.clone().org).into());
                Self::deposit_event(RawEvent::ProposalExecuted(proposal_id, res.map(|_| ()).map_err(|e| e.error)));
                external_weight = external_weight.saturating_add(Self::get_result_weight(res).unwrap_or(decoded_call_weight));
            }
            Proposals::<T>::remove(proposal_id);

            Self::deposit_event(RawEvent::ProposalClosed(proposal_id, proposal_result));

            Ok(Some(T::WeightInfo::close_proposal(T::MaxVotes::get(), T::MaxVotes::get()).saturating_add(external_weight)).into())
        }
    }
}

impl<T: Trait> Module<T> {
    /// A handy helper functions to run two functions on what elements were removed from a
    /// slice and on the added elements. Should be pretty useful when trying to limit
    /// database read and writes since we execute the functions only on the changed elements.
    fn try_run_on_changes<Elem: Ord>(
        old_vec: &[Elem],
        new_vec: &[Elem],
        on_old: impl FnMut(&Elem) -> DispatchResult,
        on_new: impl FnMut(&Elem) -> DispatchResult,
    ) -> DispatchResult {
        Self::try_run_if_not_in_right(old_vec, new_vec, on_old)
            .and_then(|_| Self::try_run_if_not_in_right(new_vec, old_vec, on_new))
    }

    /// Run `to_run` on every elent that is in `left` but not in `right`.
    fn try_run_if_not_in_right<Elem: Ord>(
        left: &[Elem],
        right: &[Elem],
        mut to_run: impl FnMut(&Elem) -> DispatchResult,
    ) -> DispatchResult {
        left.into_iter().try_for_each(|elem| {
            if right.binary_search(&elem).is_err() {
                return to_run(elem);
            }

            Ok(())
        })
    }

    /// Given any counter return the associated organization id
    pub fn org_id_for(counter: u32) -> T::AccountId {
        ORGS_MODULE_ID.into_sub_account(counter)
    }

    /// Makes sure that the `origin` is a registered organization
    fn ensure_org(
        origin: T::Origin,
    ) -> Result<(T::AccountId, OrganizationDetailsOf<T>), DispatchError> {
        match ensure_signed(origin) {
            Err(e) => Err(e.into()),
            Ok(maybe_org_id) => Ok((
                maybe_org_id.clone(),
                Self::try_get_parameters(&maybe_org_id)?,
            )),
        }
    }

    /// Fetch an org details or error
    fn try_get_parameters(
        org_id: &T::AccountId,
    ) -> Result<OrganizationDetailsOf<T>, DispatchError> {
        match Parameters::<T>::get(org_id) {
            Some(details) => Ok(details),
            None => Err(Error::<T>::NotAnOrganization.into()),
        }
    }

    /// Fetch a proposal details or error
    fn try_get_proposal(proposal_id: ProposalIdOf<T>) -> Result<ProposalOf<T>, DispatchError> {
        match Proposals::<T>::get(proposal_id) {
            Some(proposal) => Ok(proposal),
            None => Err(Error::<T>::ProposalNotFound.into()),
        }
    }

    /// Hash the block number, org id and proposal together to generate its id
    fn proposal_id(org_id: &T::AccountId, proposal: Box<<T as Trait>::Call>) -> ProposalIdOf<T> {
        // Proposals are organization specific and there can not be two identical proposals opened
        // in the same organization.
        T::Hashing::hash_of(&[org_id.encode(), proposal.encode()])
    }

    /// Return the weight of a dispatch call result as an `Option`.
    ///
    /// Will return the weight regardless of what the state of the result is.
    fn get_result_weight(result: DispatchResultWithPostInfo) -> Option<Weight> {
        match result {
            Ok(post_info) => post_info.actual_weight,
            Err(err) => err.post_info.actual_weight,
        }
    }

    /// Generate an organization id and use `do_create_with_id` to actually create the org.
    fn do_create(details: OrganizationDetailsOf<T>) -> DispatchResult {
        let counter = Self::counter();
        let new_counter = counter.checked_add(1).ok_or(Error::<T>::CounterOverflow)?;
        let org_id = Self::org_id_for(counter);

        Self::do_create_with_id(details, org_id)?;
        // We only increment the counter after the do_create_with_id as it may error early in
        // which case we do not want to write to our storage.
        Counter::put(new_counter);

        Ok(())
    }

    /// Alternative to do_create where the caller set the org_id themselves. Mostly used
    /// for benchmarks where we want the `org_id` to be whitelisted.
    fn do_create_with_id(
        details: OrganizationDetailsOf<T>,
        org_id: T::AccountId,
    ) -> DispatchResult {
        // Sorting details allows us to be more efficient when updating them later on.
        let mut details = details;
        details.sort();

        // We first write the counter so that even if the calls below fail we will always regenerate a new and
        // different organization id.
        details.executors.iter().try_for_each(|account| {
            RoleManagerOf::<T>::grant_role(
                Some(&account),
                RoleBuilderOf::<T>::apply_as_organization(&org_id),
            )
        })?;
        Parameters::<T>::insert(&org_id, details.clone());

        Self::deposit_event(RawEvent::OrganizationCreated(org_id, details));
        Ok(())
    }
}
