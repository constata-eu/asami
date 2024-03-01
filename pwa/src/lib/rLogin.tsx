import RLogin from '@rsksmart/rlogin'
import { WalletConnect2Provider } from '@rsksmart/rlogin-walletconnect2-provider'
/*
import Portis from '@portis/web3'
import Torus from '@toruslabs/torus-embed'
*/
import { trezorProviderOptions } from '@rsksmart/rlogin-trezor-provider'
import { ledgerProviderOptions } from '@rsksmart/rlogin-ledger-provider'
import { dcentProviderOptions } from '@rsksmart/rlogin-dcent-provider'
import { Settings } from "../settings";

const supportedChains = [
  {
    chainId: '0x1e', // hex 30
    chainName: 'RSK Mainnet',
    nativeCurrency: {
      name: 'RSK BTC',
      symbol: 'RBTC',
      decimals: 18
    },
    rpcUrls: ['https://public-node.rsk.co'],
    blockExplorerUrls: ['https://explorer.rsk.co'],
    iconUrls: ['https://developers.rsk.co/assets/img/favicons/android-chrome-192x192.png']
  },
  {
    chainId: '0x1f', // hex 31
    chainName: 'RSK Testnet',
    nativeCurrency: {
      name: 'TEST RSK BTC',
      symbol: 'tRBTC',
      decimals: 18
    },
    rpcUrls: ['https://public-node.testnet.rsk.co'],
    blockExplorerUrls: ['https://explorer.testnet.rsk.co'],
    iconUrls: ['https://developers.rsk.co/assets/img/favicons/android-chrome-192x192.png']
  },
  {
    chainId: '0x539', // hex 1337
    chainName: 'RSK local',
    nativeCurrency: {
      name: 'Local RSK BTC',
      symbol: 'lRBTC',
      decimals: 18
    },
    rpcUrls: ['http://localhost:8545'],
    blockExplorerUrls: [],
    iconUrls: ['https://developers.rsk.co/assets/img/favicons/android-chrome-192x192.png']
  }
];

//const supportedChains = Object.keys(rpcUrls).map(Number)

export const rLogin = new RLogin({
  providerOptions: {
    walletconnect: {
      package: WalletConnect2Provider,
      options: {
        rpc: Settings.rsk.rpcUrls
      }
    },
    /*
    portis: {
      package: Portis,
      options: {
        id: "a1c8672b-7b1c-476b-b3d0-41c27d575920",
        network: {
          nodeUrl: 'https://public-node.rsk.co',
          chainId: 31,
        }
      }
    },
    torus: {
        package: Torus,
    },
    */
    'custom-ledger': ledgerProviderOptions,
    'custom-dcent': dcentProviderOptions,
    'custom-trezor': {
      ...trezorProviderOptions,
      options: {
        manifestEmail: 'info@iovlabs.org',
        manifestAppUrl: 'https://basic-sample.rlogin.identity.rifos.org/',
      }
    }
  },
  ethereumChains: Settings.rsk.supportedChains,
})
