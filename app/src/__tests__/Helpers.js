import { ApiPromise, WsProvider } from '@polkadot/api';
import { bnToBn } from '@polkadot/util';

import config from '../config';
import { parseCall, coinVotingState, convictionVotingState } from '../helpers';

describe('helpers test suite', () => {
    describe('parseCall', () => {
        let wsProvider;
        let api;

        beforeAll(async () => {
            wsProvider = new WsProvider(config["PROVIDER_SOCKET"]);
            api = await ApiPromise.create({ provider: wsProvider, types: config.types });
        });

        it('system.remark', async () => {
            const knownSystemRemark = '0x00010400';
            expect(parseCall(api, knownSystemRemark)).toEqual('system.remark(0x00)');
        });

        it('tokens.transfer', async () => {
            const knownNativeTokensTransferToAlice = '0x060400d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0010a5d4e80000000000000000000000';
            expect(parseCall(api, knownNativeTokensTransferToAlice)).toEqual('tokens.transfer(Native,5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY,1000000000000)');
        });
    });

    describe('coinVotingState', () => {
        it('expired', () => {
            const expiredState = {
                parameters: {
                    ttl: bnToBn('10'),
                    min_quorum: 50,
                    min_participation: 33,
                },
                total_favorable: bnToBn('0'),
                total_against: bnToBn('0'),
                created_on: bnToBn('0')
            };
            const totalSupply = bnToBn('2000000000000000000');

            const [passing, expired] = coinVotingState(expiredState, totalSupply, 1000);

            expect(passing).toEqual(false);
            expect(expired).toEqual(true);
        });

        it('is passing', () => {
            const passingState = {
                parameters: {
                    ttl: bnToBn('10'),
                    min_quorum: 50,
                    min_participation: 33,
                },
                total_favorable: bnToBn('1000000000000000000'),
                total_against: bnToBn('0'),
                created_on: bnToBn('0')
            };
            const totalSupply = bnToBn('2000000000000000000');

            const [passing, expired] = coinVotingState(passingState, totalSupply, 0);

            expect(passing).toEqual(true);
            expect(expired).toEqual(false);
        });

        it('needs to wait', () => {
            const unCloseableState = {
                parameters: {
                    ttl: bnToBn('10'),
                    min_quorum: 50,
                    min_participation: 33,
                },
                total_favorable: bnToBn('0'),
                total_against: bnToBn('0'),
                created_on: bnToBn('0')
            };
            const totalSupply = bnToBn('2000000000000000000');

            const [passing, expired] = coinVotingState(unCloseableState, totalSupply, 0);

            expect(passing).toEqual(false);
            expect(expired).toEqual(false);
        });
    });

    describe('convictionVotingState', () => {
        it('is expired', () => {
            const expiredState = {
                parameters: {
                    ttl: bnToBn('20'),
                    min_quorum: 50,
                    min_participation: 33
                },
                created_on: bnToBn('0')
            };
            const totalSupply = bnToBn('2000000000000000000');

            const [passing, expired] = convictionVotingState(expiredState, totalSupply, 1000);

            expect(passing).toEqual(false);
            expect(expired).toEqual(true);
        });

        it('needs to wait', () => {
            const unCloseableState = {
                parameters: {
                    ttl: bnToBn('20'),
                    min_quorum: 50,
                    min_participation: 33
                },
                created_on: bnToBn('0')
            };
            const totalSupply = bnToBn('2000000000000000000');

            const [passing, expired] = convictionVotingState(unCloseableState, totalSupply, 0);

            expect(passing).toEqual(false);
            expect(expired).toEqual(false);
        });
    });
});