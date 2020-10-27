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

//! This pallet implements a bylaws system for account permissioning.
//! A bylaw is typically made of a "tag" and a "rule". Tags represents
//! the kinds of calls being filtered, for instance an hypothetic tag
//! `Monetary` could refer to all extrinsics moving tokens. A rule is
//! then a simple script implemented in a DSL (typically Rust enums)
//! to help the system decide if the call should be approved or discarded.
//!
//! The bylaws pallet should typically be added to the `SignedExtra` of a
//! runtime so that it could filter incoming calls.
//!
//! **NOTE**: this module is a bit specific in the sense that **no actual
//! access control is performed on critical functions**, indeed, we expect
//! the host runtime to create its own default bylaws to control those.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_error, decl_event, decl_module, decl_storage, traits::Get, Parameter};
use frame_system::ensure_signed;
use governance_os_support::rules::{CallTagger, Rule, SuperSetter};
use sp_runtime::traits::{MaybeSerializeDeserialize, Member, StaticLookup};
use sp_std::prelude::Vec;

mod signed_extra;
#[cfg(test)]
mod tests;

pub use signed_extra::CheckBylaws;

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    /// Tags are used to identify incoming calls and match them to some rules.
    type Tag: Parameter + Member + Copy + MaybeSerializeDeserialize + SuperSetter;
    /// An object to link incoming calls to tags.
    type Tagger: CallTagger<Self::AccountId, Self::Call, Self::Tag>;
    /// How bylaws are represented inside the system.
    type Bylaw: Parameter
        + Member
        + MaybeSerializeDeserialize
        + Clone
        + Rule<Self::AccountId, Self::Call>;
    /// The default bylaws to apply to calls without a bylaw already, typically this would be
    /// `Allow` for public networks and `Deny` for permissioned networks. We take in a list
    /// tag and bylaw in order to provide customization abilities.
    type DefaultBylaws: Get<Vec<(Self::Tag, Self::Bylaw)>>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Bylaws {
        /// Links an account to a series of bylaws.
        pub Bylaws get(fn bylaws): map hasher(blake2_128_concat) T::AccountId => Vec<(T::Tag, T::Bylaw)>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Tag = <T as Trait>::Tag,
        Bylaw = <T as Trait>::Bylaw,
    {
        /// Somebody added a bylaw to the account. [source, account, tag, bylaw]
        BylawAdded(AccountId, AccountId, Tag, Bylaw),
        /// Somebody deleted a bylaw from the account. [source, account, tag, bylaw]
        BylawRemoved(AccountId, AccountId, Tag, Bylaw),
        /// Somebody cleared the bylaws associated to an account. [source, account]
        BylawsReset(AccountId, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// The couple of bylaw and tag you are looking for doesn't exist for this account.
        BylawNotFound,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        /// Add a `bylaw` to a given account `who` for a call matching `tag`.
        #[weight = 0]
        fn add_bylaw(origin, who: <T::Lookup as StaticLookup>::Source, tag: T::Tag, bylaw: T::Bylaw) {
            let caller = ensure_signed(origin)?;
            let who_lookup = T::Lookup::lookup(who)?;

            <Bylaws<T>>::mutate(&who_lookup, |vec| vec.push((tag, bylaw.clone())));

            Self::deposit_event(RawEvent::BylawAdded(caller, who_lookup, tag, bylaw));
        }

        /// Remove a `bylaw` from a given account `who` for a call matching `tag`.
        #[weight = 0]
        fn remove_bylaw(origin, who: <T::Lookup as StaticLookup>::Source, tag: T::Tag, bylaw: T::Bylaw) {
            let caller = ensure_signed(origin)?;
            let who_lookup = T::Lookup::lookup(who)?;

            // When it comes to removal we chose to delete bylaws by (tag, bylaw) and not by identifier
            // as removing by id would mess things up in the event that calls to `remove_bylaw` are batched,
            // indeed, removing one bylaw could substract one from all other bylaw ids.
            let mut bylaws = <Bylaws<T>>::get(&who_lookup);
            let location = bylaws.iter().cloned().position(|(inner_tag, inner_bylaw)| inner_tag == tag && inner_bylaw == bylaw).ok_or(Error::<T>::BylawNotFound)?;
            bylaws.remove(location);

            <Bylaws<T>>::mutate(&who_lookup, |v| *v = bylaws);

            Self::deposit_event(RawEvent::BylawRemoved(caller, who_lookup, tag, bylaw));
        }

        /// Clear all bylaws associated to `who`. Account will still have to comform to the default runtime bylaws.
        #[weight = 0]
        fn reset_bylaws(origin, who: <T::Lookup as StaticLookup>::Source) {
            let caller = ensure_signed(origin)?;
            let who_lookup = T::Lookup::lookup(who)?;

            <Bylaws<T>>::remove(&who_lookup);

            Self::deposit_event(RawEvent::BylawsReset(caller, who_lookup));
        }
    }
}
