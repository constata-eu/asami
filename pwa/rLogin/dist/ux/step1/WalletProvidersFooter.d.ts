import React from 'react';
import { themesOptions } from '../../theme';
interface Interface {
    changeLanguage: (event: any) => void;
    changeTheme: (theme: themesOptions) => void;
    availableLanguages: {
        code: string;
        name: string;
    }[];
    selectedLanguageCode: string;
    selectedTheme: themesOptions;
}
declare const WalletProvidersFooter: React.FC<Interface>;
export default WalletProvidersFooter;
//# sourceMappingURL=WalletProvidersFooter.d.ts.map