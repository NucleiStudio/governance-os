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

//! This pallet handles compatibility between the "legacy" substrate ACL system
//! which distinguishes between account origins and a root origin. It lets users
//! with a `Root` role trigger calls dispatched with the substrate root origin,
//! typically this gives the possibility to trigger upgrade to the chains and
//! interact with modules that do not support our bylaws system.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_event, decl_module, decl_storage,
    dispatch::DispatchResultWithPostInfo,
    traits::{Get, UnfilteredDispatchable},
    weights::{GetDispatchInfo, Pays, Weight},
    Parameter,
};
use governance_os_pallet_bylaws::RoleBuilder;
use governance_os_support::traits::RoleManager;
use sp_runtime::{traits::StaticLookup, DispatchResult};
use sp_std::boxed::Box;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event> + Into<<Self as frame_system::Trait>::Event>;

    /// A sudo-able call.
    type Call: Parameter + UnfilteredDispatchable<Origin = Self::Origin> + GetDispatchInfo;

    /// The role builder used by the `bylaws` pallet
    type RoleBuilder: RoleBuilder<Role = <RoleManagerOf<Self> as RoleManager>::Role>;

    /// Pallet used to manage and check for roles.
    type RoleManager: RoleManager<AccountId = Self::AccountId>;
}

type RoleBuilderOf<T> = <T as Trait>::RoleBuilder;
type RoleManagerOf<T> = <T as Trait>::RoleManager;

decl_storage! {
    trait Store for Module<T: Trait> as Compat {}
}

decl_event!(
    pub enum Event {
        /// A sudo just took place. \[result\]
        CompatSudid(DispatchResult),
        /// A root user just performed a call as someone else. \[result\]
        CompatDidAs(DispatchResult),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Dispatches the given call with a `Root` origin.
        #[weight = (call.get_dispatch_info().weight + 10_000, call.get_dispatch_info().class)]
        fn sudo(origin, call: Box<<T as Trait>::Call>) -> DispatchResultWithPostInfo {
            RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::root())?;

            let res = call.dispatch_bypass_filter(frame_system::RawOrigin::Root.into());
            Self::deposit_event(Event::CompatSudid(res.map(|_| ()).map_err(|e| e.error)));

            // Caller won't pay a fee.
            Ok(Pays::No.into())
        }

        /// A variant of the `sudo` dispatchable that will let caller specify its weight. This could be
        /// useful when trying to push a runtime upgrade but should be used with parcimony.
        #[weight = (*_weight, call.get_dispatch_info().class)]
        fn sudo_custom_weight(origin, call: Box<<T as Trait>::Call>, _weight: Weight) -> DispatchResultWithPostInfo {
            // Just proxy back to the `sudo` call
            Self::sudo(origin, call)
        }

        // Dispatches the call with the origin set to `who`.
        #[weight = (
            call.get_dispatch_info().weight
                .saturating_add(10_000)
                // AccountData for inner call origin accountdata.
                .saturating_add(T::DbWeight::get().reads_writes(1, 1))
                // Two read performed by ensure_has_role
                .saturating_add(T::DbWeight::get().reads(2)),
            call.get_dispatch_info().class
        )]
        fn doas(origin,
            who: <T::Lookup as StaticLookup>::Source,
            call: Box<<T as Trait>::Call>
        ) -> DispatchResultWithPostInfo {
            RoleManagerOf::<T>::ensure_has_role(origin, RoleBuilderOf::<T>::root())?;

            let who = T::Lookup::lookup(who)?;

            let res = call.dispatch_bypass_filter(frame_system::RawOrigin::Signed(who).into());
            Self::deposit_event(Event::CompatDidAs(res.map(|_| ()).map_err(|e| e.error)));

            // Sudo user does not pay a fee.
            Ok(Pays::No.into())
        }
    }
}
