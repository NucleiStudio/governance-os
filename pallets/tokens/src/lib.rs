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

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, Parameter,
};
use frame_system::ensure_signed;
use governance_os_support::Currencies;
use pallet_balances::AccountData;
use sp_runtime::traits::{
    AtLeast32BitUnsigned, CheckedAdd, MaybeSerializeDeserialize, Member, StaticLookup,
};
use sp_std::cmp::{Eq, PartialEq};

#[cfg(feature = "std")]
use sp_std::collections::btree_map::BTreeMap;

#[cfg(test)]
mod tests;

mod adapter;
mod currencies;
mod details;
mod imbalances;

pub use adapter::NativeCurrencyAdapter;
pub use details::CurrencyDetails;

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
        pub Details get(fn details): map hasher(blake2_128_concat) T::CurrencyId => CurrencyDetails<T::AccountId>;
    }
    add_extra_genesis {
        config(endowed_accounts): Vec<(T::CurrencyId, T::AccountId, T::Balance)>;
        config(currency_details): Vec<(T::CurrencyId, CurrencyDetails<T::AccountId>)>;
        build(|config: &GenesisConfig<T>| {
            config.endowed_accounts.iter().for_each(|(currency_id, account_id, initial_balance)| {
                <Balances<T>>::mutate(account_id, currency_id, |account_data| account_data.free = *initial_balance)
            });

            config.currency_details.iter().for_each(|(currency_id, details)| {
                <Details<T>>::mutate(currency_id, |det| *det = details.clone())
            });
        })
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Balance = <T as Trait>::Balance,
        CurrencyId = <T as Trait>::CurrencyId,
    {
        /// A new currency has been created. [currency id, owner]
        CurrencyCreated(CurrencyId, AccountId),
        /// Some units of currency were issued. [currency_id, dest, amount]
        CurrencyMinted(CurrencyId, AccountId, Balance),
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
        /// The currency ID is already used by another currency
        CurrencyAlreadyExists,
        /// This call an only be used by the currency owner
        NotCurrencyOwner,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        /// Creates a new currency with 0 units, to issue units to people one would have to call
        /// `issue`. This will register the caller of this dispatchable as the owner of the currency
        /// so they can issue or burn units. This will produce an error if `currency_id` is already
        /// used by another currency.
        #[weight = 0]
        pub fn create(origin, currency_id: T::CurrencyId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(!Details::<T>::contains_key(currency_id), Error::<T>::CurrencyAlreadyExists);

            Details::<T>::mutate(currency_id, |details| *details = CurrencyDetails {
                owner: who.clone(),
            });

            Self::deposit_event(RawEvent::CurrencyCreated(currency_id, who));
            Ok(())
        }

        /// Issue some units of the currency identified by `currency_id` and credit them to `dest`.
        /// Can only be called by the owner of the currency.
        #[weight = 0]
        pub fn mint(origin, currency_id: T::CurrencyId, dest: <T::Lookup as StaticLookup>::Source, amount: T::Balance) -> DispatchResult {
            Self::ensure_owner_of_currency(origin, currency_id)?;
            let to = T::Lookup::lookup(dest)?;
            <Self as Currencies<T::AccountId>>::mint(currency_id, &to, amount)?;

            Self::deposit_event(RawEvent::CurrencyMinted(currency_id, to, amount));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    /// Set the free balance of `who` in `currency_id`. You are supposed to update the total
    /// issuance yourself.
    fn set_free_balance(currency_id: T::CurrencyId, who: &T::AccountId, balance: T::Balance) {
        <Balances<T>>::mutate(who, currency_id, |account_data| account_data.free = balance);
    }

    /// Make sure that `origin` is the owner of  `currency_id`.
    fn ensure_owner_of_currency(origin: T::Origin, currency_id: T::CurrencyId) -> DispatchResult {
        let sender = ensure_signed(origin)?;
        ensure!(
            Details::<T>::get(currency_id).owner == sender,
            Error::<T>::NotCurrencyOwner
        );

        Ok(())
    }
}
