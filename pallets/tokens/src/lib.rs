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
    decl_error, decl_event, decl_module, decl_storage, weights::Weight, Parameter,
};
use frame_system::ensure_signed;
use governance_os_support::traits::{Currencies, RoleManager};
use sp_runtime::{
    traits::{
        AtLeast32BitUnsigned, CheckedAdd, MaybeSerializeDeserialize, Member, StaticLookup, Zero,
    },
    DispatchResult,
};
use sp_std::cmp::{Eq, PartialEq};

#[cfg(feature = "std")]
use sp_std::collections::btree_map::BTreeMap;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod tests;

mod account_data;
mod adapter;
mod currencies;
mod default_weights;
mod details;
mod imbalances;
mod mutations;

pub use account_data::AccountCurrencyData;
pub use adapter::NativeCurrencyAdapter;
pub use details::CurrencyDetails;
pub use imbalances::{NegativeImbalance, PositiveImbalance};

pub trait WeightInfo {
    fn create() -> Weight;
    fn mint() -> Weight;
    fn burn() -> Weight;
    fn update_details() -> Weight;
    fn transfer() -> Weight;
}

pub trait RoleBuilder {
    type CurrencyId;
    type Role;

    /// Role for transferring certain units of currencies. This is used to create
    /// non transferrable assets or could be used for creating permissioned assets
    /// in the future.
    fn transfer_currency(id: Self::CurrencyId) -> Self::Role;

    /// Role for the account(s) that are allowed to `mint` or `burn` units of currency.
    fn manage_currency(id: Self::CurrencyId) -> Self::Role;

    /// Role for creating currencies.
    fn create_currencies() -> Self::Role;
}

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// The type used to identify currencies
    type CurrencyId: Parameter + Member + Copy + MaybeSerializeDeserialize + Ord + Default;

    /// The balance of an account.
    type Balance: Parameter
        + Member
        + AtLeast32BitUnsigned
        + Default
        + Copy
        + MaybeSerializeDeserialize;

    /// Weight values for this pallet
    type WeightInfo: WeightInfo;

    /// Pallet that is in charge of managing the roles based ACL.
    type RoleManager: RoleManager<AccountId = Self::AccountId>;

    /// This pallet relies on roles associated to a specific metadata so we need the runtime
    /// to provide some helper functions to build those so that we can keep the role definition
    /// code modular.
    type RoleBuilder: RoleBuilder<
        CurrencyId = Self::CurrencyId,
        Role = <RoleManagerOf<Self> as RoleManager>::Role,
    >;
}

type RoleBuilderOf<T> = <T as Trait>::RoleBuilder;
type RoleManagerOf<T> = <T as Trait>::RoleManager;

decl_storage! {
    trait Store for Module<T: Trait> as Tokens {
        /// Store the balances holded by an account. By storing the balances under an account (VS storing
        /// the accounts under the currency ids) we can enumerate the tokens holded by an account if needed.
        pub Balances get(fn balances): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) T::CurrencyId => AccountCurrencyData<T::Balance>;
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
    }
    add_extra_genesis {
        config(endowed_accounts): Vec<(T::CurrencyId, T::AccountId, T::Balance)>;
        config(currency_details): Vec<(T::CurrencyId, CurrencyDetails<T::AccountId>)>;
        build(|config: &GenesisConfig<T>| {
            config.currency_details.iter().cloned().for_each(|(currency_id, currency_details)| {
                Module::<T>::set_currency_acl(currency_id, currency_details, None);
                // If we have an error it means that the currency had some coins issued earlier in
                // the genesis block, thus we ignore it.
                drop(Module::<T>::maybe_create_zero_issuance(currency_id));
            });

            config.endowed_accounts.iter().for_each(|(currency_id, account_id, initial_balance)| {
                Balances::<T>::mutate(account_id, *currency_id, |d| d.free = *initial_balance);
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
        CurrencyDetails = CurrencyDetails<<T as frame_system::Trait>::AccountId>,
    {
        /// A new currency has been created. \[currency id, details\]
        CurrencyCreated(CurrencyId, CurrencyDetails),
        /// Some units of currency were issued. \[currency_id, dest, amount\]
        CurrencyMinted(CurrencyId, AccountId, Balance),
        /// Some units of currency were destroyed. \[currency_id, source, amount\]
        CurrencyBurned(CurrencyId, AccountId, Balance),
        /// Some details about a currency were changed. \[currency_id, details\]
        CurrencyDetailsChanged(CurrencyId, CurrencyDetails),
        /// Some units of currency were transferred. \[currency_id, source, dest, amount\]
        CurrencyTransferred(CurrencyId, AccountId, AccountId, Balance),
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
        /// This owner(s) of this currency have disabled transfers
        UnTransferableCurrency,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        /// Creates a new currency with 0 units, to issue units to people one would have to call
        /// `issue`. This will register the caller of this dispatchable as the owner of the currency
        /// so they can issue or burn units. This will produce an error if `currency_id` is already
        /// used by another currency. Use `transferable` to determine if the created asset can be
        /// transferred between accounts. If not, the only way to move it would be to either be root
        /// or burn then mint the tokens again.
        ///
        /// NOTE: by default, everybody can create new currencies, if it is not wanted you can use the
        /// `bylaws` pallet to restrict access to this dispatchable.
        #[weight = T::WeightInfo::create()]
        pub fn create(origin, currency_id: T::CurrencyId, transferable: bool) {
            let who = RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::create_currencies())?;

            Self::maybe_create_zero_issuance(currency_id)?;

            let details = CurrencyDetails {
                owner: who.clone(),
                transferable,
            };
            Self::set_currency_acl(currency_id, details.clone(), None);
            Self::deposit_event(RawEvent::CurrencyCreated(currency_id, details));
        }

        /// Issue some units of the currency identified by `currency_id` and credit them to `dest`.
        /// Can only be called by the owner of the currency.
        #[weight = T::WeightInfo::mint()]
        pub fn mint(origin, currency_id: T::CurrencyId, dest: <T::Lookup as StaticLookup>::Source, amount: T::Balance) {
            RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::manage_currency(currency_id))?;
            let to = T::Lookup::lookup(dest)?;
            <Self as Currencies<T::AccountId>>::mint(currency_id, &to, amount)?;
        }

        /// Destroy some units of the currency identified by `currency_id` from `from`.
        /// Can only be called by the owner of the currency.
        #[weight = T::WeightInfo::burn()]
        pub fn burn(origin, currency_id: T::CurrencyId, from: <T::Lookup as StaticLookup>::Source, amount: T::Balance) {
            RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::manage_currency(currency_id))?;
            let source = T::Lookup::lookup(from)?;
            <Self as Currencies<T::AccountId>>::burn(currency_id, &source, amount)?;
        }

        /// Update details about the currency identified by `currency_id`. For instance, this
        /// can be used to change the owner of the currency. Can only be called by the owner.
        ///
        /// **NOTE**: this will remove ownership / management access from the caller for the given
        /// currency if a new owner is specified. However, if other accounts have been granted
        /// management access to the same currency (for instance through a root action) this will
        /// not change it.
        #[weight = T::WeightInfo::update_details()]
        pub fn update_details(origin, currency_id: T::CurrencyId, details: CurrencyDetails<T::AccountId>) {
            let who = RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::manage_currency(currency_id))?;
            Self::set_currency_acl(currency_id, details.clone(), Some(who));
            Self::deposit_event(RawEvent::CurrencyDetailsChanged(currency_id, details));
        }

        /// Transfer `amount` units of the currency identified by `currency_id` from the origin's
        /// account to the balance of `dest`.
        #[weight = T::WeightInfo::transfer()]
        pub fn transfer(origin, currency_id: T::CurrencyId, dest: <T::Lookup as StaticLookup>::Source, amount: T::Balance) {
            let from = ensure_signed(origin)?;
            let to = T::Lookup::lookup(dest)?;
            <Self as Currencies<T::AccountId>>::transfer(currency_id, &from, &to, amount)?;
        }
    }
}

impl<T: Trait> Module<T> {
    /// Return the `AccountCurrencyData` for the `who` and `currency_id`.
    fn get_currency_account(
        currency_id: T::CurrencyId,
        who: &T::AccountId,
    ) -> AccountCurrencyData<T::Balance> {
        Balances::<T>::get(who, currency_id)
    }

    /// Register the ACL roles accordingly for a given currency.
    fn set_currency_acl(
        currency_id: T::CurrencyId,
        details: CurrencyDetails<T::AccountId>,
        maybe_no_longer_owner: Option<T::AccountId>,
    ) {
        if let Some(previous_owner) = maybe_no_longer_owner {
            if details.owner != previous_owner {
                // Typical error at this stage would be that the role is not granted to
                // `previous_owner`. There are some edge cases where this is possible, for
                // instance if a root user revoked is calling the functions. Thus, we prefer
                // not to fail and drop the result.
                drop(RoleManagerOf::<T>::revoke_role(
                    Some(&previous_owner),
                    RoleBuilderOf::<T>::manage_currency(currency_id),
                ));
            }
        }

        // We drop the results as it is possible that the current owner itself is calling the update
        // function or that none of the acl parameters changed.
        // Since it is cheaper to fail on `grant_role` rather than calling `has_role` before we drop.

        drop(RoleManagerOf::<T>::grant_role(
            Some(&details.owner),
            RoleBuilderOf::<T>::manage_currency(currency_id),
        ));

        if details.transferable {
            drop(RoleManagerOf::<T>::grant_role(
                None,
                RoleBuilderOf::<T>::transfer_currency(currency_id),
            ));
        } else {
            drop(RoleManagerOf::<T>::revoke_role(
                None,
                RoleBuilderOf::<T>::transfer_currency(currency_id),
            ));
        }
    }

    /// Just set the total issuance to 0. This will write to the storage. Use only when
    /// creating new currencies.
    /// If this detect that the issuance field is already set this will not reset it to 0.
    fn maybe_create_zero_issuance(currency_id: T::CurrencyId) -> DispatchResult {
        // In order to prevent duplication of currencies (aka somebody creating an already existing
        // currency) have to mark the currency as created and check for this flag on the next calls
        // to `create`.
        // The naïve solution would be to add a new storage field to log this. Another one would be
        // to use the `TotalIssuances` storage and create a storage entry at 0 if a currency is created.
        // Future calls to `create` will realize that there is a value there and thus prevent the call
        // from doing any changes. This way, we don't have to use yet another storage map.

        TotalIssuances::<T>::try_mutate_exists(currency_id, |issuance| match issuance {
            Some(_) => Err(Error::<T>::CurrencyAlreadyExists.into()),
            None => {
                *issuance = Some(Zero::zero());
                Ok(())
            }
        })
    }
}
