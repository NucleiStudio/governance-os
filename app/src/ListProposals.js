import React, { useEffect, useState } from 'react';
import { Grid, Dropdown, Form } from 'semantic-ui-react';
import { useSubstrate } from './substrate-lib';

import BinaryVoting from './BinaryVoting';
import Close from './Close';
import PlcrVoting from './PlcrVoting';

function Main(props) {
    const { api } = useSubstrate();
    const { accountPair, orgs } = props;

    const [txStatus, setTxStatus] = useState(null);
    const [propsDropdownOptions, setPropsDropdownOptions] = useState([]);
    const [selectedProp, setSelectedProp] = useState(null);
    const [allProposals, setAllProposals] = useState({});
    const [propDetails, setPropDetails] = useState({});
    const [propWeight, setPropWeight] = useState(0);
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
                    const parsedProposals = proposalIds.reduce((acc, propId, index) => ({
                        ...acc, [propId]: details[index].unwrap()
                    }), {});

                    setAllProposals(parsedProposals);
                }).then(unsub => {
                    unsubscribe = unsub;
                }).catch(console.error);
            });
        }, 2000);

        return () => clearInterval(interval);
    }, [api.query.organizations.proposals]);

    const parseCall = (call) => {
        let parsed = api.createType('Call', call);
        return `${parsed.section}.${parsed.method}(${parsed.args})`;
    };

    const onSelectedOrgChange = (_, { value }) => {
        setSelectedProp(null);
        setPropDetails({});
        setPropsDropdownOptions(
            Object.keys(allProposals)
                .filter((prop) => {
                    return allProposals[prop].org == value;
                })
                .map((prop) => ({
                    key: prop,
                    value: prop,
                    text: parseCall(allProposals[prop].call),
                }))
        );
        setUiFlavor('');
    };

    const coinVotingCanClose = (proposalHash, cannotCloseCb) => {
        api.query.coinVoting.proposals(proposalHash, state => {
            console.log(JSON.stringify(state));

            const currencyId = state.parameters["voting_currency"];
            const totalParticipation = state["total_favorable"].add(state["total_against"]);
            const minParticipation = state.parameters["min_participation"] / 100;
            const minQuorum = state.parameters["min_quorum"] / 100;
            const totalFavorable = state["total_favorable"];
            const createdOn = state["created_on"].toNumber();
            const ttl = state.parameters.ttl.toNumber();

            api.query.tokens.totalIssuances(currencyId, totalSupply => {
                const enoughParticipation = totalParticipation > minParticipation * totalSupply;
                const enoughQuorum = totalFavorable > minQuorum * totalParticipation;
                const proposalPassing = enoughParticipation && enoughQuorum;

                let unsub = null;
                api.derive.chain.bestNumber(now => {
                    const proposalExpired = now > createdOn + ttl;

                    if (unsub !== null) {
                        unsub();
                    }

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
        // we only support closing a conviction voting proposal once it is expired
        // as conviction accumulates over time, it needs to be computed regularly
        // and this would amount to lot of duplicated code here. in the future,
        // we may add a RPC call for it.

        api.query.convictionVoting.proposals(proposalHash, state => {
            console.log(JSON.stringify(state));

            const createdOn = state["created_on"].toNumber();
            const ttl = state.parameters.ttl.toNumber();

            let unsub = null;
            api.derive.chain.bestNumber(now => {
                const proposalExpired = now > createdOn + ttl;

                if (unsub !== null) {
                    unsub();
                }

                if (proposalExpired) {
                    setUiFlavor('close');
                } else {
                    cannotCloseCb();
                }
            })
                .then(u => unsub = u)
                .catch(console.error);
        }).catch(console.error);
    };

    const plcrVotingCanClose = (proposalHash, cannotCloseCb) => {
        api.query.plcrVoting.proposals(proposalHash, state => {
            console.log(JSON.stringify(state));

            const currencyId = state.parameters["voting_currency"];
            const totalParticipation = state["revealed_favorable"].add(state["revealed_against"]);
            const minParticipation = state.parameters["min_participation"] / 100;
            const minQuorum = state.parameters["min_quorum"] / 100;
            const totalFavorable = state["revealed_favorable"];
            const createdOn = state["created_on"].toNumber();
            const ttl = state.parameters["commit_duration"].add(state.parameters["reveal_duration"]).toNumber();

            api.query.tokens.totalIssuances(currencyId, totalSupply => {
                const enoughParticipation = totalParticipation > minParticipation * totalSupply;
                const enoughQuorum = totalFavorable > minQuorum * totalParticipation;
                const proposalPassing = enoughParticipation && enoughQuorum;

                let unsub = null;
                api.derive.chain.bestNumber(now => {
                    const proposalExpired = now > createdOn + ttl;

                    if (unsub !== null) {
                        unsub();
                    }

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
