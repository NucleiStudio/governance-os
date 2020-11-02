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

use crate::*;
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;
use governance_os_support::{acl::RoleManager, benchmarking::SEED};
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

fn prepare_benchmark<T: Trait>(
    b: u32,
) -> (T::AccountId, <T::Lookup as StaticLookup>::Source, T::Role) {
    let target: T::AccountId = account("target", b, SEED);
    let target_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(target.clone());
    let role = T::Role::default();

    for _ in 0..b {
        // `grant_role` would avoid duplicates so we have set this manually. Note that for those
        // benchmarks to be relevant `T::RootRole::get()` MUST be different from `T::Role::default()`.
        // We are simply trying to force the maximum number of iterations (worst case scenario).
        Roles::<T>::mutate(Some(&target), |v| v.push(T::RootRole::get()));
    }

    (target, target_lookup, role)
}

benchmarks! {
    _ { }

    grant_role {
        let b in 0 .. T::MaxRoles::get();

        let (target, target_lookup, role) = prepare_benchmark::<T>(b);
    }: _(RawOrigin::Root, Some(target_lookup), role)
    verify {
        assert_eq!(Roles::<T>::get(Some(&target)).iter().any(|&r| r==role), true);
    }

    revoke_role {
        let b in 1 .. T::MaxRoles::get();

        let (target, target_lookup, role) = prepare_benchmark::<T>(b - 1);
        drop(<Module<T> as RoleManager>::grant_role(Some(&target), role));
    }: _(RawOrigin::Root, Some(target_lookup), role)
    verify {
        assert_eq!(Roles::<T>::get(Some(&target)).iter().any(|&r| r==role), false);
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

    create_benchmarking_test!(new_test_ext, Test, grant_role, test_benchmark_grant_role);
    create_benchmarking_test!(new_test_ext, Test, revoke_role, test_benchmark_revoke_role);
}
