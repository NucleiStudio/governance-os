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
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use governance_os_support::benchmarking::SEED;
use sp_runtime::traits::StaticLookup;
use sp_std::{
    convert::{TryFrom, TryInto},
    prelude::*,
};

fn prepare_benchmark<T: Trait>(
    b: u32,
) -> Result<
    (
        T::AccountId,
        T::AccountId,
        <T::Lookup as StaticLookup>::Source,
    ),
    &'static str,
> {
    let caller: T::AccountId = whitelisted_caller();
    let target: T::AccountId = account("target", 0, SEED);
    let target_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(target.clone());

    for _ in 0..b {
        Module::<T>::add_bylaw(
            RawOrigin::Signed(caller.clone()).into(),
            target_lookup.clone(),
            T::Tag::default(),
            T::Bylaw::default(),
        )?;
    }

    Ok((caller, target, target_lookup))
}

benchmarks! {
    _ { }

    add_bylaw {
        let b in 0 .. (T::MaxBylaws::get() - 1);
        let (caller, target, target_lookup) = prepare_benchmark::<T>(b)?;
    }: _(RawOrigin::Signed(caller.clone()), target_lookup, T::Tag::default(), T::Bylaw::default())
    verify {
        assert_eq!(Bylaws::<T>::get(&target).len(), usize::try_from(b + 1).unwrap());
    }

    remove_bylaw {
        let b in 1 .. T::MaxBylaws::get();
        let (caller, target, target_lookup) = prepare_benchmark::<T>(b)?;
    }: _(RawOrigin::Signed(caller.clone()), target_lookup, T::Tag::default(), T::Bylaw::default())
    verify {
        assert_eq!(Bylaws::<T>::get(&target).len(), usize::try_from(b - 1).unwrap());
    }

    reset_bylaws {
        let b in 0 .. T::MaxBylaws::get();
        let (caller, target, target_lookup) = prepare_benchmark::<T>(b)?;
    }: _(RawOrigin::Signed(caller.clone()), target_lookup)
    verify {
        assert_eq!(Bylaws::<T>::get(&target).len(), usize::try_from(0).unwrap());
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

    create_benchmarking_test!(new_test_ext, Test, add_bylaw, test_benchmark_add_bylaw);
    create_benchmarking_test!(
        new_test_ext,
        Test,
        remove_bylaw,
        test_benchmark_remove_bylaw
    );
    create_benchmarking_test!(
        new_test_ext,
        Test,
        reset_bylaws,
        test_benchmark_reset_bylaws
    );
}
