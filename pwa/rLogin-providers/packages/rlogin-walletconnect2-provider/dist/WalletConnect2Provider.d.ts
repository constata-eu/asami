import { EthereumProvider } from '@walletconnect/ethereum-provider';
import { EthereumProviderOptions } from '@walletconnect/ethereum-provider/dist/types/EthereumProvider';
export declare class WalletConnect2Provider extends EthereumProvider {
    options: EthereumProviderOptions;
    constructor(opts: EthereumProviderOptions);
    connect(opts: any): Promise<void>;
    sendAsync(args: any, callback: any): void;
}
