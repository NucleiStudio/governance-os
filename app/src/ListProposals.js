import React, { useEffect, useState } from 'react';
import { Grid, Message, Dropdown, Form, Input, Button, Label, Icon } from 'semantic-ui-react';
import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

function Main(props) {
    const { api } = useSubstrate();
    const { accountPair, orgs } = props;

    const [txStatus, setTxStatus] = useState(null);
    const [propsDropdownOptions, setPropsDropdownOptions] = useState([]);
    const [support, setSupport] = useState(0);
    const [selectedProp, setSelectedProp] = useState(null);
    const [allProposals, setAllProposals] = useState({});
    const [propDetails, setPropDetails] = useState({});

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
                    text: allProposals[prop].call.toHuman(), // TODO: decode this
                }))
        );
    };

    const onSelectedProposalChange = (_, { value }) => {
        setSelectedProp(value);
        setPropDetails(allProposals[value]);
    };

    const metadataExplainer = () => {
        if (propDetails.voting !== undefined) {
            return `This proposal uses the ${propDetails.voting} voting system.`;
        }

        return 'Please select an organization and associated proposal.';
    }

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
                <Form.Field>
                    <Message
                        content={metadataExplainer()}
                    />
                </Form.Field>
                <Form.Field>
                    <Label basic color='teal'>
                        <Icon name='hand point right' />
                        1 Unit = 1000000000000&nbsp;
                    </Label>
                </Form.Field>
                <Form.Field>
                    <Input
                        placeholder='1000000000000'
                        fluid
                        type='number'
                        label='Support'
                        onChange={setSupport}
                    />
                </Form.Field>
                <Form.Field style={{ textAlign: 'center' }}>
                    <Button.Group>
                        <TxButton
                            accountPair={accountPair}
                            label='Vote For'
                            type='SIGNED-TX'
                            color='green'
                            setStatus={setTxStatus}
                            attrs={{
                                palletRpc: 'organizations',
                                callable: 'decideOnProposal',
                                inputParams: [selectedProp, { [propDetails.voting]: { in_support: true, power: parseInt(support) } }],
                                paramFields: [true, true]
                            }}
                        />
                        <Button.Or />
                        <TxButton
                            accountPair={accountPair}
                            label='Vote Against'
                            type='SIGNED-TX'
                            color='red'
                            setStatus={setTxStatus}
                            attrs={{
                                palletRpc: 'organizations',
                                callable: 'decideOnProposal',
                                inputParams: [selectedProp, { [propDetails.voting]: { in_support: false, power: parseInt(support) } }],
                                paramFields: [true, true]
                            }}
                        />
                    </Button.Group>
                </Form.Field>
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
