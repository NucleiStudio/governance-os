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

use crate::{CoinVoting, ConvictionVoting, PlcrVoting};
use codec::{Decode, Encode};
pub use governance_os_pallet_coin_voting::{
    VoteData as CoinVoteData, VotingParameters as CoinVotingParameters,
};
pub use governance_os_pallet_conviction_voting::{
    Conviction, VotingParameters as ConvictionVotingParameters,
};
pub use governance_os_pallet_plcr_voting::{
    VoteData as PlcrVoteData, VotingParameters as PlcrVotingParameters,
};
use governance_os_primitives::{AccountId, Balance, BlockNumber, CurrencyId, Hash};
use governance_os_support::traits::{ProposalResult, StandardizedVoting, VotingRouter};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::result;

/// An enum to wrap the different voting parameters for all the runtime voting
/// implementations.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum RuntimeVotingParameters {
    CoinVoting(CoinVotingParameters<BlockNumber, CurrencyId>),
    ConvictionVoting(ConvictionVotingParameters<BlockNumber, CurrencyId>),
    PlcrVoting(PlcrVotingParameters<BlockNumber, CurrencyId>),
}

/// An enum to wrap the different vote data for all the runtime voting
/// implementations.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum RuntimeVoteData {
    CoinVoting(CoinVoteData<Balance>),
    ConvictionVoting(Conviction<Balance>),
    PlcrVoting(PlcrVoteData<Balance, Hash>),
}

/// An enum to differentiate between the different runtime voting
/// implementations. The voting router will use it to select the
/// right parameters and vote data.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum RuntimeVotingSystemId {
    CoinVoting,
    ConvictionVoting,
    PlcrVoting,
}

pub struct RuntimeVotingRouter;
impl VotingRouter for RuntimeVotingRouter {
    type AccountId = AccountId;
    type VotingSystemId = RuntimeVotingSystemId;
    type Parameters = RuntimeVotingParameters;
    type ProposalId = Hash;
    type VoteData = RuntimeVoteData;

    fn initiate(
        voting_system: Self::VotingSystemId,
        proposal: Self::ProposalId,
        parameters: Self::Parameters,
    ) -> DispatchResult {
        match (voting_system, parameters) {
            (RuntimeVotingSystemId::CoinVoting, RuntimeVotingParameters::CoinVoting(params)) => {
                CoinVoting::initiate(proposal, params)
            }
            (
                RuntimeVotingSystemId::ConvictionVoting,
                RuntimeVotingParameters::ConvictionVoting(params),
            ) => ConvictionVoting::initiate(proposal, params),
            (RuntimeVotingSystemId::PlcrVoting, RuntimeVotingParameters::PlcrVoting(params)) => {
                PlcrVoting::initiate(proposal, params)
            }
            _ => Err("wrong voting system, voting parameters pair".into()),
        }
    }

    fn veto(voting_system: Self::VotingSystemId, proposal: Self::ProposalId) -> DispatchResult {
        match voting_system {
            RuntimeVotingSystemId::CoinVoting => CoinVoting::veto(proposal),
            RuntimeVotingSystemId::ConvictionVoting => ConvictionVoting::veto(proposal),
            RuntimeVotingSystemId::PlcrVoting => PlcrVoting::veto(proposal),
        }
    }

    fn vote(
        voting_system: Self::VotingSystemId,
        proposal: Self::ProposalId,
        voter: &Self::AccountId,
        vote_data: Self::VoteData,
    ) -> DispatchResult {
        match (voting_system, vote_data) {
            (RuntimeVotingSystemId::CoinVoting, RuntimeVoteData::CoinVoting(data)) => {
                CoinVoting::vote(proposal, voter, data)
            }
            (RuntimeVotingSystemId::ConvictionVoting, RuntimeVoteData::ConvictionVoting(data)) => {
                ConvictionVoting::vote(proposal, voter, data)
            }
            (RuntimeVotingSystemId::PlcrVoting, RuntimeVoteData::PlcrVoting(data)) => {
                PlcrVoting::vote(proposal, voter, data)
            }
            _ => Err("wrong voting system, vote data pair".into()),
        }
    }

    fn close(
        voting_system: Self::VotingSystemId,
        proposal: Self::ProposalId,
    ) -> result::Result<ProposalResult, DispatchError> {
        match voting_system {
            RuntimeVotingSystemId::CoinVoting => CoinVoting::close(proposal),
            RuntimeVotingSystemId::ConvictionVoting => ConvictionVoting::close(proposal),
            RuntimeVotingSystemId::PlcrVoting => PlcrVoting::close(proposal),
        }
    }
}
