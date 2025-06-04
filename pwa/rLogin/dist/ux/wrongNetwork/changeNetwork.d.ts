export interface AddEthereumChainParameter {
    chainId: string;
    chainName: string;
    nativeCurrency: {
        name: string;
        symbol: string;
        decimals: 18;
    };
    rpcUrls: string[];
    blockExplorerUrls?: string[];
    iconUrls?: string[];
}
export declare const networks: Map<number, AddEthereumChainParameter>;
//# sourceMappingURL=changeNetwork.d.ts.map