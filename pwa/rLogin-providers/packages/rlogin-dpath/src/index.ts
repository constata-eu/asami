/**
 * Get the BIP-44 account derivation path for a given network and index. The network
 * is the return value for RPC eth_chainId
 * Standards:
 * - BIP-44: https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki
 * - SLIP-44: https://github.com/satoshilabs/slips/blob/master/slip-0044.md
 * - RSKIP-57: https://github.com/rsksmart/RSKIPs/blob/master/IPs/RSKIP57.md
 * - EIP-155: https://github.com/ethereum/EIPs/blob/master/EIPS/eip-155.md
 * - EIP-695: https://github.com/ethereum/EIPs/blob/master/EIPS/eip-695.md
 * @param chainId EIP-155 chain Id
 * @param index derive different accounts based on SLIP-44
 * @param isLedger use it for RSK Testnet and Ledger
 * @returns the first account of standard BIP-44 derviation path forthe given network
 */

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export const getDPathByChainId = (chainId: number, index: number = 0, _?: boolean): string => {
  // if (isLedger && chainId === 31) return `m/44'/1'/0'/0/${index}` // Ledger + RSK Testnet - based on slip-44 - 37310 does not work - user can set standard derivation path using RSK Testnet app
  switch (chainId) {
    case 30: return `m/44'/137'/0'/0/${index}` // RSK Mainnet - based on rskip-57
    case 31: return `m/44'/37310'/0'/0/${index}` // RSK Testnet - based on rskip-57
    case 1: return `m/44'/60'/0'/0/${index}` // Ethereum Mainnet - based on eip-155 and slip-44
    case 3:// Ropsten
    case 4:// Rinkeby
    case 5:// Goerli
    case 42: // Kovan
      return `m/44'/1'/0'/0/${index}` // Ethereum testnets - based on eip-155 and slip-44
    default: throw new Error('Network not supported please specify the derivation path')
  }
}
