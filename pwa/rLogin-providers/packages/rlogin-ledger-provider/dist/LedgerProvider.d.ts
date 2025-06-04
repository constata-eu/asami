import { PersonalSignParams, SignParams, EthSendTransactionParams } from '@rsksmart/rlogin-eip1193-types';
import { RLoginEIP1193ProviderOptions, RLoginEIP1193Provider } from '@rsksmart/rlogin-eip1193-proxy-subprovider';
type LedgerProviderOptions = RLoginEIP1193ProviderOptions & {
    debug?: boolean;
    dPath?: string;
    messageHashed?: boolean;
};
export declare class LedgerProvider extends RLoginEIP1193Provider {
    #private;
    readonly isLedger = true;
    protected dpath: string;
    private appEthConnected;
    private appEth?;
    private debug;
    messageHashed: boolean;
    constructor({ chainId, rpcUrl, dPath, debug, messageHashed }: LedgerProviderOptions);
    connect(): Promise<RLoginEIP1193Provider>;
    chooseAccount(dpath: string): Promise<RLoginEIP1193Provider>;
    getAddresses(dPaths: string[]): Promise<{
        path: string;
        address: string;
    }[]>;
    ethSendTransaction(params: EthSendTransactionParams): Promise<string>;
    private validateConnectionAndPersonalSign;
    ethSign(params: SignParams): Promise<string>;
    personalSign(params: PersonalSignParams): Promise<string>;
    disconnect(): Promise<void>;
}
export {};
