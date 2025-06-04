import { IIPFSCpinnerClient as DataVault } from '@rsksmart/ipfs-cpinner-client-types';
export interface SDR {
    credentials: string[];
    claims: string[];
}
export type DataField = {
    [key: string]: string[];
};
export type Data = {
    credentials: DataField;
    claims: DataField;
};
export type SelectiveDisclosureField = {
    [key: string]: string;
};
export interface SD {
    credentials: SelectiveDisclosureField;
    claims: SelectiveDisclosureField;
}
export declare const fetchSelectiveDisclosureRequest: (sdr: SDR, dataVault: DataVault, did: string) => Promise<Data>;
//# sourceMappingURL=sdr.d.ts.map