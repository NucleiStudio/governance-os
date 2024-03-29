import React, { useEffect, useState } from 'react';
import { Grid, Dropdown, Form } from 'semantic-ui-react';
import { useSubstrate } from './substrate-lib';

import BinaryVoting from './BinaryVoting';
import Close from './Close';
import PlcrVoting from './PlcrVoting';

import { parseCall, coinVotingState, convictionVotingState, plcrVotingState } from './helpers';

/// This component is in charge two things: listing proposals
/// for each organizations and letting users vote on them.
function Main(props) {
    const { api } = useSubstrate();
    const { accountPair, orgs } = props;

    // tx status
    const [txStatus, setTxStatus] = useState(null);
    // dropdown options for each available proposals
    const [propsDropdownOptions, setPropsDropdownOptions] = useState([]);
    // which proposal we selected, this is used to display the
    // correct forms
    const [selectedProp, setSelectedProp] = useState(null);
    // every single proposals in the system
    const [allProposals, setAllProposals] = useState({});
    // details about the current proposal
    const [propDetails, setPropDetails] = useState({});
    // computed weight of the current proposal - used for closing
    const [propWeight, setPropWeight] = useState(0);
    // which 'ui flavor' we want to display, basically wether we
    // want to:
    // - present a binary voting form
    // - present a plcr voting form
    // - present a close proposal form
    const [uiFlavor, setUiFlavor] = useState('');

    useEffect(() => {
        let unsubscribe;

        const interval = setInterval(() => {
            if (unsubscribe) {
                // unsub from previous subs
                unsubscribe();
            }

            api.query.organizations.proposals.keys().then(proposals => {
                const proposalIds = proposals.map(({ args: [proposalId] }) => proposalId.toHuman());

                api.query.organizations.proposals.multi(proposalIds, (details) => {
                    // due to the timestamp we may have a bit of delay to unsub and
                    // cleanup the list of `proposalIds` which may lead to unwrapping
                    // errors, which we definitely want to avoid - so we use a try
                    // catch to avoid this

                    try {
                        const parsedProposals = proposalIds.reduce((acc, propId, index) => ({
                            ...acc, [propId]: details[index].unwrap()
                        }), {});

                        setAllProposals(parsedProposals);
                    } catch (err) {
                        console.error(err);
                    }
                }).then(unsub => {
                    unsubscribe = unsub;
                }).catch(console.error);
            });
        }, 2000);

        return () => clearInterval(interval);
    }, [api.query.organizations.proposals]);

    const onSelectedOrgChange = (_, { value }) => {
        // A new org was selected, do a bit of wizardy

        // No proposal is currently selected
        setSelectedProp(null);
        // If no proposal is selected, there are no details
        // about it
        setPropDetails({});
        // Make sure to display only the proposals for this org
        setPropsDropdownOptions(
            Object.keys(allProposals)
                .filter((prop) => {
                    return allProposals[prop].org == value;
                })
                .map((prop) => ({
                    key: prop,
                    value: prop,
                    text: parseCall(api, allProposals[prop].call),
                }))
        );

        console.log(JSON.stringify(allProposals));

        // Since no proposal is selected we don't know which
        // form we can show yet
        setUiFlavor('');
    };

    const coinVotingCanClose = (proposalHash, cannotCloseCb) => {
        // If the proposal can be closed, setUiFlavor to 'close',
        // otherwise call the cannotCloseCb function.
        // This basically reimplement the code from the substrate
        // pallet in JS.

        api.query.coinVoting.proposals(proposalHash, state => {
            console.log(JSON.stringify(state));

            const currencyId = state.parameters["voting_currency"];

            api.query.tokens.totalIssuances(currencyId, totalSupply => {
                let unsub = null;
                api.derive.chain.bestNumber(now => {
                    if (unsub !== null) {
                        // Auto unsub, we don't need recurrent block number
                        // updates
                        unsub();
                    }

                    const [proposalPassing, proposalExpired] = coinVotingState(state, totalSupply, now);

                    if (proposalPassing || proposalExpired) {
                        setUiFlavor('close');
                    } else {
                        cannotCloseCb();
                    }
                })
                    .then(u => unsub = u)
                    .catch(console.error);
            }).catch(console.error);
        }).catch(console.error);
    };

    const convictionVotingCanClose = (proposalHash, cannotCloseCb) => {
        // If the proposal can be closed, setUiFlavor to 'close',
        // otherwise call the cannotCloseCb function.
        // This basically reimplement the code from the substrate
        // pallet in JS.

        api.query.convictionVoting.proposals(proposalHash, state => {
            console.log(JSON.stringify(state));

            const currencyId = state.parameters["voting_currency"];

            api.query.tokens.totalIssuances(currencyId, totalSupply => {
                let unsub = null;
                api.derive.chain.bestNumber(now => {
                    if (unsub !== null) {
                        // Auto unsub, we don't need recurrent block number
                        // updates
                        unsub();
                    }

                    const [proposalPassing, proposalExpired] = convictionVotingState(state, totalSupply, now);

                    if (proposalPassing || proposalExpired) {
                        setUiFlavor('close');
                    } else {
                        cannotCloseCb();
                    }
                })
                    .then(u => unsub = u)
                    .catch(console.error);
            }).catch(console.error);
        }).catch(console.error);
    };

    const plcrVotingCanClose = (proposalHash, cannotCloseCb) => {
        // If the proposal can be closed, setUiFlavor to 'close',
        // otherwise call the cannotCloseCb function.
        // This basically reimplement the code from the substrate
        // pallet in JS.

        api.query.plcrVoting.proposals(proposalHash, state => {
            console.log(JSON.stringify(state));

            const currencyId = state.parameters["voting_currency"];

            api.query.tokens.totalIssuances(currencyId, totalSupply => {
                let unsub = null;
                api.derive.chain.bestNumber(now => {
                    if (unsub !== null) {
                        // Auto unsub, we don't need recurrent block number
                        // updates
                        unsub();
                    }

                    const [proposalPassing, proposalExpired] = plcrVotingState(state, totalSupply, now);

                    if (proposalPassing || proposalExpired) {
                        setUiFlavor('close');
                    } else {
                        cannotCloseCb();
                    }
                })
                    .then(u => unsub = u)
                    .catch(console.error);
            }).catch(console.error);
        }).catch(console.error);
    };

    const canClose = (proposal, proposalHash, cannotCloseCb) => {
        // Router between all the other canClose functions. This
        // routes the proposal to the correct function 
        if (proposal.voting.toHuman() === 'CoinVoting') {
            coinVotingCanClose(proposalHash, cannotCloseCb);
        } else if (proposal.voting.toHuman() === 'ConvictionVoting') {
            convictionVotingCanClose(proposalHash, cannotCloseCb);
        } else if (proposal.voting.toHuman() === 'PlcrVoting') {
            plcrVotingCanClose(proposalHash, cannotCloseCb);
        } else {
            cannotCloseCb();
        }
    };

    const selectUiFlavor = (proposalHash) => {
        canClose(allProposals[proposalHash], proposalHash, () => {
            if (allProposals[proposalHash].voting.toHuman() !== 'PlcrVoting') {
                setUiFlavor('binary');
            } else {
                setUiFlavor('plcr');
            }
        });
    };

    const onSelectedProposalChange = (_, { value }) => {
        setSelectedProp(value);
        setPropDetails(allProposals[value]);
        selectUiFlavor(value);

        // a random address that we are using for our queries
        const ZERO_ACCOUNT = '5CAUdnwecHGxxyr5vABevAfZ34Fi4AaraDRMwfDQXQ52PXqg';
        api.tx(api.createType('Call', allProposals[value].call))
            .paymentInfo(ZERO_ACCOUNT)
            .then(({ weight }) => setPropWeight(weight))
            .catch(console.error);
    };

    // For some reason `orgs.map` is undefined
    const orgDropdownOptions = Object.keys(orgs).map((addr) => ({
        key: addr,
        value: addr,
        text: addr,
    }));

    return (
        <Grid.Column width={8}>
            <h1>Explore Proposals</h1>
            <Form>
                <Form.Field>
                    <Dropdown
                        placeholder='Organization Address'
                        search
                        selection
                        options={orgDropdownOptions}
                        onChange={onSelectedOrgChange}
                    />
                </Form.Field>
                <Form.Field>
                    <Dropdown
                        placeholder='Proposal Id'
                        search
                        selection
                        options={propsDropdownOptions}
                        onChange={onSelectedProposalChange}
                    />
                </Form.Field>
                {
                    uiFlavor === 'close' &&
                    <Close
                        accountPair={accountPair}
                        proposalId={selectedProp}
                        proposalWeight={propWeight}
                        setTxStatus={setTxStatus}
                    />
                }
                {
                    uiFlavor === 'binary' &&
                    <BinaryVoting
                        accountPair={accountPair}
                        proposalId={selectedProp}
                        proposalDetails={propDetails}
                        setTxStatus={setTxStatus}
                    />
                }
                {
                    uiFlavor === 'plcr' &&
                    <PlcrVoting
                        accountPair={accountPair}
                        proposalId={selectedProp}
                        proposalDetails={propDetails}
                        setTxStatus={setTxStatus}
                    />
                }
                <div style={{ overflowWrap: 'break-word' }}>{txStatus}</div>
            </Form>
        </Grid.Column>
    );
}

export default function ListProposals(props) {
    const { api } = useSubstrate();
    return api.query.organizations &&
        api.query.organizations.proposals &&
        api.query.coinVoting &&
        api.query.plcrVoting &&
        api.query.convictionVoting &&
        api.query.tokens &&
        api.query.tokens.totalIssuances &&
        api.derive.chain.bestNumber ? <Main {...props} /> : null;
}
