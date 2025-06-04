import * as React from 'react';
import { SimpleFunction, IProviderUserOptions } from 'web3modal';
import { IIPFSCpinnerClient as IDataVault, IAuthManagerNewable, IWeb3ProviderEncryptionManager } from '@rsksmart/ipfs-cpinner-client-types';
import { SDR, SD } from './ux/step2';
import { AuthKeys } from './lib/did-auth';
import { AddEthereumChainParameter } from './ux/wrongNetwork/changeNetwork';
import { ThemeType, themesOptions } from './theme';
import { NetworkParams } from './lib/networkOptionsTypes';
import { InfoOptions } from './ux/confirmInformation/InfoOptions';
declare global {
    interface Window {
        ethereum: any;
        web3: any;
        showRLoginModal: (step?: Step) => void;
    }
}
export interface DataVaultPackage {
    default: IDataVault;
    AuthManager: IAuthManagerNewable;
    AsymmetricEncryptionManager: IWeb3ProviderEncryptionManager;
    SignerEncryptionManager: IWeb3ProviderEncryptionManager;
}
export interface DataVaultOptions {
    package: DataVaultPackage;
    serviceUrl: string;
}
interface IModalProps {
    userProviders: IProviderUserOptions[];
    onClose: SimpleFunction;
    providerController: any;
    onConnect: (provider: any, disconnect: () => void, selectedLanguage: string, selectedTheme: themesOptions, dataVault?: IDataVault, authKeys?: AuthKeys) => Promise<void>;
    onError: (error: any) => Promise<void>;
    onAccountsChange: (accounts: string[]) => void;
    onChainChange: (chainId: string | number) => void;
    onLanguageChanged: (language: string) => void;
    onThemeChanged: (theme: themesOptions) => void;
    backendUrl?: string;
    keepModalHidden?: boolean;
    supportedChains?: number[];
    supportedLanguages?: string[];
    dataVaultOptions?: DataVaultOptions;
    themes: {
        [K in themesOptions]: ThemeType;
    };
    defaultTheme: themesOptions;
    rpcUrls?: {
        [key: string]: string;
    };
    infoOptions: InfoOptions;
    afterDisconnect: () => void;
    ethereumChains?: Map<number, AddEthereumChainParameter>;
}
type Step = 'Step1' | 'Step2' | 'confirmInformation' | 'walletInfo' | 'error' | 'wrongNetwork' | 'changeNetwork' | 'chooseNetwork' | 'choosePath' | 'loading' | 'tutorial';
interface ErrorDetails {
    title: string;
    description?: string;
    footerCta?: React.ReactNode;
}
type NetworkConnectionConfig = {
    chainId: number;
    rpcUrl?: string;
    networkParams?: NetworkParams;
};
interface IModalState {
    show: boolean;
    currentStep: Step;
    lightboxOffset: number;
    provider?: any;
    sdr?: SDR;
    sd?: SD;
    challenge?: string;
    address?: string;
    chainId?: number;
    errorReason?: ErrorDetails;
    dataVault?: IDataVault;
    loadingReason?: string;
    currentTheme?: themesOptions;
    selectedProviderUserOption?: {
        provider: IProviderUserOptions;
        chosenNetwork?: NetworkConnectionConfig;
    };
    chosenNetwork?: NetworkConnectionConfig;
}
/**
 * IProviderUserOptions with added onClick variable
 */
export interface RLoginIProviderUserOptions extends IProviderUserOptions {
    name: string;
    logo: string;
    description: string;
    onClick: (optionalOpts?: {
        chainId: number;
        rpcUrl: string;
    }) => Promise<any>;
}
export declare class Core extends React.Component<IModalProps, IModalState> {
    constructor(props: IModalProps);
    state: IModalState;
    lightboxRef?: HTMLDivElement | null;
    mainModalCard?: HTMLDivElement | null;
    private availableLanguages;
    get selectedTheme(): themesOptions;
    get selectedLanguageCode(): string;
    componentDidUpdate(_prevProps: IModalProps, _prevState: IModalState): void;
    showModalWithStep(step: 'Step1' | 'changeNetwork' | 'walletInfo'): void;
    private setupLanguages;
    /** accounts related */
    private did;
    /** chain id related */
    private setChainId;
    /**
     * ContinueSettingUp
     * After connecting to the provider but before detecting the flavor
     */
    private continueSettingUp;
    private validateCurrentChain;
    private changeMetamaskNetwork;
    /**
     * Before connecting to the provider, go through a checklist of items
     * Check if the provider supports multiple networks and if the user
     * should choose a network first.
     * @param provider The provider selected by the user to use
     */
    private preConnectChecklist;
    private chooseNetwork;
    /**
     * Checklist before sending the connect method
     */
    private preTutorialChecklist;
    /** Pre-Step 1 - user picked a wallet, and network and waiting to connect */
    private connectToWallet;
    /** Step 1 Provider Answered
    * The provider has answered and is ready to go to the next step
    * or access the data vault.
    */
    private setupProvider;
    private detectFlavor;
    /** Step 2  */
    private fetchSelectiveDisclosureRequest;
    private onConfirmSelectiveDisclosure;
    /** Step 3 */
    private onConfirmAuth;
    /**
     * Helper function to see if the confirm information step should be shown or not.
     * If true, sets the currentStep and turns on the modal, if false, then continues
     * to ConfirmAuth which will detect the flavor.
     */
    private shouldShowConfirmStep;
    private setLightboxRef;
    /**
    * Disconnect from the provider
    */
    disconnect(): void;
    /**
     * Close Modal
     * Triggered when the user closes the modal without finishing the process
     */
    private closeModal;
    changeLanguage: (language: string) => void;
    changeTheme: (theme: themesOptions) => void;
    render: () => React.JSX.Element;
}
export {};
//# sourceMappingURL=Core.d.ts.map