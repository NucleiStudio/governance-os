/// parse a call from hexadecimal to a human understandable string for better UX
const parseCall = (api, call) => {
    let parsed = api.createType('Call', call);
    return `${parsed.section}.${parsed.method}(${parsed.args})`;
}

const proposalPassing = (totalParticipation, totalFavorable, minParticipation, minQuorum, totalSupply) => {
    const enoughParticipation = totalParticipation > minParticipation * totalSupply;
    const enoughQuorum = totalFavorable > minQuorum * totalParticipation;
    return enoughParticipation && enoughQuorum;
};

const proposalExpired = (now, createdOn, ttl) => {
    return now > createdOn + ttl;
};

/// return wether a proposal will pass or expire for the coin voting system
const coinVotingState = (state, totalSupply, now) => {
    return [
        proposalPassing(
            state["total_favorable"].add(state["total_against"]),
            state["total_favorable"],
            state.parameters["min_participation"] / 100,
            state.parameters["min_quorum"] / 100,
            totalSupply
        ),
        proposalExpired(now, state["created_on"].toNumber(), state.parameters.ttl.toNumber())
    ];
};

/// return wether a proposal will pass or expire for the conviction voting system
const convictionVotingState = (state, _totalSupply, now) => {
    return [
        // we only support closing a conviction voting proposal once it is expired
        // as conviction accumulates over time, it needs to be computed regularly
        // and this would amount to lot of duplicated code here. in the future,
        // we may add a RPC call for it.
        false,
        proposalExpired(now, state["created_on"].toNumber(), state.parameters.ttl.toNumber())
    ];
};

/// return wether a proposal will pass or expire for the plcr voting system
const plcrVotingState = (state, totalSupply, now) => {
    return [
        proposalPassing(
            state["revealed_favorable"].add(state["revealed_against"]),
            state["revealed_favorable"],
            state.parameters["min_participation"] / 100,
            state.parameters["min_quorum"] / 100,
            totalSupply
        ),
        proposalExpired(
            now,
            state["created_on"].toNumber(),
            state.parameters["commit_duration"].add(state.parameters["reveal_duration"]).toNumber()
        )
    ];
};

export {
    parseCall,
    coinVotingState,
    convictionVotingState,
    plcrVotingState,
};