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

use crate::{Bylaws, Event, Runtime, System};
use frame_support::{parameter_types, weights::IdentityFee};
use governance_os_pallet_tokens::NativeCurrencyAdapter;
use governance_os_primitives::{Balance, CurrencyId, Role};

impl governance_os_pallet_tokens::Trait for Runtime {
    type Event = Event;
    type Balance = Balance;
    type CurrencyId = CurrencyId;
    type WeightInfo = ();
    type AccountStore = System;
    type RoleBuilder = Role;
    type RoleManager = Bylaws;
}

parameter_types! {
    pub const NativeCurrencyId: CurrencyId = CurrencyId::Native;
}

/// The system's native currency, typically used to pay for fees.
pub type NativeCurrency = NativeCurrencyAdapter<Runtime, NativeCurrencyId>;

parameter_types! {
    pub const TransactionByteFee: Balance = 1;
}

impl pallet_transaction_payment::Trait for Runtime {
    type Currency = NativeCurrency;
    // TODO: split fees between block author and native dOrg
    type OnTransactionPayment = ();
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
}
