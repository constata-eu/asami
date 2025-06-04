import { RLoginEIP1193Provider, RLoginEIP1193ProviderOptions } from '@rsksmart/rlogin-eip1193-proxy-subprovider';
import { EthSendTransactionParams, SignParams, PersonalSignParams } from '@rsksmart/rlogin-eip1193-types';
type TrezorOptions = {
    manifestEmail: string;
    manifestAppUrl: string;
};
export type TrezorProviderOptions = RLoginEIP1193ProviderOptions & TrezorOptions & {
    debug?: boolean;
    dPath?: string;
};
export declare class TrezorProvider extends RLoginEIP1193Provider {
    #private;
    readonly isTrezor = true;
    path: string;
    opts: TrezorOptions;
    initialized: boolean;
    connected: boolean;
    debug: boolean;
    constructor({ rpcUrl, chainId, debug, dPath, manifestEmail, manifestAppUrl }: TrezorProviderOptions);
    connect(): Promise<any>;
    chooseAccount(dpath: string): Promise<RLoginEIP1193Provider>;
    getAddresses(dPaths: string[]): Promise<{
        path: string;
        address: string;
    }[]>;
    private validateConnectionAndSign;
    personalSign(params: PersonalSignParams): Promise<string>;
    ethSign(params: SignParams): Promise<string>;
    ethSendTransaction(params: EthSendTransactionParams): Promise<string>;
}
export {};
