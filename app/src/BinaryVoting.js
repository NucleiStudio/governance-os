import React, { useState } from 'react';
import { Form, Input, Button, Label, Icon } from 'semantic-ui-react';
import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

/// This component is in charge of handling votes for Binary Voting systems
/// such as Conviction and Coin voting. A 'Binary' voting system is a system
/// where you can either vote yes or no.
function Main(props) {
    const { accountPair, proposalId, proposalDetails, setTxStatus } = props;

    // Amount of coins supporting the vote
    const [support, setSupport] = useState(0);

    return (
        <div>
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
                        label='Vote For'
                        type='SIGNED-TX'
                        color='green'
                        setStatus={setTxStatus}
                        attrs={{
                            palletRpc: 'organizations',
                            callable: 'decideOnProposal',
                            inputParams: [proposalId, { [proposalDetails.voting]: { in_support: true, power: support } }],
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
                            inputParams: [proposalId, { [proposalDetails.voting]: { in_support: false, power: parseInt(support) } }],
                            paramFields: [true, true]
                        }}
                    />
                </Button.Group>
            </Form.Field>
        </div>
    );
}

export default function BinaryVoting(props) {
    const { api } = useSubstrate();
    return api.query.organizations &&
        api.query.organizations.proposals &&
        api.query.coinVoting &&
        api.query.convictionVoting ? <Main {...props} /> : null;
}
