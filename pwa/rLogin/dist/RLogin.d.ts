import { IProviderControllerOptions, SimpleFunction } from 'web3modal';
import { DataVaultOptions } from './Core';
import { themesOptions } from './theme';
import { InfoOptions } from './ux/confirmInformation/InfoOptions';
import { AddEthereumChainParameter } from './ux/wrongNetwork/changeNetwork';
interface RLoginOptions {
    backendUrl?: string;
    keepModalHidden?: boolean;
    supportedChains?: number[];
    supportedLanguages?: string[];
    dataVaultOptions?: DataVaultOptions;
    customThemes?: any;
    defaultTheme?: themesOptions;
    rpcUrls?: {
        [key: string]: string;
    };
    infoOptions?: InfoOptions;
    ethereumChains?: AddEthereumChainParameter[];
}
type Options = Partial<IProviderControllerOptions> & RLoginOptions;
export declare class RLogin {
    private eventController;
    private rLoginStorage;
    private providerController;
    private userProviders;
    private supportedChains?;
    private supportedLanguages?;
    private backendUrl?;
    private keepModalHidden;
    private dataVaultOptions?;
    private themes;
    private defaultTheme;
    private rpcUrls?;
    private infoOptions;
    private ethereumChains?;
    private coreRef;
    constructor(opts?: Options);
    get cachedProvider(): string;
    private showModal;
    showWalletInfo: () => void | undefined;
    showChangeNetwork: () => void | undefined;
    /** handles an event */
    private handleOnAndTrigger;
    /** event handlers */
    private onClose;
    private onConnect;
    private onError;
    private onAccountsChange;
    private onChainChange;
    private onThemeChanged;
    private onLanguageChanged;
    private setupHandlers;
    /** renders the modal in DOM */
    private renderModal;
    /**
     * Connect to rLogin. This will prompt the modal based on the
     * definitions.
     */
    connect: () => Promise<any>;
    /**
     * Connect to rLogin with a specific wallet provider
     * @param id provider id (same of configuration)
     */
    connectTo: (id: string) => Promise<any>;
    /**
     * Subscribe to modal event
     * @param event event name
     * @param callback event callback closure
     */
    on(event: string, callback: SimpleFunction): SimpleFunction;
    /**
     * Unsubscribe from modal event
     * @param event event name
     * @param callback event callback closure
     */
    off(event: string, callback?: SimpleFunction): void;
    /**
     * Clear cached provider from local storage
     */
    clearCachedProvider(): void;
    /**
     * Set cached provider in local storage
     * @param id provider id (same of configuration)
     */
    setCachedProvider(id: string): void;
}
export {};
//# sourceMappingURL=RLogin.d.ts.map