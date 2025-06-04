import { TrezorProvider, TrezorProviderOptions } from './TrezorProvider';
export declare const trezorProviderOptions: {
    display: {
        logo: string;
        name: string;
        description: string;
    };
    connector: (ProviderPackage: any, options: TrezorProviderOptions) => Promise<any>;
    package: typeof TrezorProvider;
};
