const Asami = artifacts.require("Asami");
const AsamiCore = artifacts.require("AsamiCore");

module.exports = async function(deployer, network) {
  switch (deployer.network_id) {
    case 30: // Mainnet
      return (await deployer.deploy(AsamiCore, "0xe700691da7b9851f2f35f8b8182c69c53ccad9db"));

		/*
    case 31: // Testnet
      return (await deployer.deploy(Asami, "0xcb46c0ddc60d18efeb0e586c17af6ea36452dae0", "0x9dca13cb227b6839e75dac0cb9682a9626aa1596"));
		*/

    default:
      const MockDoc = artifacts.require("MockDoc");
      await deployer.deploy(MockDoc);
      await deployer.deploy(Asami, MockDoc.address, "0x0000000000000000000000000000000000000000");
      return (await deployer.deploy(AsamiCore, MockDoc.address));
  }
};
