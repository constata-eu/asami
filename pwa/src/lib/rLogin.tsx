import RLogin from '@rsksmart/rlogin'
import { trezorProviderOptions } from '@rsksmart/rlogin-trezor-provider'
import { ledgerProviderOptions } from '@rsksmart/rlogin-ledger-provider'
import { dcentProviderOptions } from '@rsksmart/rlogin-dcent-provider'
import { WalletConnect2Provider } from '@rsksmart/rlogin-walletconnect2-provider'
import Portis from '@portis/web3'
import { Settings } from "../settings";

export const rLogin = new RLogin({
  ethereumChains: Settings.rsk.supportedChains,
  providerOptions: {
    walletconnect: {
      package: WalletConnect2Provider,
      options: {
        projectId: 'PROJECTID',
        chains: [Settings.rsk.chainId],
        showQrModal: true,
        rpcMap: Settings.rsk.rpcUrls
      }
    },
    portis: {
      package: Portis,
      options: {
        id: "a1c8672b-7b1c-476b-b3d0-41c27d575920",
        network: {
          nodeUrl: Settings.rsk.supportedChains[0].rpcUrls[0],
          chainId: Settings.rsk.chainId,
        }
      }
    },
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
})
