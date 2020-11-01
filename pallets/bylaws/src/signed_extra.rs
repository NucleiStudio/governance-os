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

use crate::{Module, Trait};
use codec::{Decode, Encode};
use frame_support::{traits::GetCallMetadata, weights::DispatchInfo};
use governance_os_support::acl::{CallFilter, RoleManager};
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
pub struct CheckRole<T>(marker::PhantomData<T>);

impl<T> SignedExtension for CheckRole<T>
where
    T: Trait + Send + Sync + Debug,
    T::Call: Dispatchable<Info = DispatchInfo> + GetCallMetadata,
{
    const IDENTIFIER: &'static str = "BylawsRole";
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
        let roles = T::CallFilter::roles_for(who, call, info, len);
        // Either `who` is root either it has one of the roles needed.
        if roles.is_empty()
            || roles
                .iter()
                .cloned()
                .find(|role| Module::<T>::has_role(who, *role))
                .is_some()
        {
            return Ok(ValidTransaction {
                ..Default::default()
            });
        }

        Err(InvalidTransaction::Call.into())
    }
}

// For utility purposes, we support the `Default` constructor (mostly used in tests).
#[cfg(test)]
impl<T> Default for CheckRole<T> {
    fn default() -> Self {
        CheckRole {
            0: marker::PhantomData,
        }
    }
}
