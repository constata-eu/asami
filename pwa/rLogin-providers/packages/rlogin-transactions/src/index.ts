import { Transaction } from '@rsksmart/rlogin-eip1193-types'
import BN from 'bn.js'

export type CompleteTx = {
  from: string
  to: string
  nonce: number
  data: string
  value: string | number
  gasLimit: number
  gasPrice: number
}

type ClientTxOptions = Partial<Transaction & { gas: number }>

const parseOrEstimateGas = async (provider: any, tx: ClientTxOptions, finalTx: Partial<CompleteTx>) => {
  if (tx.gasLimit) {
    return tx.gasLimit
  } else if (tx.gas) {
    return tx.gas
  } else {
    return await provider.estimateGas(finalTx)
  }
}

const toNumber = (v: number | any) => typeof v === 'number' ? v : Number(v)

const parseValue = (value: Transaction['value'] | undefined) => {
  if (!value) return '0x0'
  if (typeof value === 'string' && value.slice(0, 2) === '0x') return value
  return '0x' + new BN(value).toString(16)
}

const getNonce = (provider: any, account: string) => provider.getTransactionCount(account, 'pending').then(r => r.toNumber())
const getGasPrice = (provider: any) => provider.gasPrice().then(r => Math.floor(r.toNumber() * 1.01))
/**
 * This function parses the gas limit transaction option received from a web3 client or
 * calculates it if not set. Will use `gasLimit` as final tx option return type to support
 * Ledger, Trezor and D'Cent devices with any web3 client.
 *
 * Gas limit option can be set in any `gas` or `gasLimit` option depending on web3 client
 * choice. Refs:
 * - ethers: https://github.com/ethers-io/ethers.js/blob/master/packages/providers/src.ts/json-rpc-provider.ts#L180-L184
 * - web3.js: https://github.com/ChainSafe/web3.js/blob/977f0f9e67af431b214163dbff084a2fa0673564/packages/web3-core-helpers/src/formatters.js#L157-L159
 *
 * We return only `gasLimit` option, since we3 clients support accepting it. Based on:
 * - WalletConnect: https://github.com/WalletConnect/walletconnect-monorepo/blob/468148cb70998a9512ffa35770d95a8f80e407bd/packages/ethereum-provider/test/shared/WalletClient.ts#L105-L110
 * - Metamask: https://github.com/MetaMask/metamask-extension/blob/9e2935dd55f51053657fe0bbf1e816591fc834ec/app/scripts/controllers/transactions/index.js#L910-L915
 * @param provider http provider
 * @param from address of the to account
 * @param tx { from: string, to: string, value?: number, data?: hex, nonce?: number, gasPrice?: number, (gas | gasLimit): number }
 * @returns Transaction
 */
export const createTransaction = async (provider: any, from: string, tx: ClientTxOptions): Promise<CompleteTx> => {
  const finalTx: Partial<CompleteTx> = {
    from: from.toLowerCase(),
    to: tx.to.toLowerCase(),
    value: parseValue(tx.value),
    data: tx.data || '0x',
    nonce: tx.nonce || await getNonce(provider, tx.from),
    gasPrice: tx.gasPrice || await getGasPrice(provider)
  }

  finalTx.gasLimit = await parseOrEstimateGas(provider, tx, finalTx).then(toNumber)

  return finalTx as CompleteTx
}
