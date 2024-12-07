const Asami = artifacts.require("Asami");
const AsamiCore = artifacts.require("AsamiCore");
const MockDoc = artifacts.require("MockDoc");

module.exports = async function(callback) {
  const legacy = await Asami.deployed();
  const asami = await AsamiCore.deployed();
  const doc = await MockDoc.deployed();

  const accounts = await web3.eth.getAccounts();
  const memberAddress = process.env.MEMBER_ADDRESS;
  const adminAddress = process.env.ADMIN_ADDRESS;

	await legacy.setAdminAddress(adminAddress);
	
  for (let a of [adminAddress, memberAddress]) {
    await doc.transfer(a, web3.utils.toWei("420000000", "ether"), { from: accounts[0] });
    await web3.eth.sendTransaction({from: accounts[0], to: a, value: web3.utils.toWei('100', 'ether')})
  }
  console.log(`{"legacy":"${legacy.address}", "asami":"${asami.address}", "doc":"${doc.address}", "deployer":"${accounts[0]}"}`);
  callback();
}

