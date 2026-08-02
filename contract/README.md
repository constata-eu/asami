# Asami contract

The contract is built and tested only with Foundry.

    forge fmt --check
    forge test

Deploy:

    DEPLOYER_ADDRESS=0xd9FCAe4315920387f00725C78285D6D41C30b967 \
    PROXY_ADMIN_ADDRESS=0xd9FCAe4315920387f00725C78285D6D41C30b967 \
    CONTRACT_OWNER_ADDRESS=0xd9FCAe4315920387f00725C78285D6D41C30b967 \
    DISBURSEMENT_AUTOMATOR_ADDRESS=0x3e79325b61d941e7996f0a1aad4f66a703e24faa \
    ROOTSTOCK_RPC_URL=... \
    forge script contract/script/Deploy.s.sol:Deploy \
      --rpc-url "$ROOTSTOCK_RPC_URL" \
      --broadcast \
      --trezor \
      --mnemonic-derivation-paths "m/44'/137'/0'/0/1"

The complete Trezor path is `m/44'/137'/0'/0/1`; Forge does not allow combining a custom path with `--mnemonic-indexes`.

The deployment creates a new Asami implementation, TransparentUpgradeableProxy, and ProxyAdmin. Routine member, asset, payment, and withdrawal operations must use the proxy address.

The automator can only call the individual native and token payment methods. It cannot change members, assets, ownership, pause state, upgrades, or arbitrary withdrawals.
