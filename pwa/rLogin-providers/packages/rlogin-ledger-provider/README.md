<p align="middle">
  <img src="https://user-images.githubusercontent.com/766679/236442723-004fc7a5-edb2-4477-86da-0b687d62702f.svg" alt="logo" height="100" >
</p>
<h3 align="middle"><code>@rsksmart/rlogin-ledger-provider</code></h3>
<p align="middle">
  rLogin Ledger Provider
</p>
<p align="middle">
  <a href="https://github.com/rsksmart/rlogin-ledger-connector/actions/workflows/ci.yml" alt="ci">
    <img src="https://github.com/rsksmart/rlogin-ledger-connector/actions/workflows/ci.yml/badge.svg" alt="ci" />
  </a>
  <a href="https://developers.rsk.co/rif/templates/">
    <img src="https://img.shields.io/badge/-docs-brightgreen" alt="docs" />
  </a>
  <a href="https://lgtm.com/projects/g/rsksmart/rlogin-ledger-connector/context:javascript">
    <img src="https://img.shields.io/lgtm/grade/javascript/github/rsksmart/rlogin-ledger-connector" />
  </a>
  <a href='https://coveralls.io/github/rsksmart/rlogin-ledger-connector?branch=main'>
    <img src='https://coveralls.io/repos/github/rsksmart/rlogin-ledger-connector/badge.svg?branch=main' alt='Coverage Status' />
  </a>
  <!--
  <a href="https://hits.seeyoufarm.com">
    <img src="https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Frsksmart%2Frlogin-ledger-connector&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false"/>
  </a>
    <a href="https://badge.fury.io/js/%40rsksmart%2Frlogin-ledger-connector">
      <img src="https://badge.fury.io/js/%40rsksmart%2Frlogin-ledger-connector.svg" alt="npm" />
    </a>
  -->
</p>

A Ledger provider connection for rLogin. Still in beta and tested manually for now.

## Features

Allow users to connect to your dapp using a Ledger device. Currently works with USB and returns an EIP1193 provider.

## Implementation

The implementation is a bit different for Ledger because it is not a Web3Modal supported provider. 

Add the dependecy to your project

```
yarn add @rsksmart/rlogin-ledger-provider --save
```

In your dapp, your rLogin implementation should be similar to this:

```
import RLogin from '@rsksmart/rlogin'
import { ledgerProviderOptions } from '@rsksmart/rlogin-ledger-provider'

// ...

const rLogin = new RLogin({
  cacheProvider: false,
  providerOptions: {
    // ... other providers, i.e. WalletConnect or Portis, etc
   'custom-ledger': {
      ...ledgerProviderOptions,
      options: {
        rpcUrl: 'https://public-node.testnet.rsk.co',
        chainId: 31
      }
    }
  },
  supportedChains: [30, 31]
})
```

### Implementation notes

- Similar to the Portis connector, you can only specify a single chainId to connect to.
- The `custom-` needs to be added because Ledger is not a Web3Modal supported provider. The `...ledgerProviderOptions` contains the Ledger's text and image and connects rLogin to the provider.
- Ledger has two apps that work with RSK:
  - The RSK App will only work with RSK Mainnet as it uses the correct derivation path of `44'/137'/0'/0/0`
  - To use RSK Testnet, you must use the Ethereum App on the Ledger. It will use the standard Ethereum derivation path of `44'/60'/0'/0/0`. As of writing, no Ledger app will accept the RSK Testnet derivation path.
- pass `debug: true` for console logs that may help you debug.

## Run for development

Install dependencies:

```
yarn i
```

### Run unit tests

```
yarn test
```

Coverage report with:

```
yarn run test:coverage
```

### Run linter

```
yarn run lint
```

Auto-fix:

```
yarn run lint:fix
```

### Build for production

```
yarn run build
```
