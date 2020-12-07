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

//! A set of common traits to define Access Control lists between pallets and
//! runtime users.

use frame_system::{ensure_signed, RawOrigin};
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::convert::Into;

pub enum AclError {
    MissingRole,
}

impl Into<DispatchError> for AclError {
    fn into(self) -> DispatchError {
        match self {
            AclError::MissingRole => {
                DispatchError::Other("account doesn't have the required role(s)")
            }
        }
    }
}

/// This trait can be implemented by a pallet to expose an interface for other pallets to
/// manage their own role based access control features.
pub trait RoleManager {
    type AccountId;
    type Role;

    /// Should return `true` if `traget` has the role `role`. This can be the case
    /// if the role was granted directly to the target or if it was granted to all accounts.
    fn has_role(target: &Self::AccountId, role: Self::Role) -> bool;

    /// Grants `target` the role `role`. If target is `None` then it should give the role to
    /// every account that exists or may exists on the chain.
    fn grant_role(target: Option<&Self::AccountId>, role: Self::Role) -> DispatchResult;

    /// Should revoke the role `role` for `target`. If the role wasn't granted to `target` this
    /// should error.
    fn revoke_role(target: Option<&Self::AccountId>, role: Self::Role) -> DispatchResult;

    /// A helper function that will require the origin to have the `role` granted. We provide a
    /// default implementation for it.
    fn ensure_has_role<OuterOrigin>(
        origin: OuterOrigin,
        role: Self::Role,
    ) -> Result<Self::AccountId, DispatchError>
    where
        OuterOrigin: Into<Result<RawOrigin<Self::AccountId>, OuterOrigin>>,
    {
        let who = ensure_signed(origin)?;
        match Self::has_role(&who, role) {
            true => Ok(who),
            false => Err(AclError::MissingRole.into()),
        }
    }
}
