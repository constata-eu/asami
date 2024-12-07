require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  networks: {
    hardhat: {
      initialBaseFeePerGas: "10",
      blockGasLimit: 10000000,  // Optional: Set a consistent block gas limit
      mining: {
        mempool: {
          order: "fifo"
        }
      }
    }
  },
  solidity: {
    version: "0.8.19",
    settings: {          // See the solidity docs for advice about optimization and evmVersion
      optimizer: {
        enabled: true,
        runs: 200,
      }
    }
  }
};



