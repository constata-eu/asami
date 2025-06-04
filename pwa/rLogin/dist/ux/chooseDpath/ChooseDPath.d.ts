import React from 'react';
export interface AccountInterface {
    index: number;
    dPath: string;
    address: string;
    balance?: string;
}
export declare const Column: import("styled-components").StyledComponent<"div", import("styled-components").DefaultTheme, {
    width?: number | undefined;
}, never>;
interface Interface {
    provider: any;
    selectPath: (address: string) => void;
    handleError: (error: any) => void;
}
export declare const ChooseDPathComponent: React.FC<Interface>;
export {};
//# sourceMappingURL=ChooseDPath.d.ts.map