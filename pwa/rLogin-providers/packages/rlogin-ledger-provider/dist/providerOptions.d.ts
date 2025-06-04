import { LedgerProvider } from './LedgerProvider';
export declare const ledgerProviderOptions: {
    display: {
        logo: string;
        name: string;
        description: string;
    };
    connector: (ProviderPackage: any, options: any) => Promise<any>;
    package: typeof LedgerProvider;
};
