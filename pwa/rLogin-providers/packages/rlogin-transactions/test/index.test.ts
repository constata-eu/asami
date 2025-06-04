import { Transaction } from '@rsksmart/rlogin-eip1193-types'
import { createTransaction } from '../src/index'
import BN from 'bn.js'

const from = '0x987654321'
const to = '0x123456789'

const basicTx: Transaction = { from, to }

const mockTransactionCount = 5
const mockGasPrice = 10000
const mockEstimateGas = 35000

const mockProvider = {
  getTransactionCount: () => Promise.resolve(new BN(mockTransactionCount)),
  gasPrice: () => Promise.resolve(new BN(mockGasPrice)),
  estimateGas: () => Promise.resolve(mockEstimateGas)
}

describe('createTransaction', () => {
  test('transaction with estimations', async () => {
    const result = await createTransaction(mockProvider, basicTx.from, basicTx)

    const expectedEstimatedTx: Transaction = {
      ...basicTx,
      value: '0x0',
      data: '0x',
      nonce: mockTransactionCount,
      gasPrice: mockGasPrice * 1.01,
      gasLimit: mockEstimateGas
    }

    expect(result).toMatchObject(expectedEstimatedTx)
  })

  test('complete transaction', async () => {
    const completeTx: Transaction = {
      ...basicTx,
      value: '0x1010',
      data: '0xabcd',
      nonce: 10,
      gasPrice: 1000,
      gasLimit: 2000
    }

    const result = await createTransaction(mockProvider, completeTx.from, completeTx)

    expect(result).toMatchObject(completeTx)
  })

  describe('value to hex string', () => {
    test('already hex', async () => {
      const tx = {
        ...basicTx,
        value: '0xabcd'
      }

      const result = await createTransaction(mockProvider, from, tx)

      expect(result.value).toEqual('0xabcd')
    })

    test('base 10 string to hex string', async () => {
      const tx = {
        ...basicTx,
        value: '10000'
      }

      const result = await createTransaction(mockProvider, from, tx)

      expect(result.value).toEqual('0x2710')
    })

    test('number to hex string', async () => {
      const tx = {
        ...basicTx,
        value: 10000
      }

      const result = await createTransaction(mockProvider, from, tx)

      expect(result.value).toEqual('0x2710')
    })
  })

  describe('estimate gas', () => {
    const customGasLimit = 12000

    test('can pass gas property', async () => {
      const tx = {
        ...basicTx,
        gas: customGasLimit
      }

      const result = await createTransaction(mockProvider, from, tx)

      expect(result.gasLimit).toEqual(customGasLimit)
      expect((result as any).gas).toBeUndefined()
    })

    test('can pass gasLimit property', async () => {
      const tx = {
        ...basicTx,
        gasLimit: customGasLimit
      }

      const result = await createTransaction(mockProvider, from, tx)

      expect(result.gasLimit).toEqual(customGasLimit)
      expect((result as any).gas).toBeUndefined()
    })
  })
})
