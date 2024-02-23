const Asami = artifacts.require("Asami");
const MockDoc = artifacts.require("MockDoc");

module.exports = async function(callback) {
  const accounts = await web3.eth.getAccounts();
  const asami = await Asami.deployed();
  const doc = await MockDoc.deployed();

  const memberAddress = process.env.MEMBER_ADDRESS;
  const adminAddress = process.env.ADMIN_ADDRESS;

  await asami.setAdminAddress(adminAddress);

  for (let a of [adminAddress, memberAddress]) {
    await doc.transfer(a, web3.utils.toWei("420000000", "ether"), { from: accounts[0] });
    await web3.eth.sendTransaction({from: accounts[0], to: a, value: web3.utils.toWei('100', 'ether')})
  }
  console.log(`{"asami":"${asami.address}", "doc":"${doc.address}", "deployer":"${accounts[0]}"}`);
  callback();
}

