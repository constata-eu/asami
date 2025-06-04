import React from 'react';
import { SD } from '../../lib/sdr';
import { IProviderUserOptions } from 'web3modal';
import { InfoOptions } from './InfoOptions';
import { AddEthereumChainParameter } from '../wrongNetwork/changeNetwork';
interface ConfirmInformationProps {
    chainId: number | undefined;
    address: string | undefined;
    displayMode: boolean;
    sd: SD | undefined;
    providerUserOption: IProviderUserOptions;
    provider: any;
    onConfirm: () => Promise<any>;
    onCancel: () => void;
    infoOptions: InfoOptions;
    disconnect: () => void;
    ethereumChains?: Map<number, AddEthereumChainParameter>;
}
export declare function ConfirmInformation({ displayMode, chainId, address, providerUserOption, sd, provider, onConfirm, onCancel, infoOptions, disconnect, ethereumChains }: ConfirmInformationProps): React.JSX.Element;
export declare function shortAddress(address?: string): string;
export {};
//# sourceMappingURL=ConfirmInformation.d.ts.map