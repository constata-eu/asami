import type { NetworkParamsAllOptions } from './networkOptionsTypes';
export declare const LEDGER_NAME = "Ledger";
export declare const TREZOR_NAME = "Trezor";
export declare const DCENT_NAME = "D'Cent";
export declare const TORUS_NAME = "Torus";
export declare const PORTIS_NAME = "Portis";
export declare function isHardwareWalletProvider(providerName: string): boolean;
export declare function getTutorialLocalStorageKey(providerName: string): "RLogin:DontShowTutorialAgain:Ledger" | "RLogin:DontShowTutorialAgain:Trezor" | "RLogin:DontShowTutorialAgain:DCent";
export declare function isLedger(providerName: string): boolean;
export declare function isTrezor(providerName: string): boolean;
export declare function isDCent(providerName: string): boolean;
export declare function isTorus(providerName: string): boolean;
export declare function isPortis(providerName: string): boolean;
export declare function requiresNetworkSelection(providerName: string): boolean;
export declare function requiresAccountSelection(provider: any): any;
export declare const PROVIDERS_NETWORK_PARAMS: {
    [key: string]: NetworkParamsAllOptions;
};
//# sourceMappingURL=hardware-wallets.d.ts.map