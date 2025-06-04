import { DCentProvider, DCentProviderOptions } from './DCentProvider';
export declare const dcentProviderOptions: {
    display: {
        logo: string;
        name: string;
        description: string;
    };
    connector: (ProviderPackage: any, options: DCentProviderOptions) => Promise<any>;
    package: typeof DCentProvider;
};
