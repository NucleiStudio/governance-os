import React, { useState } from 'react';
import { Form, Input, Button, Label, Icon, Dropdown } from 'semantic-ui-react';
import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

/// This component is in charge of handling votes to the PLCR voting scheme.
/// It is a bit different from Binary voting since votes have first to be
/// 'committed' before being 'revealed'.
function Main(props) {
    const { api } = useSubstrate();
    const { accountPair, proposalId, proposalDetails, setTxStatus } = props;

    // How many coins are used to support the vote
    const [support, setSupport] = useState(0);
    // Wether we are in favor or against the proposal
    const [favorable, setFavorable] = useState(true);
    // Salt for the commit / reveal hashing
    const [salt, setSalt] = useState(0);

    const genHash = () => {
        // Leverage the Substrate API to auto hash our tuple type.
        const payload = api.createType('(Balance, bool, u64)', [support, favorable, salt]);
        return payload.hash;
    };

    return (
        <div>
            <Form.Field>
                <Input
                    placeholder='0'
                    fluid
                    type='number'
                    label='Salt'
                    onChange={(_, { value }) => setSalt(value)}
                />
            </Form.Field>
            <Form.Field>
                <Dropdown
                    placeholder='Decision'
                    search
                    selection
                    options={[
                        {
                            key: 'in_favor',
                            value: true,
                            text: 'Favorable',
                        }, {
                            key: 'against',
                            value: false,
                            text: 'Against',
                        }
                    ]}
                    onChange={(_, { value }) => setFavorable(value)}
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
                    onChange={(_, { value }) => setSupport(value)}
                />
            </Form.Field>
            <Form.Field style={{ textAlign: 'center' }}>
                <Button.Group>
                    <TxButton
                        accountPair={accountPair}
                        label='Commit'
                        type='SIGNED-TX'
                        color='green'
                        setStatus={setTxStatus}
                        attrs={{
                            palletRpc: 'organizations',
                            callable: 'decideOnProposal',
                            inputParams: [proposalId, { [proposalDetails.voting]: { 'Commit': genHash() } }],
                            paramFields: [true, true]
                        }}
                    />
                    <Button.Or />
                    <TxButton
                        accountPair={accountPair}
                        label='Reveal'
                        type='SIGNED-TX'
                        color='blue'
                        setStatus={setTxStatus}
                        attrs={{
                            palletRpc: 'organizations',
                            callable: 'decideOnProposal',
                            inputParams: [proposalId, { [proposalDetails.voting]: { 'Reveal': [support, favorable, salt] } }],
                            paramFields: [true, true]
                        }}
                    />
                </Button.Group>
            </Form.Field>
        </div>
    );
}

export default function PlcrVoting(props) {
    const { api } = useSubstrate();
    return api.query.organizations &&
        api.query.organizations.proposals &&
        api.query.plcrVoting ? <Main {...props} /> : null;
}
