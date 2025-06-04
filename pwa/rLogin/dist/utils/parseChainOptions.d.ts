import { AddEthereumChainParameter } from '../ux/wrongNetwork/changeNetwork';
import { InfoOptions } from '../ux/confirmInformation/InfoOptions';
export declare function parseSupportedChains(ethereumChains?: AddEthereumChainParameter[]): number[] | undefined;
export declare function parseRpcUrls(ethereumChains?: AddEthereumChainParameter[]): Record<string, string> | undefined;
export declare function parseInfoOptions(ethereumChains?: AddEthereumChainParameter[]): InfoOptions | undefined;
export declare function chainArrToMap(ethereumChains?: AddEthereumChainParameter[]): Map<number, AddEthereumChainParameter> | undefined;
//# sourceMappingURL=parseChainOptions.d.ts.map