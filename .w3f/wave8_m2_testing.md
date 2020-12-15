# Milestone 2 Testing

Milestone 2 covered the development of an `organization` pallet to create and manage decentralized organizations on the fly.

## Changes from milestone 1
In milestone 1, `ALICE` was granted the `root` role, this is no longer the case. Instead we have created a demonstration organization (under address `5EYCAe5gvgRHjJhnqvRtXTXVJqXWesQMcq8p5d2jmF89z84d`) which has the role. `ALICE` was granted the ability to execute calls from it without going through a voting procedure, other accounts can create and debate on votes based on the following parameters:
- votes are represented and denominated in our `Native` currency.
- creating a new proposal will cost you `10` units of currency which will be counted in favor of it.
- at least `33% + 1` of the circulating coins need to participate in the votes.
- for a proposal to pass, at least `50% + 1` of the circulating coins need to have been casted in favor of the proposal.
- if a proposal fills all the participation and quorum criteria it can be "closed" and executed early.
- if a proposal does not satisfy our criteria or is not passing, it can be "closed" but not executed after `50` blocks passed - we kept this number small to ease testing.

## Requirements
You will need a few things first:
1. A running local node, the easiest way to create one is to use this command: `docker run --rm -p 9944:9944 -it eteissonniere/governance-os --dev --tmp --ws-external`.
2. A correctly configured [Polkadot JS UI](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer), you may need the types which you can find [here](../types.json).

## `organizations` pallet
Our goal when creating the `organizations` pallet was to keep it modular enough so that people would be able to create, and potentially implement, their own governance models. Runtime or contracts (pending the next substrate release :smirk:) developers should be able to develop custom and advanced models while users can select their own parameters.

### Creating an organization
In the Governance OS, an organization is treated as a first class citizen of the chain. It is represented by an account id just like any other users and can receive, send funds or dispatch any extrinsics. This is a bit different from the traditional patterns which involves the creation of a distinct treasury which is then controlled by a separate pallet such as `democracy` or `collective` as we can see in most runtimes.

As you will see very soon, an organization is created from two distinct parameters:
- A list of `executors`, potentially empty, this reference accounts that can trigger calls from the organizations without going through a voting procedure. This could be other organizations, `ink!` contracts or other users.
- A **voting system**, picked from a selection of runtime provided systems. We currently support two configurations: no voting system at all (if you'd like to use executors only or have the voting system be managed through a smart contract) and a **coin based** one (basically one token equals one vote).

Enough reading, let's create our own organization!

Submit the following extrinsic: `organizations.create` with the following parameters:
- add `ALICE` to the list of executors (click the "Add Item" button)
- choose the `CoinBased` voting system with the following parameters:
  - `voting_currency` to `Native`, this means you are using the same currency that the one used to pay for fees to represent votes and voting power. You could also use another one if you created it previously.
  - `creation_fee` to 10 units. Anybody creating a proposal will have to reserve 10 coins.
  - `min_quorum` of 50. This means that 50% + 1 coins of the total circulating supply need to be locked in favor of the proposal.
  - `min_participation` to 33. At least 33% + 1 of the circulating coins of the voting currency need to participate in the vote.
  - `ttl` to 10. We keep it short for testing, but basically after 10 blocks we will be able to close a non passing proposal, in a real world scenario this would be a bigger number.

Submit and look for incoming events `organizations.OrganizationCreated` will be fired with an address which is the one of the organization. Keep it somewhere as we will need it. If you didn't create any other organization before it should be `5EYCAe5gvgRHjJhnqvSAGV55RLCi7RWEMW5KdZiaE4ztFZ67` (we use a counter to deterministically generate those).

You can now get more details about the organization by querying the chain state `organizations.parameters(5EYCAe5gvgRHjJhnqvSAGV55RLCi7RWEMW5KdZiaE4ztFZ67)`.

### Using a previously created organization
#### As an executor
Using the organization address submit the extrinsic `organizations.applyAs(org. addr, extrinsic)`, in our case we did (using `ALICE`) `organizations.applyAs(5EYCAe5gvgRHjJhnqvSAGV55RLCi7RWEMW5KdZiaE4ztFZ67, system.remark(0x00))`.

An event `organizations.OrganizationExecuted` will be fired to log the extrinsic's result. Since it is dispatched from the organization account id it could have been a balance transfer for instance, provided that the organisation had received some coins before.

#### As a voter
##### Creating a proposal
We are going to create two proposals for the organization `5EYCAe5gvgRHjJhnqvSAGV55RLCi7RWEMW5KdZiaE4ztFZ67` (change the address as needed but you shouldn't have to if you followed all the instructions), we will oppose one of them and support the other one later. For a change, let's submit the calls from `BOB` which happens to have some coins in its balance as well.

Proposal 1 will be created via `organizations.createProposal(5EYCAe5gvgRHjJhnqvSAGV55RLCi7RWEMW5KdZiaE4ztFZ67, system.remark(0xdeadbeef))`.

Proposal 2 will be created via `organizations.createProposal(5EYCAe5gvgRHjJhnqvSAGV55RLCi7RWEMW5KdZiaE4ztFZ67, system.remark(0x42))`.

For each proposal an event `organizations.ProposalSubmitted` is fired with a hashed proposal id, in our case we got:
- proposal 1: `0xf287ad503c098de778ae666183e505a33e1e282b83c4556216dd36fcaa39b465`
- proposal 2: `0xb88d6a6958e7f5f0f447d9aab0489143409172496a94d1722201c5182aadcc4a`

The IDs are generated deterministically from the proposals themselves so that if you submitted the same extrinsic you should have the same IDs. If not, you will need to use your own IDs. One cool thing about this is that we can avoid duplicated opened proposals in an organization. If you try to submit an already existing proposal for the same organization you will get an error. Hopefully, this should limit spamming in a real world scenario.

We can also view all the proposal's details and "state" (as in, votes and other metadata) by querying the chain state. For instance for proposal 1 we would do `organizations.proposals(0xf287ad503c098de778ae666183e505a33e1e282b83c4556216dd36fcaa39b465)`.

Also note that based on our parameters the proposal creator will need to have at least 10 coins locked into the proposal. Even though users can edit their votes the proposal creator will always need to keep at least 10 coins there until the proposal gets closed. This is used to prevent spamming and ensure that the creator of the proposal has funds at stake, of course one could set this value to 0 when creating an organization.

> In the dev chain configuration both `ALICE` and `BOB` have `1.1529k` units of currency. Since we use the `Native` currency for our examples we are going to create votes with high stakes to match the minimum participation criteria.

##### Voting against a proposal
Let's have `ALICE` oppose proposal 1 by submitting the following extrinsic: `organizations.decideOnProposal(0xf287ad503c098de778ae666183e505a33e1e282b83c4556216dd36fcaa39b465, 1000, No)` from her account.

Note that the event `organizations.ProposalVoteCasted` is casted, the chain state for the proposal should be updated as well. If you view the chain state `system.accounts(ALICE)` you will see 1000 units of currency reserved as well.

##### Voting in favor of a proposal
Since `BOB` saw `ALICE` opposing its first proposal let's have him block more coins in favor of the second one by submitting `organizations.decideOnProposal(0xb88d6a6958e7f5f0f447d9aab0489143409172496a94d1722201c5182aadcc4a, 1000, Yes)` from his account.

The event should be fired as well and similar chain state changes should happen that with `ALICE`.

##### Updating votes
Let's play a bit more. Since `ALICE` saw `BOB` locking so many coins on proposal 2 she knows that he won't be able to balance her own decision on proposal 1, so she decides to withdraw `600` coins from her position. To do this she will simply create a new vote in opposition of proposal 1 but with a `power` (or stake) of `400`. The chain will detect that she already had voted on this proposal and update her position to only `400` coins thus freeing `600` units: submit `organizations.decideOnProposal(0xf287ad503c098de778ae666183e505a33e1e282b83c4556216dd36fcaa39b465, 400, No)` from her account.

When viewing the chain state `system.accounts(ALICE)` you will see that `ALICE` now has only `400` coins reserved.

This can be used if a voter change its mind or if somebody would like to cancel a vote.

##### Closing a proposal
The pallet does not auto execute or delete proposals, we need to *close* them by submitting one last extrinsic. When doing the closing call the system will detect wether it needs to execute the proposal or not. Anybody can submit the closing extrinsic on any closeable proposal, a proposal is closeable if it is passing (i.e. minimum quorum and participation rate conditions are met) or if it is expired.

We submit the following two extrinsics from either `ALICE` or `BOB`:
- `organizations.closeProposal(0xf287ad503c098de778ae666183e505a33e1e282b83c4556216dd36fcaa39b465, 3000000)`
- `organizations.closeProposal(0xb88d6a6958e7f5f0f447d9aab0489143409172496a94d1722201c5182aadcc4a, 3000000)`

You may have noticed a `proposal_weight_bound` parameter, this is used for weight estimation and is adjusted after the proposal execution (or non execution).

You should see a few events:
- two `organizations.ProposalClosed` for each proposal which logs the result of votes, with a `Yes` for proposal 2 and a `No` for proposal 1.
- one `organizations.ProposalExecuted` for proposal 2 which logs the ID of the proposal and an extrinsic result.

#### Organizations wide calls
We also offer two "organization restricted" calls, meaning that they can only be executed by the organization itself, this can happen by using `applyAs` if allowed to (for instance, for `ALICE` since she is an `executor` based on our initial parameters) or by having another proposal execute them through a separate vote. The `organizations.vetoProposal` can be used to close a proposal but never execute it, this could be useful if we want some kind of supervisory entity to watch over the organization. While `organizations.mutate` can be used to change an organization's details (i.e. who can be an executor and what voting system is in use). By design, changing an organization's voting system will not change it for opened proposal, which means that non closed proposal at the time of the parameter changes will continue using the old parameters until they are closed.