import Eth from 'ethjs-query';
import { IRLoginEIP1193Provider, EthSendTransactionParams, PersonalSignParams, SignParams } from '@rsksmart/rlogin-eip1193-types';
export type RLoginEIP1193ProviderOptions = {
    rpcUrl: string;
    chainId: number;
};
export declare abstract class RLoginEIP1193Provider implements IRLoginEIP1193Provider {
    protected selectedAddress?: string;
    readonly chainId: number;
    protected provider: typeof Eth;
    constructor({ rpcUrl, chainId }: RLoginEIP1193ProviderOptions);
    abstract ethSendTransaction(params: EthSendTransactionParams): Promise<string>;
    abstract personalSign(params: PersonalSignParams): Promise<string>;
    abstract ethSign(params: SignParams): Promise<string>;
    private validateSender;
    request({ method, params }: {
        method: any;
        params: any;
    }): Promise<any>;
    sendAsync(request: {
        method: string;
        params?: any;
    }, cb: any): void;
    on(method: string): void;
    removeAllListeners(): void;
}
