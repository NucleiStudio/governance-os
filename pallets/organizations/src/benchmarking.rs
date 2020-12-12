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

//! Benchmarks for the `organizations` pallet. While we tried to keep those generic
//! and adaptable to any eventual runtime defined voting systems we'd expect that for
//! less common implementations one would have to fine tune those.

use crate::*;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::weights::GetDispatchInfo;
use frame_system::RawOrigin;
use governance_os_support::{
    benchmarking::{advance_blocks, SEED},
    traits::{Currencies, RoleManager},
};
use sp_runtime::traits::StaticLookup;
use sp_std::{boxed::Box, prelude::*};

fn make_details_with_executors<T: Trait>(b: u32, offset: u32) -> OrganizationDetailsOf<T> {
    OrganizationDetails {
        executors: (0..b)
            .map(|i| account("executor", i + offset, SEED))
            .collect::<Vec<T::AccountId>>(),
        voting: T::VotingSystem::default(),
    }
}

/// Create and register an org before returning its ID. The ID is non whitelisted
/// so if you plan to trigger calls from it maybe create the org manually.
fn setup_org<T: Trait>() -> (T::AccountId, <T::Lookup as StaticLookup>::Source) {
    let details = make_details_with_executors::<T>(0, 0);
    drop(Module::<T>::do_create(details));
    let org_id = Module::<T>::org_id_for(Module::<T>::counter() - 1);

    (org_id.clone(), T::Lookup::unlookup(org_id))
}

fn make_proposal<T: Trait>(org_id: &T::AccountId) -> (Box<<T as Trait>::Call>, ProposalIdOf<T>) {
    let proposal: Box<<T as Trait>::Call> =
        Box::new(frame_system::Call::<T>::remark((1..100).collect::<Vec<_>>()).into());
    (proposal.clone(), Module::<T>::proposal_id(org_id, proposal))
}

fn set_free_balance<T: Trait>(target: &T::AccountId, balance: BalanceOf<T>) {
    drop(
        <<T as Trait>::Currencies as Currencies<<T as frame_system::Trait>::AccountId>>::mint(
            <<<T as Trait>::Currencies as Currencies<<T as frame_system::Trait>::AccountId>>::CurrencyId>::default(),
            target,
            balance,
        )
    );
}

fn setup_votes<T: Trait>(proposal_id: ProposalIdOf<T>, in_favor: bool, nb_votes: u32) {
    for i in 0..nb_votes {
        let voter: T::AccountId = match in_favor {
            true => account("favorable_voter", i, SEED),
            false => account("against_voter", i, SEED),
        };
        // Give voter some coins. We assume we need to use the default currency
        set_free_balance::<T>(&voter, 10.into());
        drop(Module::<T>::decide_on_proposal(
            RawOrigin::Signed(voter).into(),
            proposal_id,
            10.into(), // make sure we vote with some cash so that we avoid optimizations
            in_favor,
        ));
    }
}

benchmarks! {
    _ { }

    create {
        let b in 0 .. 100; // number of executors in the details

        let details = make_details_with_executors::<T>(b, 0);
        let caller: T::AccountId = whitelisted_caller();
        drop(T::RoleManager::grant_role(Some(&caller), <T as Trait>::RoleBuilder::create_organizations()));
    }: _(RawOrigin::Signed(caller), details.clone())
    verify {
        let org_id = Module::<T>::org_id_for(Module::<T>::counter()-1);
        let mut verify_details = details;
        verify_details.sort();
        assert_eq!(Module::<T>::parameters(org_id).unwrap(), verify_details);
    }

    mutate {
        let b in 0..100; // number of executors in the new details - get their roles granted
        let c in 0..T::MaxExecutors::get(); // number of executors in the old details - get their roles revoked

        let old_details = make_details_with_executors::<T>(c, 0);
        let new_details = make_details_with_executors::<T>(b, c);
        let org_id: T::AccountId = whitelisted_caller();
        drop(Module::<T>::do_create_with_id(old_details, org_id.clone()));
    }: _(RawOrigin::Signed(org_id.clone()), new_details.clone())
    verify {
        let mut verify_details = new_details;
        verify_details.sort();
        assert_eq!(Module::<T>::parameters(org_id).unwrap(), verify_details);
    }

    create_proposal {
        let (org_id, org_id_lookup) = setup_org::<T>();
        let (proposal, proposal_id) = make_proposal::<T>(&org_id);
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), org_id_lookup, proposal)
    verify {
        assert!(Proposals::<T>::contains_key(proposal_id));
    }

    veto_proposal {
        let b in 0..T::MaxVotes::get(); // number of votes casted in favor of the proposal
        let c in 0..T::MaxVotes::get(); // number of votes casted against the proposal

        let details = make_details_with_executors::<T>(0, 0);
        let org_id: T::AccountId = whitelisted_caller();
        let org_id_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(org_id.clone());
        drop(Module::<T>::do_create_with_id(details, org_id.clone()));

        let (proposal, proposal_id) = make_proposal::<T>(&org_id);
        drop(Module::<T>::create_proposal(RawOrigin::Signed(whitelisted_caller()).into(), org_id_lookup, proposal));
        setup_votes::<T>(proposal_id, true, b);
        setup_votes::<T>(proposal_id, false, c);
    }: _(RawOrigin::Signed(org_id), proposal_id)
    verify {
        assert!(!Proposals::<T>::contains_key(proposal_id));
    }

    decide_on_proposal_favorable {
        let b in 0..T::MaxVotes::get(); // number of votes casted in favor of the proposal
        let c in 0..T::MaxVotes::get(); // number of votes casted against the proposal

        let (org_id, org_id_lookup) = setup_org::<T>();
        let (proposal, proposal_id) = make_proposal::<T>(&org_id);
        let caller: T::AccountId = whitelisted_caller();
        drop(Module::<T>::create_proposal(RawOrigin::Signed(caller.clone()).into(), org_id_lookup, proposal));
        setup_votes::<T>(proposal_id, true, b);
        setup_votes::<T>(proposal_id, false, c);
        set_free_balance::<T>(&caller, 10.into());

        let proposal_before_vote = Proposals::<T>::get(proposal_id).unwrap();
    }: decide_on_proposal(RawOrigin::Signed(caller), proposal_id, 10.into(), true)
    verify {
        let proposal_after_vote = Proposals::<T>::get(proposal_id).unwrap();
        assert!(proposal_before_vote != proposal_after_vote);
    }

    decide_on_proposal_against {
        let b in 0..T::MaxVotes::get(); // number of votes casted in favor of the proposal
        let c in 0..T::MaxVotes::get(); // number of votes casted against the proposal

        let (org_id, org_id_lookup) = setup_org::<T>();
        let (proposal, proposal_id) = make_proposal::<T>(&org_id);
        let caller: T::AccountId = whitelisted_caller();
        drop(Module::<T>::create_proposal(RawOrigin::Signed(caller.clone()).into(), org_id_lookup, proposal));
        setup_votes::<T>(proposal_id, true, b);
        setup_votes::<T>(proposal_id, false, c);
        set_free_balance::<T>(&caller, 10.into());

        let proposal_before_vote = Proposals::<T>::get(proposal_id).unwrap();
    }: decide_on_proposal(RawOrigin::Signed(caller), proposal_id, 10.into(), false)
    verify {
        let proposal_after_vote = Proposals::<T>::get(proposal_id).unwrap();
        assert!(proposal_before_vote != proposal_after_vote);
    }

    close_proposal {
        let b in 0..T::MaxVotes::get(); // number of votes casted in favor of the proposal
        let c in 0..T::MaxVotes::get(); // number of votes casted against the proposal

        let (org_id, org_id_lookup) = setup_org::<T>();
        let (proposal, proposal_id) = make_proposal::<T>(&org_id);
        let caller: T::AccountId = whitelisted_caller();
        drop(Module::<T>::create_proposal(RawOrigin::Signed(caller.clone()).into(), org_id_lookup, proposal.clone()));
        setup_votes::<T>(proposal_id, true, b);
        setup_votes::<T>(proposal_id, false, c);
        advance_blocks::<T>(1.into()); // Skip any eventual ttl which would be set to 0 by `Default`
    }: _(RawOrigin::Signed(caller), proposal_id, proposal.get_dispatch_info().weight)
    verify {
        assert!(!Proposals::<T>::contains_key(proposal_id));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::mock::{ExtBuilder, Test};
    use frame_support::assert_ok;
    use governance_os_support::create_benchmarking_test;

    fn new_test_ext() -> sp_io::TestExternalities {
        ExtBuilder::default().build()
    }

    create_benchmarking_test!(new_test_ext, Test, create, test_benchmark_create);
    create_benchmarking_test!(new_test_ext, Test, mutate, test_benchmark_mutate);
    create_benchmarking_test!(
        new_test_ext,
        Test,
        create_proposal,
        test_benchmark_create_proposal
    );
    create_benchmarking_test!(
        new_test_ext,
        Test,
        veto_proposal,
        test_benchmark_veto_proposal
    );
    create_benchmarking_test!(
        new_test_ext,
        Test,
        decide_on_proposal_favorable,
        test_benchmark_decide_on_proposal_favorable
    );
    create_benchmarking_test!(
        new_test_ext,
        Test,
        decide_on_proposal_against,
        test_benchmark_decide_on_proposal_against
    );
    create_benchmarking_test!(
        new_test_ext,
        Test,
        close_proposal,
        test_benchmark_close_proposal
    );
}
