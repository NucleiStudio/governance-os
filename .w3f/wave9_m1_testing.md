# Milestone 1 Testing

Milestone 1 covered the development of two pallets: `coin-voting` and `plcr-voting` along with any necessary refactoring; let's see how they can be tested.

## Requirements
You will need a few things first:
1. A running local node, the easiest way to create one is to use this command: `docker run --rm -p 9944:9944 -it eteissonniere/governance-os --dev --tmp --ws-external`.
2. A correctly configured [Polkadot JS UI](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer), you may need the types which you can find [here](../types.json).

## Liquid Democracy / Coin Voting
### Create an organization
Submit the extrinsic `organizations.create` with the following parameters:
- `RuntimeVotingSystemId`: keep it to `CoinVoting`, this is how you can change an organization's voting system.
- `ttl`: for testing set it to `10`, this means that we will be able to close a proposal 10 blocks after its creation.
- `voting_currency`: this lets us choose which currency will represent votes, keep it to `Native`.
- `min_quorum`: set it to `50`, this is the minimum quorum that must be reached for a proposal to be considered passing.
- `min_participation`: set it to `33`, if less than `33%` of the total supply of the tokens vote then the proposal will be considered as failing.
- `vote_counting_strategy`: either `Simple` or `Quadratic`. This defines how votes are counted, with `Simple`, one tokens means one vote while with `Quadratic` the power in favor or against a proposal is equal to the square root of the tokens staked.

Submit the transaction, the event `organizations.OrganizationCreated` should be triggered and contain an account id, keep it somewhere as it is the address of the organization you created. In our case it was `5EYCAe5gvgRHjJhnqvSAGV55RLCi7RWEMW5KdZiaE4ztFZ67` but this may change if you created other organizations before.

### Create a proposal
You can now submit a proposal via the extrinsic `organizations.createProposal`, we used the following parameters:
- `org_id`: the address of the organization you created.
- `call`: what the proposal would execute if it passes. We just used a simple `system.remark` call.

Once you submit the transaction, the event `organizations.ProposalSubmitted` is triggered. It contains a `proposal_id` that you can copy and use to view the state of the given proposal. In our case it was `0xa1e04c7a00a4fde442898f5e0c4332205ec50c57366399f2643c7446867133be` but this may be different for you.

You should also be able to view the proposal via the chain state `coinVoting.proposals`, it should contain informations regarding current votes, when the proposal was created etc.

### Cast some votes
Submit the extrinsic `organizations.decideOnProposal` with the following parameters:
- `proposal_id` is the id of the proposal you want to cast the vote for. In our case it is `0xa1e04c7a00a4fde442898f5e0c4332205ec50c57366399f2643c7446867133be`.
- `vote_data` lets you set the vote you want to cast, keep it to `CoinVoting` for now and fill the following fields:
   - `in_support`: wether the account supports the proposal, we set it to `true`.
   - `power`: how much tokens are locked in favor of this proposal, we set it to `1M` units.

Once you submit the transaction the event `organizations.ProposalVoteCasted` should be triggered. The chain state of the proposal should have been mutated as well. If you are using the `Simple` vote counting strategy the coins will be locked, if you are using the `Quadratic` strategy, the coins will be reserved.

> The development chain spec we are using grants a little over 1M of `Native` currency to `Alice` and `Bob`. This is why we vote with 1M units in order to reach the minimum participation criteria.

### Close and execute the proposal
Assuming you casted a favorable vote previously and didn't add any against the proposal you should be able to close and execute the proposal now. To do so, submit the extrinsic `organizations.closeProposal` with the following parameters:
- `proposal_id` is the id of the proposal you want to cast the vote for. In our case it is `0xa1e04c7a00a4fde442898f5e0c4332205ec50c57366399f2643c7446867133be`.
- `proposal_weight_bound` is used for weight computation and is adjusted after the proposal is executed. We set it to `3000000` which should be enough for our `system.remark` call.

After submitting the transaction you should see the event `organizations.ProposalClosed` which will indicate wether the proposal passed or not.

## PLCR Voting
The main change from the coin voting pallet is the handling of the votes which have to happen in two phases:
- first, voters have to "commit" their votes in the form of a hash
- then, they have to "reveal" their votes after some time

We only count revealed votes as commits are concealing the real votes in order to prevent collusion and human biases.

### Create an organization
Submit the extrinsic `organizations.create` with the following parameters:
- `RuntimeVotingSystemId`: set it to `PlcrVoting`, and set the `RuntimeVotingParameters` to `PlcrVoting` as well.
- `commit_duration`: for testing set it to `100`, this means you have `100` blocks to commit your votes after creating a proposal, if you need more time just bump this number.
- `reveal_duration`: just like `commit_duration`, we set it to `100`, again, feel free to bump it up.
- `voting_currency`: keep it to `Native`.
- `min_quorum`: set it to `50`, this is the minimum quorum that must be reached for a proposal to be considered passing.
- `min_participation`: set it to `33`, if less than `33%` of the total supply of the tokens vote then the proposal will be considered as failing.

Submit the transaction, the event `organizations.OrganizationCreated` should be triggered and contain an account id, keep it somewhere as it is the address of the organization you created. In our case it was `5EYCAe5gvgRHjJhnqvSS1WcfXpsuZyc76B1qBWQQgtscXF15` but this may change if you created other organizations before.

### Create a proposal
You can now submit a proposal via the extrinsic `organizations.createProposal`, we used the following parameters:
- `org_id`: the address of the organization you created.
- `call`: what the proposal would execute if it passes. We just used a simple `system.remark` call.

Once you submit the transaction, the event `organizations.ProposalSubmitted` is triggered. It contains a `proposal_id` that you can copy and use to view the state of the given proposal. In our case it was `0x6b7a92a8c495efcd7000543e80761cb94f8504b193f6f2ada4ca5d8128ad425b` but this may be different for you.

You should also be able to view the proposal via the chain state `plcrVoting.proposals`, it should contain informations regarding current votes, when the proposal was created etc.

### Cast some votes
With PLCR voting you need to generate two kinds of votes, a commit and a reveal vote. We have included a command to generate those. In order to generate a favorable vote with `1M` units of currency you can use this:
```sh
$ cargo run -- generate-plcr-votes --power 1000000000000000000 --in-support --salt 1
Commit: 0xbc4dc199f6cc94982a96301c84321fb23c842667eb7218c584c2e8624dc9c67e
Reveal: 1000000000000000000, true, 1
```

> To continue, make sure the proposal you are voting on is less than a 100 blocks old, else you will have to create a new one because too much time would have elapsed.

And you can use it like this:
1. Commit the vote via `organizations.decideOnProposal` with the following parameters:
   - `proposal_id`: your proposal id.
   - `vote_data`: set it to `PlcrVoting.Commit(0xbc4dc199f6cc94982a96301c84321fb23c842667eb7218c584c2e8624dc9c67e)` or whichever commit value you generated before.
2. `100` blocks or so later submit the extrinsic `organizations.decideOnProposal` with the following parameters:
   - `proposal_id`: your proposal id.
   - `vote_data`: set it to `PlcrVoting.Reveal(1M, true, 1)` or whichever values you generated before when generating the commit hash.

### Close and execute the proposal
Assuming you casted a favorable vote previously and didn't add any against the proposal you should be able to close and execute the proposal now. To do so, submit the extrinsic `organizations.closeProposal` with the following parameters:
- `proposal_id` is the id of the proposal you want to cast the vote for.
- `proposal_weight_bound` is used for weight computation and is adjusted after the proposal is executed. We set it to `3000000` which should be enough for our `system.remark` call.

After submitting the transaction you should see the event `organizations.ProposalClosed` which will indicate wether the proposal passed or not.
