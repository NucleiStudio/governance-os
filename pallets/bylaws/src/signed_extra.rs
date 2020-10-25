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

use crate::Trait;
use codec::{Decode, Encode};
use frame_support::{
    traits::{Get, GetCallMetadata},
    weights::DispatchInfo,
};
use governance_os_support::rules::Rule;
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
        sp_runtime::print("🏛 CheckBylaws::validate");

        match T::DefaultBylaw::get().validate(who, call, info, len) {
            true => Ok(ValidTransaction {
                ..Default::default()
            }),
            false => Err(InvalidTransaction::Call.into()),
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
