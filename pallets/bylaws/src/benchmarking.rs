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
use governance_os_support::benchmarking::SEED;
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

fn prepare_benchmark<T: Trait>() -> (T::AccountId, <T::Lookup as StaticLookup>::Source, T::Role) {
    let target: T::AccountId = account("target", 0, SEED);
    let target_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(target.clone());
    let role = T::Role::default();

    (target, target_lookup, role)
}

benchmarks! {
    _ { }

    grant_role {
        let (target, target_lookup, role) = prepare_benchmark::<T>();
    }: _(RawOrigin::Root, Some(target_lookup), role)
    verify {
        assert_eq!(Module::<T>::has_role(&target, role), true);
    }

    revoke_role {
        let (target, target_lookup, role) = prepare_benchmark::<T>();
        Module::<T>::set_role(Some(&target), role);
    }: _(RawOrigin::Root, Some(target_lookup), role)
    verify {
        assert_eq!(Module::<T>::has_role(&target, role), false);
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
