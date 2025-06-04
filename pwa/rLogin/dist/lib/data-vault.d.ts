import { EIP1193Provider } from './provider';
import { IIPFSCpinnerClient as DataVault } from '@rsksmart/ipfs-cpinner-client-types';
import { DataVaultOptions } from '../Core';
export declare const createDataVault: (dataVaultOptions: DataVaultOptions, provider: EIP1193Provider, did: string, address: string) => Promise<DataVault>;
export declare const getContentsFromDataVault: (dataVault: DataVault, did: string, key: string) => Promise<string[]>;
//# sourceMappingURL=data-vault.d.ts.map