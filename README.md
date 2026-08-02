# Asami

Asami is a private club to thank and encourage Rootstock bitcoiners.

This repository contains a deliberately small system:

- an upgradeable Foundry contract that stores members and accepted assets and sends one payment per call;
- one Rust binary that indexes events, creates database disbursements, pays members, serves REST data and the landing page, and publishes queued notifications;
- one bilingual React landing page;
- one end-to-end scenario.

There is no token, campaign system, login, wallet connection, GraphQL API, administration UI, or user account system.

## Repository

- contract/src/Asami.sol — upgradeable membership registry and single-payment treasury.
- src — the asami Rust binary.
- migrations — retained historical migrations plus the additive minimal schema.
- pwa — the public landing page.
- integration_tests — the readable Rust E2E fixture and single club scenario.
- scripts/e2e — a thin launcher for the Rust E2E test.
- scripts/run-dev — a thin launcher for the Rust seeded local environment.
- scripts/deploy — production build and optional Foundry deployment.

Initialize dependencies after cloning:

    git submodule update --init --recursive

Foundry's standard `forge install` workflow keeps Solidity dependencies as pinned Git
submodules under `lib/`. In this repository those submodules are `forge-std`,
`openzeppelin-contracts`, and `openzeppelin-contracts-upgradeable`. The project's
contract, Rust, frontend, and end-to-end tests are ordinary files in this repository;
none of the test suites are submodules.

## Local verification

Requirements: Rust, Foundry, Node/npm, Docker, and curl.

Run the fast suites:

    forge test
    cargo test
    npm --prefix pwa ci
    npm --prefix pwa run build

Run the complete scenario:

    scripts/e2e

Explore the seeded landing page interactively:

    scripts/run-dev

The E2E is written in Rust and owns its disposable PostgreSQL container, Anvil
process, API server, ChromeDriver, and browser lifecycle. The pinned ChromeDriver
and Chromium support files live under integration_tests/chromedrivers. On the first
run, the matching Chromium executable is downloaded into that fixture directory.
It covers member indexing, native and ERC-20 distribution, minimum reconsideration,
terminal underfunding, low-gas stopping/refill, unsupported-token alerts, REST
responses, and the rendered landing page.

## Binary commands

Production binaries are built inside Ubuntu 22.04 so they remain compatible with
glibc 2.35 servers. The builder compiles the frontend first because its assets are
embedded in the Rust executable:

    scripts/release-builder

The compatible single binary is written to `release/asami`.

Commands:

- asami migrate
- asami api
- asami worker
- asami worker --once
- asami logs
- asami config
- asami x-auth-url --account asami_english
- asami x-auth-complete --account asami_english --redirected-url URL
- asami recover-unsent --transaction-hash 0x...
- asami demo

The API process serves the frontend and these public read-only endpoints:

- GET /members
- GET /assets
- GET /config
- GET /logs
- GET /health
- GET /ready

## Configuration

Copy .env.example into the deployment secret/configuration system. Secrets must never be committed or logged.

Important settings include:

- DATABASE_URL
- ROOTSTOCK_RPC_URL
- ROOTSTOCK_CHAIN_ID
- ASAMI_CONTRACT_ADDRESS
- CONTRACT_DEPLOY_BLOCK
- AUTOMATOR_MNEMONIC
- AUTOMATOR_DERIVATION_PATH (defaults to the legacy `m/44'/60'/0'/0/0` path)
- DISBURSEMENT_AUTOMATOR_ADDRESS
- MIN_AUTOMATOR_GAS_RESERVE
- GAS_BALANCE_SAFETY_BASIS_POINTS
- FINALITY_DEPTH
- BLOCK_CHUNK_SIZE
- TRANSFER_RECONSIDERATION_LIMIT
- LOW_GAS_TWEET_COOLDOWN_HOURS
- X_DAILY_POST_CAP
- X_CLIENT_ID
- X_CLIENT_SECRET
- X_REDIRECT_URI
- ASAMI_ENCRYPTION_KEY

ASAMI_ENCRYPTION_KEY is a base64-encoded 32-byte key used to encrypt X access and refresh tokens in PostgreSQL.

When `AUTOMATOR_MNEMONIC` is configured, `asami worker` reads one BIP-39
passphrase line from standard input when input is piped. This supports production
wrappers such as `systemd-ask-password --no-tty | asami worker`. When launched
from an interactive terminal, it prompts without echoing instead. The derived
address must equal `DISBURSEMENT_AUTOMATOR_ADDRESS`; otherwise the worker exits
before processing payouts. `AUTOMATOR_PRIVATE_KEY` remains available only for
local development and tests.

The passphrase is never accepted through an environment variable.

## Deployment

Set the three role addresses independently:

- PROXY_ADMIN_ADDRESS
- CONTRACT_OWNER_ADDRESS
- DISBURSEMENT_AUTOMATOR_ADDRESS

Production deployment uses the Trezor account at `m/44'/137'/0'/0/1`. Pass that complete path to Forge; custom derivation paths cannot be combined with `--mnemonic-indexes`. Run:

    DEPLOY_CONTRACT=1 \
    DEPLOYER_ADDRESS=0xd9FCAe4315920387f00725C78285D6D41C30b967 \
    PROXY_ADMIN_ADDRESS=0xd9FCAe4315920387f00725C78285D6D41C30b967 \
    CONTRACT_OWNER_ADDRESS=0xd9FCAe4315920387f00725C78285D6D41C30b967 \
    DISBURSEMENT_AUTOMATOR_ADDRESS=0x3e79325b61d941e7996f0a1aad4f66a703e24faa \
    ROOTSTOCK_RPC_URL=https://public-node.rsk.co \
    scripts/deploy

For routine application deployments after the contracts are already deployed, run:

    scripts/build-and-deploy-to root@167.99.18.108

This opens one persistent SSH control session, builds the frontend and compatible
single binary through `scripts/release-builder`, uploads it with a checksum check,
runs migrations, atomically replaces `/home/asami/asami`, and restarts both systemd
services. It then asks for the automator wallet passphrase through systemd remote
TTY prompt. SSH authentication is requested only once. Override the remote directory
or unit names with `ASAMI_REMOTE_DIR`, `ASAMI_API_SERVICE`, and
`ASAMI_WORKER_SERVICE` when necessary.

Set `DEPLOY_TARGET` as well to copy the single release binary. The deployment JSON records the proxy, implementation, ProxyAdmin, owner, automator, chain ID, and deployment block. Foundry records every deployment transaction hash in:

    broadcast/Deploy.s.sol/<chain-id>/run-latest.json

Run migrations before starting the new service version. Production normally uses two service commands from the same binary:

    asami api
    asami worker

Complete the English X OAuth setup once using x-auth-url and x-auth-complete with the `asami_english` account key.

## Payout model

Each eligible incoming transfer creates one database disbursement and one immutable payout intent per member from the latest synchronized member projection. Every nonzero payout is sent in its own transaction.

The contract has no batch concept, payout identifier, completed-payout mapping, or off-chain disbursement reference. PostgreSQL attempt and receipt records are the only logical payout ledger.

A payout is complete only after a successful receipt is stored. Rootstock payouts
are explicitly signed as legacy transactions. Ambiguous submissions retain the
exact signed transaction and are reconciled or safely rebroadcast before any
later payout can allocate a nonce.

Attempts made by an older binary may lack those signed bytes. After stopping the
worker and independently confirming that each transaction hash is absent from
the node and explorer, recover them explicitly (repeat the option for multiple
hashes):

    asami recover-unsent \
      --transaction-hash 0xFIRST \
      --transaction-hash 0xSECOND

The command checks the receipt, transaction, and pending account nonce before it
marks the corresponding payout retryable. It refuses ambiguous recovery.

If contract funds are unavailable when a transfer is reconsidered, the disbursement and calculated intents are retained as a terminal funds_not_available failure. It is logged and never announced as successful.

## Database-loss procedure

Because payout idempotency is intentionally database-only:

1. Stop the worker.
2. Withdraw every remaining contract asset using the owner.
3. Restore a backup, or migrate an empty database.
4. Reindex from CONTRACT_DEPLOY_BLOCK.
5. Let historical transfers become terminal underfunded disbursements.
6. Resume the worker for new transfers.

Rebuilding without first draining the contract can repeat historical payments.

## Trust model

- The contract owner centrally controls membership, scores, accepted assets, minimums, the automator, pause state, and all withdrawals.
- The proxy administrator can replace the implementation.
- The off-chain worker calculates the 90 percent member distribution.
- The automator submits one payment instruction at a time.
- The contract does not enforce proportional correctness or database idempotency.
- Arbitrary tokens can be sent to the contract; unconfigured tokens are indexed but not distributed.
- Disabled, below-minimum, retained, or unsupported funds remain owner-controlled.
- Finalized block hashes are verified. A deep finalized reorganization halts processing for operator review.
- X posts are operational thank-you or warning messages, never token endorsements.
