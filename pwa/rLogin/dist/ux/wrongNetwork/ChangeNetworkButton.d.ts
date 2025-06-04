import React from 'react';
import { AddEthereumChainParameter } from './changeNetwork';
interface ChangeNetworkButtonInterface {
    params: AddEthereumChainParameter | undefined;
    changeNetwork: (params: AddEthereumChainParameter) => void;
}
declare const ChangeNetworkButton: React.FC<ChangeNetworkButtonInterface>;
export default ChangeNetworkButton;
//# sourceMappingURL=ChangeNetworkButton.d.ts.map