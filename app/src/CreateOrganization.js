import React, { useState } from 'react';
import { Grid, Form, Dropdown, Input } from 'semantic-ui-react';
import config from './config';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

function Main(props) {
    const { api } = useSubstrate();
    const { accountPair } = props;

    const [status, setStatus] = useState(null);
    const [parameters, setParameters] = useState({})
    const [formState, setFormState] = useState({
        executors: '',
        votingSystem: '',
        votingParams: {},
    });

    const onVotingSystemChanged = (_, { value }) => {
        const paramsTypesName = config.types["RuntimeVotingParameters"]["_enum"][value];
        setFormState({ ...formState, votingSystem: value, votingParams: {} });
        setParameters(config.types[paramsTypesName]);
    };

    const onParamValueChanged = (_, { label, value }) => {
        let paramsCopy = formState.votingParams;
        paramsCopy[label] = value;

        setFormState({ ...formState, votingParams: paramsCopy });
    };

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
