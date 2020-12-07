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
use governance_os_support::{benchmarking::SEED, traits:Currencies};
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

benchmarks! {
    _ { }

    create {
        let token_id: T::CurrencyId = T::CurrencyId::default();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()), token_id, true)
    verify {
        assert_eq!(TotalIssuances::<T>::contains_key(token_id), true);
    }

    mint {
        let token_id: T::CurrencyId = T::CurrencyId::default();
        let coins_to_mint: T::Balance = 10_000_000.into();
        let caller: T::AccountId = whitelisted_caller();

        let _ = Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), token_id, true);

        let to: T::AccountId = account("to", 0, SEED);
        let to_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(to.clone());
    }: _(RawOrigin::Signed(caller.clone()), token_id, to_lookup, coins_to_mint)
    verify {
        assert_eq!(<Module<T> as Currencies<T::AccountId>>::free_balance(token_id, &to), coins_to_mint);
        assert_eq!(TotalIssuances::<T>::get(token_id), coins_to_mint);
    }

    burn {
        let token_id: T::CurrencyId = T::CurrencyId::default();
        let coins_to_burn: T::Balance = 10_000_000.into();
        let caller: T::AccountId = whitelisted_caller();

        let to: T::AccountId = account("to", 0, SEED);
        let to_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(to.clone());

        let _ = Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), token_id, true);
        let _ = <Module<T> as Currencies<T::AccountId>>::mint(token_id, &to, coins_to_burn);
    }: _(RawOrigin::Signed(caller.clone()), token_id, to_lookup, coins_to_burn)
    verify {
        assert_eq!(<Module<T> as Currencies<T::AccountId>>::free_balance(token_id, &to), 0.into());
        assert_eq!(TotalIssuances::<T>::get(token_id), 0.into());
    }

    update_details {
        let token_id: T::CurrencyId = T::CurrencyId::default();
        let caller: T::AccountId = whitelisted_caller();

        let _ = Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), token_id, true);

        // Worst case scenario: owner is changed
        let new_owner: T::AccountId = account("owner", 0, SEED);
        let new_details = CurrencyDetails {
            owner: new_owner.clone(),
            transferable: true,
        };
    }: _(RawOrigin::Signed(caller.clone()), token_id, new_details)
    verify {
        assert_eq!(RoleManagerOf::<T>::has_role(&new_owner, RoleBuilderOf::<T>::manage_currency(token_id)), true);
    }

    transfer {
        let token_id: T::CurrencyId = T::CurrencyId::default();
        let coins_to_transfer: T::Balance = 10_000_000.into();
        let caller: T::AccountId = whitelisted_caller();

        let _ = Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), token_id, true);
        let _ = Module::<T>::set_free_balance(token_id, &caller, coins_to_transfer);

        let to: T::AccountId = account("to", 0, SEED);
        let to_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(to.clone());
    }: _(RawOrigin::Signed(caller.clone()), token_id, to_lookup, coins_to_transfer)
    verify {
        assert_eq!(<Module<T> as Currencies<T::AccountId>>::free_balance(token_id, &caller), 0.into());
        assert_eq!(<Module<T> as Currencies<T::AccountId>>::free_balance(token_id, &to), coins_to_transfer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::mock::{ExtBuilder, Test};
    use frame_support::assert_ok;
    use governance_os_support::create_benchmarking_test;

    fn new_test_ext() -> sp_io::TestExternalities {
        ExtBuilder::default().one_hundred_for_alice_n_bob().build()
    }

    create_benchmarking_test!(new_test_ext, Test, create, test_benchmark_create);
    create_benchmarking_test!(new_test_ext, Test, mint, test_benchmark_mint);
    create_benchmarking_test!(new_test_ext, Test, burn, test_benchmark_burn);
    create_benchmarking_test!(
        new_test_ext,
        Test,
        update_details,
        test_benchmark_update_details
    );
    create_benchmarking_test!(new_test_ext, Test, transfer, test_benchmark_transfer);
}
