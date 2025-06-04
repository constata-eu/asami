import { IProviderInfo } from 'web3modal';
/**
 * WalletConnect 2.0 Provider Modifications
 * To be compatiable with WC2.0  the following change is needed. RLogin is not up to date with web3modal
 * and as such, this patch is needed until we can update.
 * @param providerInfo IProviderInfo
 * @returns provider
 */
export declare const walletConnect2Provider: (providerInfo: IProviderInfo) => {
    connector: (ProviderPackage: any, options: any) => Promise<any>;
    package: {
        required: string[];
    };
    id: string;
    type: string;
    check: string;
    name: string;
    logo: string;
    description?: string | undefined;
};
//# sourceMappingURL=walletconnect2.d.ts.map