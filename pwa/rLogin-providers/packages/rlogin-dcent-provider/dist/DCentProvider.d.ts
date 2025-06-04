import DCentRPCProvider from '@rsksmart/dcent-provider';
import { RLoginEIP1193Provider, RLoginEIP1193ProviderOptions } from '@rsksmart/rlogin-eip1193-proxy-subprovider';
import { EthSendTransactionParams, PersonalSignParams } from '@rsksmart/rlogin-eip1193-types';
export type DCentProviderOptions = RLoginEIP1193ProviderOptions & {
    dPath?: string;
    debug?: boolean;
};
export declare class DCentProvider extends RLoginEIP1193Provider {
    #private;
    readonly isDcent = true;
    path: string;
    rpcUrl: string;
    appEthInitialized: boolean;
    appEthConnected: boolean;
    enabled: boolean;
    dcentProvider: typeof DCentRPCProvider;
    constructor({ rpcUrl, chainId, dPath, debug }: DCentProviderOptions);
    connect(): Promise<any>;
    personalSign(params: PersonalSignParams): Promise<string>;
    ethSign(params: PersonalSignParams): Promise<string>;
    ethSendTransaction(params: EthSendTransactionParams): Promise<string>;
}
