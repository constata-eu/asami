import React from 'react';
import { IProviderUserOptions } from 'web3modal';
import { themesOptions } from '../../theme';
interface IWalletProvidersProps {
    userProviders: IProviderUserOptions[];
    connectToWallet: (provider: IProviderUserOptions) => void;
    changeLanguage: (event: any) => void;
    changeTheme: (theme: themesOptions) => void;
    availableLanguages: {
        code: string;
        name: string;
    }[];
    selectedLanguageCode: string;
    selectedTheme: themesOptions;
}
export declare const userProvidersByName: (userProviders: IProviderUserOptions[]) => {
    [name: string]: IProviderUserOptions;
};
export declare const WalletProviders: ({ userProviders, connectToWallet, changeLanguage, changeTheme, availableLanguages, selectedLanguageCode, selectedTheme }: IWalletProvidersProps) => React.JSX.Element;
export {};
//# sourceMappingURL=WalletProviders.d.ts.map