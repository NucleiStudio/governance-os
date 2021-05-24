import React, { useEffect, useState } from 'react';
import { Grid, Message, Dropdown, Form } from 'semantic-ui-react';
import { useSubstrate } from './substrate-lib';

import BinaryVoting from './BinaryVoting';
import PlcrVoting from './PlcrVoting';

function Main(props) {
    const { api } = useSubstrate();
    const { accountPair, orgs } = props;

    const [txStatus, setTxStatus] = useState(null);
    const [propsDropdownOptions, setPropsDropdownOptions] = useState([]);
    const [selectedProp, setSelectedProp] = useState(null);
    const [allProposals, setAllProposals] = useState({});
    const [propDetails, setPropDetails] = useState({});
    const [uiFlavor, setUiFlavor] = useState('');

    useEffect(() => {
        let unsubscribe;

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

        return () => unsubscribe && unsubscribe();
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

    const onSelectedProposalChange = (_, { value }) => {
        setSelectedProp(value);
        setPropDetails(allProposals[value]);

        window.call = allProposals[value].call;

        if (allProposals[value].voting.toHuman() !== 'PlcrVoting') {
            setUiFlavor('binary');
        } else {
            setUiFlavor('plcr');
        }
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
        api.query.convictionVoting ? <Main {...props} /> : null;
}
