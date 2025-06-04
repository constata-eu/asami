import AppEth from '@ledgerhq/hw-app-eth'
import { CompleteTx } from '@rsksmart/rlogin-transactions'
import { ethers } from 'ethers'

/**
 * Sign a transaction using the Ledger
 * @param transactionData
 * @param appEth
 * @param path
 * @param chainId
 * @returns serialized Transaction
 */
export const signTransaction = (
  transactionData: CompleteTx, appEth: AppEth, path: string, chainId: number
) => {
  // (nonce, gasprice, startgas, to, value, data, chainid, 0, 0)
  const { from: address, ...txPlainData } = transactionData
  const txData: ethers.utils.UnsignedTransaction = {
    ...txPlainData,
    chainId
  }
  const serializedEthersTx = ethers.utils.serializeTransaction(txData).slice(2)
  return appEth.signTransaction(path, serializedEthersTx)
    .then((sig: {r: string, v: string, s: string}) => {
      const signature = {
        v: parseInt('0x' + sig.v),
        r: '0x' + sig.r,
        s: '0x' + sig.s,
        from: address
      }
      // Serialize the same transaction as before, but adding the signature on it
      return ethers.utils.serializeTransaction(txData, signature).slice(2)
    })
    .catch((err: Error) => {
      console.error('Error signing transaction:', err)
      throw err
    })
}

// converts hex string '0x...' to utf string
export function convertFromHex (hex:string) {
  let str:string = ''
  for (let i = 2; i < hex.length; i += 2) { str += String.fromCharCode(parseInt(hex.substr(i, 2), 16)) }
  return str
}
