const Asami = artifacts.require("Asami");
const MockDoc = artifacts.require("MockDoc");

module.exports = async function(deployer, network) {
  switch (deployer.network_id) {
    case 31:
      return (await deployer.deploy(Asami, "0xcb46c0ddc60d18efeb0e586c17af6ea36452dae0"));
    case 30:
      return console.log("Not ready for prod yet");
    default:
      await deployer.deploy(MockDoc);
      return (await deployer.deploy(Asami, MockDoc.address));
  }
};
