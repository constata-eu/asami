import { AddEthereumChainParameter } from '../ux/wrongNetwork/changeNetwork';
interface RequestArguments {
    readonly method: string;
    readonly params?: readonly unknown[] | object;
}
export interface EIP1193Provider {
    request<T = string>(args: RequestArguments): Promise<T>;
    isMetaMask: boolean | null;
    isNiftyWallet: boolean | null;
}
export declare const ethAccounts: (provider: EIP1193Provider) => Promise<string[]>;
export declare const ethChainId: (provider: EIP1193Provider) => Promise<string>;
export declare const personalSign: (provider: EIP1193Provider, address: string, data: string) => Promise<string>;
export declare const addEthereumChain: (provider: EIP1193Provider, params: AddEthereumChainParameter) => Promise<string>;
export declare const isMetamask: (provider: EIP1193Provider) => boolean | null;
export {};
//# sourceMappingURL=provider.d.ts.map