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

use crate::{Bylaws, Trait};
use codec::{Decode, Encode};
use frame_support::{
    storage::StorageMap,
    traits::{Get, GetCallMetadata},
    weights::DispatchInfo,
};
use governance_os_support::rules::{CallTagger, Rule, SuperSetter};
use sp_runtime::{
    traits::{DispatchInfoOf, Dispatchable, SignedExtension},
    transaction_validity::{
        InvalidTransaction, TransactionValidity, TransactionValidityError, ValidTransaction,
    },
    RuntimeDebug,
};
use sp_std::{fmt::Debug, marker, result};

/// This structure can be used as part of the `SignedExtra` portion of a runtime
/// so that the pallet intercepts calls and apply them the registered bylaws.
/// `T` should be a value implementing `crate::Trait`, normally a runtime.
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct CheckBylaws<T>(marker::PhantomData<T>);

impl<T> SignedExtension for CheckBylaws<T>
where
    T: Trait + Send + Sync + Debug,
    T::Call: Dispatchable<Info = DispatchInfo> + GetCallMetadata,
{
    const IDENTIFIER: &'static str = "Bylaws";
    type AccountId = T::AccountId;
    type Call = T::Call;
    type AdditionalSigned = ();
    type Pre = ();

    fn additional_signed(
        &self,
    ) -> result::Result<<Self as SignedExtension>::AdditionalSigned, TransactionValidityError> {
        Ok(())
    }

    fn validate(
        &self,
        who: &Self::AccountId,
        call: &Self::Call,
        info: &DispatchInfoOf<Self::Call>,
        len: usize,
    ) -> TransactionValidity {
        let default_bylaws = T::DefaultBylaws::get();
        let account_bylaws = Bylaws::<T>::get(who);

        if default_bylaws.is_empty() && account_bylaws.is_empty() {
            sp_runtime::print("ERROR in bylaws pallet: no bylaws are configured for the account and no default bylaws are set; skipping call filtering");
            return Ok(ValidTransaction {
                ..Default::default()
            });
        }

        match default_bylaws
            .iter()
            .chain(account_bylaws.iter())
            .cloned()
            .filter(|(tag, _)| {
                // We keep only the supersets, we assume that if
                // tag_a == tag_b superset will return true.
                tag.is_superset(&T::Tagger::tag(who, call))
            })
            .find(|(_, bylaw)| {
                // At this stage we know that tag matches `call` and `who`,
                // we simply need to check if a bylaw denies the call. If that's
                // the case we stop looking would deny the call.
                !bylaw.validate(who, call, info, len)
            }) {
            // No bylaws that would deny the call found
            None => Ok(ValidTransaction {
                ..Default::default()
            }),
            // We found at least one bylaw that would deny the call, error!
            Some(..) => Err(InvalidTransaction::Call.into()),
        }
    }
}

// For utility purposes, we support the `Default` constructor (mostly used in tests).
#[cfg(test)]
impl<T> Default for CheckBylaws<T> {
    fn default() -> Self {
        CheckBylaws {
            0: marker::PhantomData,
        }
    }
}
