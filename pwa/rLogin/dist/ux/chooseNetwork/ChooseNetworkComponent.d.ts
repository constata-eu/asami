import React from 'react';
import { NetworkParams, NetworkParamsAllOptions } from '../../lib/networkOptionsTypes';
import { AddEthereumChainParameter } from '../wrongNetwork/changeNetwork';
interface Interface {
    rpcUrls?: {
        [key: string]: string;
    };
    networkParamsOptions?: NetworkParamsAllOptions;
    providerName?: string;
    chooseNetwork: (network: {
        chainId: number;
        rpcUrl?: string;
        networkParams?: NetworkParams;
        dPath?: string;
    }) => void;
    ethereumChains?: Map<number, AddEthereumChainParameter>;
}
declare const ChooseNetworkComponent: React.FC<Interface>;
export default ChooseNetworkComponent;
//# sourceMappingURL=ChooseNetworkComponent.d.ts.map