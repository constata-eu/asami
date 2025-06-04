<p align="middle">
  <img src="https://user-images.githubusercontent.com/766679/236442723-004fc7a5-edb2-4477-86da-0b687d62702f.svg" alt="logo" height="100" >
</p>
<h3 align="middle"><code>@rsksmart/rlogin-walletconnect2-provider</code></h3>
<p align="middle">
  rLogin WalletConnect 2 Provider
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

A WalletConnect 2 provider connection for rLogin. Still in beta and tested manually for now.

## Features

Allow you to connect to WalletConnect 2 by using rLogin

## Implementation

Add the dependecy to your project

```
yarn add @rsksmart/rlogin-walletconnect2-provider
```

In your dapp, your rLogin implementation should be similar to this:

```ts
import RLogin from '@rsksmart/rlogin'
import { WalletConnect2Provider } from '@rsksmart/rlogin-walletconnect2-provider'

const rLogin = new RLogin({
  cacheProvider: false,
  providerOptions: {
    walletconnect: {
      package: WalletConnect2Provider,
      options: {
        projectId: 'PROJECTID',
        chains: [30, 31],
        showQrModal: true,
        //methods, // OPTIONAL ethereum methods
        //events, // OPTIONAL ethereum events
        rpcMap: rpcUrls, // OPTIONAL rpc urls for each chain
        //metadata, // OPTIONAL metadata of your app
        //qrModalOptions, // OPTIONAL - `undefined` by default
      }
    },
  },
  supportedChains: [30, 31]
})
```

### Implementation notes

## Run for development

Install dependencies:

```
yarn
```

### Run unit tests

TBD

### Run linter

TBD

Auto-fix:

```
yarn run lint:fix
```

### Build for production

```
yarn run build
```
