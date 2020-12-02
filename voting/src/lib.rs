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

//! This crate is in charge of containing voting systems as supported by the Governance OS.
//! They were made to integrate nicely with our own `organizations` pallet but should be reusable
//! by other projects as well.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use governance_os_support::{voting::VotingHooks, Currencies, ReservableCurrencies};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{DispatchResult, RuntimeDebug};
use sp_std::{collections::btree_map::BTreeMap, marker};

/// Metadata used and managed by all our voting systems. Need to be passed to their functions and
/// persisted in storage.
#[derive(Eq, PartialEq, RuntimeDebug, Encode, Decode, Clone, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ProposalMetadata<AccountId, Balance>
where
    AccountId: Ord,
{
    /// A map of voter => (coins, in favor or against)
    pub votes: BTreeMap<AccountId, (Balance, bool)>,
    /// Cached value of how many coins support the proposal
    pub favorable: Balance,
    /// Cached value of how many coins oppose the proposal
    pub against: Balance,
}

/// This enum is in charge of representing and handling all existing voting systems.
#[derive(Eq, PartialEq, RuntimeDebug, Encode, Decode, Copy, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum VotingSystems<Balance, CurrencyId, BlockNumber, Currencies, AccountId> {
    /// An empty voting system. Useful if someone wants to create an organization
    /// managed by other things than votes.
    None,

    /// A simple coin based voting system where people vote by reserving some of their own coins.
    /// Our implementation has the following specificities:
    /// - creating a proposal requires the creator to block some of their coins to avoid spam; the coins
    ///   are considered in support of the proposal.
    /// - in the event that someone makes multiple votes we consider future votes as updates of the previous
    ///   ones; this means that weight and locked coins are recomputed every time.
    /// - a proposal passes once a configurable quorum is met (for instance, 50% of the coins need to agree)
    /// - a minimum quorum is in place which represents the minimum amount of coins a proposal need to be even
    ///   considered
    /// - a voting period duration is configured; after it is elapsed proposals can be closed by anyone
    CoinBased(CoinBasedVotingParameters<Balance, CurrencyId, BlockNumber>),

    /// We need this entry for accepting a `Currencies` and a `AccountId` type.
    _Phantom(marker::PhantomData<(AccountId, Currencies)>),
}
impl<Balance, CurrencyId, BlockNumber, Currencies, AccountId> Default
    for VotingSystems<Balance, CurrencyId, BlockNumber, Currencies, AccountId>
{
    fn default() -> Self {
        Self::None
    }
}

/// Parameters for the coin based voting system.
#[derive(Eq, PartialEq, RuntimeDebug, Encode, Decode, Copy, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CoinBasedVotingParameters<Balance, CurrencyId, BlockNumber> {
    /// What currency is used to represent vote inside the organization.
    pub voting_currency: CurrencyId,
    /// How much one has to lock to create a proposal and fight spam.
    pub creation_fee: Balance,
    /// What percentage of coins need to be in favor of a proposal for it to pass.
    /// We will then use it via a `perbill::from_percent` call.
    pub min_quorum: Balance,
    /// What percentage of coins need to participate before a quorum is reached.
    /// We will then use it via a `perbill::from_percent` call.
    pub min_participation: Balance,
    /// After how many blocks will a proposal be considered failed if it doesn't reach
    /// a favorable outcome.
    pub ttl: BlockNumber,
}

impl<AccountId, BlockNumber, C> VotingHooks
    for VotingSystems<C::Balance, C::CurrencyId, BlockNumber, C, AccountId>
where
    AccountId: Ord,
    C: ReservableCurrencies<AccountId> + Currencies<AccountId>,
{
    type AccountId = AccountId;
    type OrganizationId = AccountId;
    type VotingSystem = Self;
    type Currencies = C;
    type Data = ProposalMetadata<AccountId, C::Balance>;

    fn on_creating_proposal(
        voting_system: Self::VotingSystem,
        creator: &Self::AccountId,
    ) -> (DispatchResult, Self::Data) {
        unimplemented!()
    }

    fn on_veto_proposal(voting_system: Self::VotingSystem, data: Self::Data) -> DispatchResult {
        unimplemented!()
    }

    fn on_decide_on_proposal(
        voting_system: Self::VotingSystem,
        data: Self::Data,
        voter: &Self::AccountId,
        power: <Self::Currencies as Currencies<Self::AccountId>>::Balance,
        in_support: bool,
    ) -> (DispatchResult, Self::Data) {
        unimplemented!()
    }

    fn can_close(voting_system: Self::VotingSystem, data: Self::Data) -> bool {
        unimplemented!()
    }

    fn passing(voting_system: Self::VotingSystem, data: Self::Data) -> bool {
        unimplemented!()
    }

    fn on_close_proposal(voting_system: Self::VotingSystem, data: Self::Data, executed: bool) {
        unimplemented!()
    }
}
