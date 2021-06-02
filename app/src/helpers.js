/// parse a call from hexadecimal to a human understandable string for better UX
const parseCall = (api, call) => {
    let parsed = api.createType('Call', call);
    return `${parsed.section}.${parsed.method}(${parsed.args})`;
}

/// return wether a proposal will pass or expire for the coin voting system
const coinVotingState = (state, totalSupply, now) => {
    const totalParticipation = state["total_favorable"].add(state["total_against"]);
    const minParticipation = state.parameters["min_participation"] / 100;
    const minQuorum = state.parameters["min_quorum"] / 100;
    const totalFavorable = state["total_favorable"];
    const createdOn = state["created_on"].toNumber();
    const ttl = state.parameters.ttl.toNumber();

    const enoughParticipation = totalParticipation > minParticipation * totalSupply;
    const enoughQuorum = totalFavorable > minQuorum * totalParticipation;
    const proposalPassing = enoughParticipation && enoughQuorum;

    const proposalExpired = now > createdOn + ttl;

    return [proposalPassing, proposalExpired];
};

/// return wether a proposal will pass or expire for the conviction voting system
const convictionVotingState = (state, totalSupply, now) => {
    // we only support closing a conviction voting proposal once it is expired
    // as conviction accumulates over time, it needs to be computed regularly
    // and this would amount to lot of duplicated code here. in the future,
    // we may add a RPC call for it.
    const proposalPassing = false;

    const createdOn = state["created_on"].toNumber();
    const ttl = state.parameters.ttl.toNumber();

    const proposalExpired = now > createdOn + ttl;

    return [proposalPassing, proposalExpired];
};

/// return wether a proposal will pass or expire for the plcr voting system
const plcrVotingState = (state, totalSupply, now) => {
    const totalParticipation = state["revealed_favorable"].add(state["revealed_against"]);
    const minParticipation = state.parameters["min_participation"] / 100;
    const minQuorum = state.parameters["min_quorum"] / 100;
    const totalFavorable = state["revealed_favorable"];
    const createdOn = state["created_on"].toNumber();
    const ttl = state.parameters["commit_duration"].add(state.parameters["reveal_duration"]).toNumber();

    const enoughParticipation = totalParticipation > minParticipation * totalSupply;
    const enoughQuorum = totalFavorable > minQuorum * totalParticipation;
    const proposalPassing = enoughParticipation && enoughQuorum;

    const proposalExpired = now > createdOn + ttl;

    return [proposalPassing, proposalExpired];
};

export {
    parseCall,
    coinVotingState,
    convictionVotingState,
    plcrVotingState,
};