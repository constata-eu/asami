const Asami = artifacts.require("Asami");
const MockDoc = artifacts.require("MockDoc");

const browserAddr = "0x68aC79AE5D9C9eaf6c783836C35bbcbaCAe7C771";

module.exports = async function(callback) {
  const accounts = await web3.eth.getAccounts();
  const asami = await Asami.deployed();
  const doc = await MockDoc.deployed();

  await doc.transfer(browserAddr, web3.utils.toWei("50", "ether"), { from: accounts[0] });
  await web3.eth.sendTransaction({from: accounts[0], to: browserAddr, value: web3.utils.toWei('100', 'ether')})
  console.log("Asami contract address is: ", asami.address);
  console.log("Doc contract address is: ", doc.address);

  callback();
}
