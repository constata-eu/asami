import { IProviderDisplayWithConnector, IProviderControllerOptions } from 'web3modal';
import { RLoginIProviderUserOptions } from '../Core';
export declare class RLoginProviderController {
    cachedProvider: string;
    shouldCacheProvider: boolean;
    disableInjectedProvider: boolean;
    private eventController;
    private injectedProvider;
    private providers;
    private providerOptions;
    private network;
    constructor(opts: IProviderControllerOptions);
    shouldDisplayProvider(id: string): boolean;
    getUserOptions: () => RLoginIProviderUserOptions[];
    getProvider(id: string): IProviderDisplayWithConnector | undefined;
    getProviderOption(id: string, key: 'package' | 'options'): any;
    clearCachedProvider(): void;
    setCachedProvider(id: string): void;
    connectTo: (id: string, connector: (providerPackage: any, opts: any) => Promise<any>, optionalOpts?: {
        chainId?: number;
        rpcUrl?: string;
        dPath?: string;
        networkParams?: any;
    }) => Promise<unknown>;
    connectToCachedProvider(): Promise<unknown>;
    on(event: string, callback: (result: any) => void): () => void;
    off(event: string, callback?: (result: any) => void): void;
}
//# sourceMappingURL=providers.d.ts.map