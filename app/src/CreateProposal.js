import React, { useEffect, useState } from 'react';
import { Grid, Form, Dropdown, Input, Label } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

const argIsOptional = (arg) =>
    arg.type.toString().startsWith('Option<');

function Main(props) {
    const { api, jsonrpc } = useSubstrate();
    const { accountPair, orgs } = props;

    const [status, setStatus] = useState(null);
    const [orgAddress, setOrgAddress] = useState(null);
    const [palletRPCs, setPalletRPCs] = useState([]);
    const [callables, setCallables] = useState([]);
    const [paramFields, setParamFields] = useState([]);
    const [formState, setFormState] = useState({
        palletRpc: '',
        callable: '',
        inputParams: []
    });
    const { palletRpc, callable, inputParams } = formState;

    const updatePalletRPCs = () => {
        if (!api) { return; }
        const apiType = api.tx;
        const palletRPCs = Object.keys(apiType).sort()
            .filter(pr => Object.keys(apiType[pr]).length > 0)
            .map(pr => ({ key: pr, value: pr, text: pr }));
        setPalletRPCs(palletRPCs);
    };

    const updateCallables = () => {
        if (!api || palletRpc === '') { return; }
        const callables = Object.keys(api.tx[palletRpc]).sort()
            .map(c => ({ key: c, value: c, text: c }));
        setCallables(callables);
    };

    const updateParamFields = () => {
        if (!api || palletRpc === '' || callable === '') {
            setParamFields([]);
            return;
        }

        let paramFields = [];

        const metaArgs = api.tx[palletRpc][callable].meta.args;

        if (metaArgs && metaArgs.length > 0) {
            paramFields = metaArgs.map(arg => ({
                name: arg.name.toString(),
                type: arg.type.toString(),
                optional: argIsOptional(arg)
            }));
        }

        setParamFields(paramFields);
    };

    useEffect(updatePalletRPCs, [api]);
    useEffect(updateCallables, [api, palletRpc]);
    useEffect(updateParamFields, [api, palletRpc, callable, jsonrpc]);

    const onPalletCallableParamChange = (_, data) => {
        setFormState(formState => {
            let res;
            const { state, value } = data;
            if (typeof state === 'object') {
                // Input parameter updated
                const { ind, paramField: { type } } = state;
                const inputParams = [...formState.inputParams];
                inputParams[ind] = { type, value };
                res = { ...formState, inputParams };
            } else if (state === 'palletRpc') {
                res = { ...formState, [state]: value, callable: '', inputParams: [] };
            } else if (state === 'callable') {
                res = { ...formState, [state]: value, inputParams: [] };
            }
            return res;
        });
    };

    // For some reason `orgs.map` is undefined
    const dropdownOptions = Object.keys(orgs).map((addr) => ({
        key: addr,
        value: addr,
        text: addr,
    }));

    return (
        <Grid.Column width={8}>
            <h1>Create a Proposal</h1>
            <Form>
                <Form.Field>
                    <Dropdown
                        placeholder='Organization Address'
                        search
                        selection
                        options={dropdownOptions}
                        onChange={(_, { value }) => setOrgAddress(value)}
                    />
                </Form.Field>
                <Form.Field>
                    <Dropdown
                        placeholder='Pallet'
                        fluid
                        label='Pallet'
                        onChange={onPalletCallableParamChange}
                        search
                        selection
                        state='palletRpc'
                        value={palletRpc}
                        options={palletRPCs}
                    />
                </Form.Field>
                <Form.Field>
                    <Dropdown
                        placeholder='Callables'
                        fluid
                        label='Callable'
                        onChange={onPalletCallableParamChange}
                        search
                        selection
                        state='callable'
                        value={callable}
                        options={callables}
                    />
                </Form.Field>
                {paramFields.map((paramField, ind) =>
                    <Form.Field key={`${paramField.name}-${paramField.type}`}>
                        <Input
                            placeholder={paramField.type}
                            fluid
                            type='text'
                            label={paramField.name}
                            state={{ ind, paramField }}
                            value={inputParams[ind] ? inputParams[ind].value : ''}
                            onChange={onPalletCallableParamChange}
                        />
                        {paramField.optional
                            ? <Label
                                basic
                                pointing
                                color='teal'
                                content='Leaving this field as blank will submit a NONE value'
                            />
                            : null
                        }
                    </Form.Field>
                )}
                <Form.Field style={{ textAlign: 'center' }}>
                    <TxButton
                        accountPair={accountPair}
                        label='Propose'
                        type='SIGNED-TX-FOR-ORG'
                        org={orgAddress}
                        setStatus={setStatus}
                        attrs={{
                            palletRpc, callable, inputParams, paramFields
                        }}
                    />
                </Form.Field>
                <div style={{ overflowWrap: 'break-word' }}>{status}</div>
            </Form>
        </Grid.Column>
    );
}

export default function CreateProposal(props) {
    const { api } = useSubstrate();
    return api.query.organizations && api.query.organizations.proposals ? <Main {...props} /> : null;
}
