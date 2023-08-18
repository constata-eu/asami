const Asami = artifacts.require("Asami");
const MockDoc = artifacts.require("MockDoc");
const Schnorr = artifacts.require("Schnorr");

module.exports = async function(deployer) {
  await deployer.deploy(MockDoc);
  return (await deployer.deploy(Asami, MockDoc.address));
};
