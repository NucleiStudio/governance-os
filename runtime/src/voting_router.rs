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

use crate::{CoinVoting, PlcrVoting};
use codec::{Decode, Encode};
use governance_os_pallet_coin_voting::{
    VoteData as CoinVoteData, VotingParameters as CoinVotingParameters,
};
use governance_os_pallet_plcr_voting::{
    VoteData as PlcrVoteData, VotingParameters as PlcrVotingParameters,
};
use governance_os_primitives::{AccountId, Balance, BlockNumber, CurrencyId, Hash};
use governance_os_support::traits::{ProposalResult, StandardizedVoting, VotingRouter};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::result;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum RuntimeVotingParameters {
    CoinVoting(CoinVotingParameters<BlockNumber, CurrencyId>),
    PlcrVoting(PlcrVotingParameters<BlockNumber, CurrencyId>),
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum RuntimeVoteData {
    CoinVoting(CoinVoteData<Balance>),
    PlcrVoting(PlcrVoteData<Balance, Hash>),
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum RuntimeVotingSystemId {
    CoinVoting,
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
            (RuntimeVotingSystemId::PlcrVoting, RuntimeVotingParameters::PlcrVoting(params)) => {
                PlcrVoting::initiate(proposal, params)
            }
            _ => Err("wrong voting system, voting parameters pair".into()),
        }
    }

    fn veto(voting_system: Self::VotingSystemId, proposal: Self::ProposalId) -> DispatchResult {
        match voting_system {
            RuntimeVotingSystemId::CoinVoting => CoinVoting::veto(proposal),
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
            RuntimeVotingSystemId::PlcrVoting => PlcrVoting::close(proposal),
        }
    }
}
