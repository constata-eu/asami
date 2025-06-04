import React from 'react';
import { AddEthereumChainParameter } from './changeNetwork';
interface WrongNetworkComponentInterface {
    supportedNetworks: number[] | undefined;
    chainId: number | undefined;
    isMetamask: boolean | null;
    isWrongNetwork: boolean;
    changeNetwork: (params: AddEthereumChainParameter) => void;
    ethereumChains?: Map<number, AddEthereumChainParameter>;
}
declare const WrongNetworkComponent: React.FC<WrongNetworkComponentInterface>;
export default WrongNetworkComponent;
//# sourceMappingURL=WrongNetworkComponent.d.ts.map