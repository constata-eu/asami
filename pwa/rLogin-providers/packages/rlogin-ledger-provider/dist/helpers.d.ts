import AppEth from '@ledgerhq/hw-app-eth';
import { CompleteTx } from '@rsksmart/rlogin-transactions';
export declare const signTransaction: (transactionData: CompleteTx, appEth: AppEth, path: string, chainId: number) => Promise<string>;
export declare function convertFromHex(hex: string): string;
