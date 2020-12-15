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

use governance_os_pallet_organizations::{Module as OrganizationsModule, OrganizationsCounter};
use governance_os_primitives::AccountId;
use governance_os_runtime::Runtime;
use sp_core::crypto::Ss58AddressFormat;

pub fn org_id_for(counter: OrganizationsCounter) -> AccountId {
    OrganizationsModule::<Runtime>::org_id_for(counter)
}

/// Return the address of the "core" dOrg which should be created during genesis
/// or later. Will be granted root role and admin access to the core currency
/// primitives by most of our genesis generation functions. Used to take network
/// wide decisions in a decentralized manner!
pub fn core_org() -> AccountId {
    org_id_for(0) // 5EYCAe5gvgRHjJhnqvRtXTXVJqXWesQMcq8p5d2jmF89z84d
}

/// Reconfigure the core crypto / ss58 crates to use our own identifier.
pub fn set_default_ss58_version() {
    // Once we start having different networks we will want to be smart about it
    // and default to different formats.

    // Once upstream releases a new version default with our id replace to the hardcoded
    // value there.
    let ss58_version = Ss58AddressFormat::SubstrateAccount;

    sp_core::crypto::set_default_ss58_version(ss58_version);
}
