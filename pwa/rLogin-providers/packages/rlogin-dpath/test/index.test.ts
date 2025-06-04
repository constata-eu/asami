import { getDPathByChainId } from '../src/index'

const testCases = [
  { title: 'rsk mainnet', chainId: 30, 0: 'm/44\'/137\'/0\'/0/0', 2: 'm/44\'/137\'/0\'/0/2' },
  { title: 'rsk testnet', chainId: 31, 0: 'm/44\'/37310\'/0\'/0/0', 2: 'm/44\'/37310\'/0\'/0/2' },
  { title: 'ethereum', chainId: 1, 0: 'm/44\'/60\'/0\'/0/0', 2: 'm/44\'/60\'/0\'/0/2' },
  ...[
    { title: 'ropsten', chainId: 3 },
    { title: 'rinkeby', chainId: 4 },
    { title: 'goreli', chainId: 5 },
    { title: 'kovan', chainId: 42 }
  ].map(({ title, chainId }) => ({
    title, chainId, 0: 'm/44\'/1\'/0\'/0/0', 2: 'm/44\'/1\'/0\'/0/2'
  }))
]
describe('dpath for network id', () => {
  for (const testCase of testCases) {
    test(testCase.title, () => {
      expect(getDPathByChainId(testCase.chainId)).toEqual(testCase['0'])
      expect(getDPathByChainId(testCase.chainId, 2)).toEqual(testCase['2'])
      expect(getDPathByChainId(testCase.chainId, 0)).toEqual(testCase['0'])
    })
  }

  test('throws for other coins', () => {
    expect(() => getDPathByChainId(200)).toThrow()
    expect(() => getDPathByChainId(200, 2)).toThrow()
    expect(() => getDPathByChainId(200, 2)).toThrow()
  })
})
