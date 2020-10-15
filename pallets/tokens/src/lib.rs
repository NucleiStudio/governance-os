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

//! This pallet implements the code to support a multi currency runtime.
//! Along with compatibility with the `Currency` trait through the use
//! of `NativeCurrencyAdapter`.
//! Caveats:
//! - for now, we do not support `reasons` and `existence_requirements`
//! - for now, we do not support `ExistentialDeposit`
//! - for now, we do not support locking or reserving funds

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_error, decl_event, decl_module, decl_storage, Parameter};
use pallet_balances::AccountData;
use sp_runtime::traits::{AtLeast32BitUnsigned, CheckedAdd, MaybeSerializeDeserialize, Member};
use sp_std::cmp::{Eq, PartialEq};

#[cfg(feature = "std")]
use sp_std::collections::btree_map::BTreeMap;

#[cfg(test)]
mod tests;

mod adapter;
mod currencies;
mod imbalances;

pub use adapter::NativeCurrencyAdapter;

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// The type used to identify currencies
    type CurrencyId: Parameter + Member + Copy + MaybeSerializeDeserialize + Ord;

    /// The balance of an account.
    type Balance: Parameter
        + Member
        + AtLeast32BitUnsigned
        + Default
        + Copy
        + MaybeSerializeDeserialize;
}

decl_storage! {
    trait Store for Module<T: Trait> as Tokens {
        pub TotalIssuances get(fn total_issuances) build(|config: &GenesisConfig<T>| {
            config
                .endowed_accounts
                .iter()
                .map(|(currency_id, _, initial_balance)| (currency_id, initial_balance))
                .fold(BTreeMap::<T::CurrencyId, T::Balance>::new(), |mut acc, (currency_id, initial_balance)| {
                    if let Some(issuance) = acc.get_mut(currency_id) {
                        *issuance = issuance.checked_add(initial_balance).expect("total issuance cannot overflow when building genesis");
                    } else {
                        acc.insert(*currency_id, *initial_balance);
                    }
                    acc
                })
                .into_iter()
                .collect::<Vec<_>>()
        }): map hasher(blake2_128_concat) T::CurrencyId => T::Balance;
        pub Balances get(fn balances): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) T::CurrencyId => AccountData<T::Balance>;
    }
    add_extra_genesis {
        config(endowed_accounts): Vec<(T::CurrencyId, T::AccountId, T::Balance)>;
        build(|config: &GenesisConfig<T>| {
            config.endowed_accounts.iter().for_each(|(currency_id, account_id, initial_balance)| {
                <Balances<T>>::mutate(account_id, currency_id, |account_data| account_data.free = *initial_balance)
            })
        })
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// This operation will cause total issuance to overflow for the given currency
        TotalIssuanceOverflow,
        /// This operation will cause total issuance to underflow for the given currency
        TotalIssuanceUnderflow,
        /// This operation will cause the balance of an account to overflow for the
        /// given currency
        BalanceOverflow,
        /// There are not enough coins inside the balance of the user to perform the action
        BalanceTooLow,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        //type Error = Error<T>;

        fn deposit_event() = default;

        // /// An example dispatchable that takes a singles value as a parameter, writes the value to
        // /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        // #[weight = 10_000 + T::DbWeight::get().writes(1)]
        // pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
        //     // Check that the extrinsic was signed and get the signer.
        //     // This function will return an error if the extrinsic is not signed.
        //     // https://substrate.dev/docs/en/knowledgebase/runtime/origin
        //     let who = ensure_signed(origin)?;

        //     // Update storage.
        //     Something::put(something);

        //     // Emit an event.
        //     Self::deposit_event(RawEvent::SomethingStored(something, who));
        //     // Return a successful DispatchResult
        //     Ok(())
        // }

        // /// An example dispatchable that may throw a custom error.
        // #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        // pub fn cause_error(origin) -> dispatch::DispatchResult {
        //     let _who = ensure_signed(origin)?;

        //     // Read a value from storage.
        //     match Something::get() {
        //         // Return an error if the value has not been set.
        //         None => Err(Error::<T>::NoneValue)?,
        //         Some(old) => {
        //             // Increment the value read from storage; will error in the event of overflow.
        //             let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
        //             // Update the value in storage with the incremented result.
        //             Something::put(new);
        //             Ok(())
        //         },
        //     }
        // }
    }
}

impl<T: Trait> Module<T> {
    /// Set the free balance of `who` in `currency_id`. You are supposed to update the total
    /// issuance yourself.
    fn set_free_balance(currency_id: T::CurrencyId, who: &T::AccountId, balance: T::Balance) {
        <Balances<T>>::mutate(who, currency_id, |account_data| account_data.free = balance);
    }
}
