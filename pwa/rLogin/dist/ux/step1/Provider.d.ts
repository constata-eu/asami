import * as React from 'react';
interface IProviderProps {
    userProvider: {
        name: string;
        logo: string;
        description: string;
        onClick?: () => Promise<void>;
    };
    handleConnect: (provider: any) => void;
    hideIfDisabled: boolean;
}
export declare function Provider(props: IProviderProps): React.JSX.Element;
export {};
//# sourceMappingURL=Provider.d.ts.map