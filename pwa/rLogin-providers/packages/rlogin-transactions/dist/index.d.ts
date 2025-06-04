import { Transaction } from '@rsksmart/rlogin-eip1193-types';
export type CompleteTx = {
    from: string;
    to: string;
    nonce: number;
    data: string;
    value: string | number;
    gasLimit: number;
    gasPrice: number;
};
type ClientTxOptions = Partial<Transaction & {
    gas: number;
}>;
export declare const createTransaction: (provider: any, from: string, tx: ClientTxOptions) => Promise<CompleteTx>;
export {};
