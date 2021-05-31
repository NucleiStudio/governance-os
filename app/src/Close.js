import React from 'react';
import { Form, Label, Icon } from 'semantic-ui-react';
import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

/// This component is in charge of handling 'close_proposal'
/// transactions for a given proposal. It simply presents a
/// `TxButton` component to send it.
function Main(props) {
    const { accountPair, proposalId, proposalWeight, setTxStatus } = props;

    return (
        <div>
            <Form.Field>
                <Label basic color='teal'>
                    <Icon name='hand point right' />
                        You are going to close this proposal which may trigger its execution
                </Label>
            </Form.Field>
            <Form.Field style={{ textAlign: 'center' }}>
                <TxButton
                    accountPair={accountPair}
                    label='Close'
                    type='SIGNED-TX'
                    color='blue'
                    setStatus={setTxStatus}
                    attrs={{
                        palletRpc: 'organizations',
                        callable: 'closeProposal',
                        inputParams: [proposalId, proposalWeight],
                        paramFields: [true, true]
                    }}
                />
            </Form.Field>
        </div >
    );
}

export default function Close(props) {
    const { api } = useSubstrate();
    return api.query.organizations &&
        api.query.organizations.proposals ? <Main {...props} /> : null;
}
