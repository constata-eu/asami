<p align="middle">
  <img src="https://user-images.githubusercontent.com/766679/236442723-004fc7a5-edb2-4477-86da-0b687d62702f.svg" alt="logo" height="100" >
</p>
<h3 align="middle"><code>@rsksmart/rlogin-dcent-provider</code></h3>
<p align="middle">
  rLogin D'Cent Provider
</p>
<p align="middle">
  <a href="https://github.com/rsksmart/rlogin-dcent-connector/actions/workflows/ci.yml" alt="ci">
    <img src="https://github.com/rsksmart/rlogin-dcent-connector/actions/workflows/ci.yml/badge.svg" alt="ci" />
  </a>
  <a href="https://developers.rsk.co/rif/templates/">
    <img src="https://img.shields.io/badge/-docs-brightgreen" alt="docs" />
  </a>
  <a href="https://lgtm.com/projects/g/rsksmart/rlogin-dcent-connector/context:javascript">
    <img src="https://img.shields.io/lgtm/grade/javascript/github/rsksmart/rlogin-dcent-connector" />
  </a>
  <a href='https://coveralls.io/github/rsksmart/rlogin-dcent-connector?branch=main'>
    <img src='https://coveralls.io/repos/github/rsksmart/rlogin-dcent-connector/badge.svg?branch=main' alt='Coverage Status' />
  </a>
</p>

A D'Cent provider connection for rLogin. Still in beta and tested manually for now.

## Features

Allow users to connect to your dapp using a D'cent device. Currently works with USB and returns an EIP1193 provider.

## Implementation

The implementation is a bit different for D'cent because it is not a Web3Modal supported provider. 

Add the dependecy to your project

```
yarn add @rsksmart/rlogin-dcent-provider --save
```

In your dapp, your rLogin implementation should be similar to this:

```
import RLogin from '@rsksmart/rlogin'
import { dcentProviderOptions } from '@rsksmart/rlogin-dcent-provider'

// ...

const rLogin = new RLogin({
  cacheProvider: false,
  providerOptions: {
    // ... other providers, i.e. WalletConnect or Portis, etc
      'custom-dcent': {
      ...dcentProviderOptions,
      options: {
        rpcUrl: 'https://public-node.testnet.rsk.co',
        chainId: 31,
        debug: true
      }
  },
  supportedChains: [30, 31]
})
```
## Security advice

Protect your cold wallet with your finger print/pin. Do the same for your app.

## Setup

After you secure your wallet, you need to create an account to do transactions. You can do this from your cold wallet or from your App.

## Firmware update

We strongly recommend update your cold wallet firmware to the latest version.

## App or software walllet

Android and Iphone supported.
D'cent have an app for manage accounts and networks: https://dcentwallet.com/MobileApp
For testnet you need to go to "Setting" and enable testnet support. After that go to "Discovery" and find the "Network" option to change the network.

## Pair your wallet

App needed. Remember turn on the "Bluetooth" from your cellphone for pair the D'cent wallet. Now you can press the "Bluetooth red icon" in the "Account" option for list your cold wallets in range and be paired.
You will see a blue led dot in the right corner of your cold wallet if you succeed.

## Docs

You can check the official documentation here: https://userguide.dcentwallet.com/

### Implementation notes

- Similar to the Portis connector, you can only specify a single chainId to connect to.
- The `custom-` needs to be connection data and provider configuration. The `...dcentProviderOptions` contains the D'cent text and image and connects rLogin to the provider.
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

## Disclaimer

`@rsksmart/rlogin-dcent-provider` depends on [`@rsksmart/dcent-provider`](https://github.com/rsksmart/dcent-provider) that is a fork from [`dcent-provider`](https://github.com/DcentWallet/dcent-provider), modified just to allow using RSK standard derivation path. The latest has dependencies with known vulnerabilities not yet fixed, and a `-beta` tag on its name. This being said, Any problem caused by this is under the responsibility of whoever installs this package. We are going to keep our `-beta` tag until this is fixed.
