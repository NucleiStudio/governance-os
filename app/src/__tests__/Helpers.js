import { ApiPromise, WsProvider } from '@polkadot/api';

import config from '../config';
import { parseCall } from '../helpers';

describe('helpers testing suite', async () => {
    let wsProvider;
    let api;

    beforeAll(async () => {
        wsProvider = new WsProvider(config["PROVIDER_SOCKET"]);
        api = await ApiPromise.create({ provider: wsProvider, types: config.types });
    });

    it('parseCall', async () => {
        const knownSystemRemark = '0x00010400';
        const knownNativeTokensTransferToAlice = '0x060400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0010a5d4e80000000000000000000000';

        expect(parseCall(api, knownSystemRemark)).toEqual('system.remark(0x00)');
        expect(parseCall(api, knownNativeTokensTransferToAlice)).toEqual('tokens.transfer(Native,5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY,1000000000000)');
    });
});