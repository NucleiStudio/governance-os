# Milestone 2 Testing

Milestone 2 covered the development of the `conviction-voting` pallet along with any necessary refactoring; let's see how it can be tested.

## Requirements
You will need a few things first:
1. A running local node, the easiest way to create one is to use this command: `docker run --rm -p 9944:9944 -it eteissonniere/governance-os --dev --tmp --ws-external` or run the command `cargo run -- --dev --tmp` in this repository.
2. A correctly configured [Polkadot JS UI](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer), you may need the types which you can find [here](../types.json).

## Testing
### Create an organization
Submit the extrinsic `organizations.create` with the following parameters:
- `RuntimeVotingSystemId`: set it to `ConvictionVoting`, this is how you can change an organization's voting system.
- `RuntimeVotingParameters` set it to `ConvictionVoting` as well.
- `ttl`: for testing set it to `10`, this means that we will be able to close a proposal 10 blocks after its creation.
- `voting_currency`: this lets us choose which currency will represent votes, keep it to `Native`.
- `min_quorum`: set it to `50`, this is the minimum quorum that must be reached for a proposal to be considered passing.
- `min_participation`: set it to `33`, if less than `33%` of conviction (as computed from the token total supply) is in favor of the vote then the proposal will be considered as failing.

Submit the transaction, the event `organizations.OrganizationCreated` should be triggered and contain an account id, keep it somewhere as it is the address of the organization you created. In our case it was `5EYCAe5gvgRHjJhnqvSAGV55RLCi7RWEMW5KdZiaE4ztFZ67` but this may change if you created other organizations before.

### Create a proposal
You can now submit a proposal via the extrinsic `organizations.createProposal`, we used the following parameters:
- `org_id`: the address of the organization you created.
- `call`: what the proposal would execute if it passes. We just used a simple `system.remark(0x00)` call.

Once you submit the transaction, the event `organizations.ProposalSubmitted` is triggered. It contains a `proposal_id` that you can copy and use to view the state of the given proposal. In our case it was `0xfb587ba9aa3e1450cd8894dde59ddf7d64bd855b41d415be7cf4e775287f305d` but this may be different for you.

You should also be able to view the proposal via the chain state `convictionVoting.proposals`, it should contain informations regarding current votes, when the proposal was created etc.

### Cast some votes
Submit the extrinsic `organizations.decideOnProposal` with the following parameters:
- `proposal_id` is the id of the proposal you want to cast the vote for. In our case it is `0xfb587ba9aa3e1450cd8894dde59ddf7d64bd855b41d415be7cf4e775287f305d`.
- `vote_data` lets you set the vote you want to cast, set it to `ConvictionVoting` for now and fill the following fields:
   - `in_support`: wether the account supports the proposal, we set it to `true`.
   - `power`: how much tokens are locked in favor of this proposal, we set it to `1M` units. Later on, it will accumulate over time as other votes are casted or time elapses.

Once you submit the transaction the event `organizations.ProposalVoteCasted` should be triggered. The chain state of the proposal should have been mutated as well.

> The development chain spec we are using grants a little over 1M of `Native` currency to `Alice` and `Bob`. This is why we vote with 1M units in order to reach the minimum participation criteria.

Feel free to submit more votes with other accounts. As usual, if you submit a new vote with an account that already voted its vote will simplpy be modified.

### Close and execute the proposal
Assuming you casted a favorable vote previously and didn't add any against the proposal you should be able to close and execute the proposal now. To do so, submit the extrinsic `organizations.closeProposal` with the following parameters:
- `proposal_id` is the id of the proposal you want to cast the vote for. In our case it is `0xfb587ba9aa3e1450cd8894dde59ddf7d64bd855b41d415be7cf4e775287f305d`.
- `proposal_weight_bound` is used for weight computation and is adjusted after the proposal is executed. We set it to `3000000` which should be enough for our `system.remark` call.

After submitting the transaction you should see the event `organizations.ProposalClosed` which will indicate wether the proposal passed or not.