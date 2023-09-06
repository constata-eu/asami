const Asami = artifacts.require("Asami");
const MockDoc = artifacts.require("MockDoc");

const browserAddr = "0x68aC79AE5D9C9eaf6c783836C35bbcbaCAe7C771";

module.exports = async function(callback) {
  const accounts = await web3.eth.getAccounts();
  const asami = await Asami.deployed();
  const doc = await MockDoc.deployed();

  for (let s of ["Instagram", "Twitter", "Youtube", "Facebook", "TikTok"]) { await asami.addSocialNetwork(s); }
  await doc.transfer(browserAddr, web3.utils.toWei("50", "ether"), { from: accounts[0] });
  await web3.eth.sendTransaction({from: accounts[0], to: browserAddr, value: web3.utils.toWei('100', 'ether')})
  console.log(`const asamiAddress = "${asami.address}";`);
  console.log(`const docAddress = "${doc.address}";`);

  callback();
}
