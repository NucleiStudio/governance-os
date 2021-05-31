import React, { useState } from 'react';
import { Grid, Form, Dropdown, Input } from 'semantic-ui-react';
import config from './config';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

/// This component generates a form to create a new organization.
function Main(props) {
    const { accountPair } = props;

    // Tx status
    const [status, setStatus] = useState(null);
    // Org parameters that we will send as part of the creation tx
    const [parameters, setParameters] = useState({})
    // Complete form state which we use to derive the final tx
    const [formState, setFormState] = useState({
        executors: '',
        votingSystem: '',
        votingParams: {},
    });

    const onVotingSystemChanged = (_, { value }) => {
        // Voting system changed. Do some wizardy to get all the parameters
        // a user has to define. We check the JSON types to do this.

        const paramsTypesName = config.types["RuntimeVotingParameters"]["_enum"][value];
        setFormState({ ...formState, votingSystem: value, votingParams: {} });
        setParameters(config.types[paramsTypesName]);
    };

    const onParamValueChanged = (_, { label, value }) => {
        // An org voting parameter changed. Update state.

        let paramsCopy = formState.votingParams;
        paramsCopy[label] = value;

        setFormState({ ...formState, votingParams: paramsCopy });
    };

    // Go through the JSON types to list and create a dropdown of
    // our different voting systems.
    const votingSystemsOptions = Object.keys(config.types["RuntimeVotingSystemId"]["_enum"]).map((voting) => ({
        key: voting,
        value: voting,
        text: voting,
    }));

    return (
        <Grid.Column width={8}>
            <h1>Create a new Organization</h1>
            <Form>
                <Form.Field>
                    <Dropdown
                        placeholder='Voting System'
                        fluid
                        label='Voting System'
                        onChange={onVotingSystemChanged}
                        search
                        selection
                        state='votingSystem'
                        options={votingSystemsOptions}
                    />
                </Form.Field>
                {Object.keys(parameters).map((paramName, ind) =>
                    <Form.Field key={`${paramName}-${parameters[paramName]}`}>
                        <Input
                            placeholder={parameters[paramName]}
                            fluid
                            type='text'
                            label={paramName}
                            onChange={onParamValueChanged}
                        />
                    </Form.Field>
                )}
                <Form.Field style={{ textAlign: 'center' }}>
                    <TxButton
                        accountPair={accountPair}
                        label='Create'
                        type='SIGNED-TX'
                        setStatus={setStatus}
                        attrs={{
                            palletRpc: 'organizations',
                            callable: 'create',
                            inputParams: [{ executors: formState.executors, voting: [formState.votingSystem, { [formState.votingSystem]: formState.votingParams }] }],
                            paramFields: [true]
                        }}
                    />
                </Form.Field>
                <div style={{ overflowWrap: 'break-word' }}>{status}</div>
            </Form>
        </Grid.Column>
    );
}

export default function CreateOrganization(props) {
    const { api } = useSubstrate();
    return api.query.organizations && api.query.organizations.parameters ? <Main {...props} /> : null;
}
