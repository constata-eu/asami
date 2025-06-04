<p align="middle">
    <img src="https://user-images.githubusercontent.com/766679/236442723-004fc7a5-edb2-4477-86da-0b687d62702f.svg" alt="logo" height="100" >
</p>
<h3 align="middle">rLogin providers</h3>
<p align="middle">
    A set of EIP-1193 providers for rLogin
</p>

This libraries are a set of wrappers that are used by [`@rsksmart/rLogin`](https://github.com/rsksmart/rLogin) that make all integrated web3 providers compatible, complaint with [EIP-1193 - Ethereum Provider JavaScript API](https://eips.ethereum.org/EIPS/eip-1193).

## Packages

EIP-1193 adapters:

- `@rsksmart/rlogin-ledger-provider` - adapter for Ledger [`@ledgerhq/hw-app-eth`](https://github.com/LedgerHQ/ledgerjs/tree/master/packages/hw-app-eth)
- `@rsksmart/rlogin-trezor-provider` - adapter for Trezor [`trezor-connect`](https://github.com/trezor/connect)
- `@rsksmart/rlogin-dcent-provider` - adapter for D'Cent [`dcent-provider`](https://github.com/DcentWallet/dcent-provider)

Internals:

- `@rsksmart/rlogin-eip1193-types` - types for EIP-1193
- `@rsksmart/rlogin-eip1193-proxy-subprovider` - fallbacks RPC requests to [`ethjs-query`](https://github.com/ethjs/ethjs-query)
- `@rsksmart/rlogin-transactions` - completes transaction fields (gas price, gas limit and nonce)
- `@rsksmart/rlogin-dpath` - calculates derivation paths for RSK

## Run for development

First, install all dependencies and link packages

```
npm i
npm run setup
```

The packages are in `/pacakges` folder.

### Run tests

```
npm test
```

### Build for production

```
npm run build
```

### Branching model

- `main` has latest release. Do merge commits.
- `develop` has latest approved PR. PRs need to pass `ci` and _LGTM_. Do squash & merge.
- Use branches pointing to `develop` to add new PRs.
- Do external PRs against latest commit in `develop`.

### Acknowledgement

- Ethereum network: EIP-1559 specifications not supported.
- One by one transaction. Confirm must be reached before do a second transaction.
- Transactions field mandatory: "to". Any others are optionals or provided by the library.
