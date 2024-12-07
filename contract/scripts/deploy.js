/*
  switch (deployer.network_id) {
    case 30: // Mainnet
      return (await deployer.deploy(AsamiCore, "0xe700691da7b9851f2f35f8b8182c69c53ccad9db"));
    case 31: // Testnet
      return (await deployer.deploy(Asami, "0xcb46c0ddc60d18efeb0e586c17af6ea36452dae0", "0x9dca13cb227b6839e75dac0cb9682a9626aa1596"));
*/

const hre = require("hardhat");

async function main() {
  const Asami = await hre.ethers.getContractFactory("Asami");
  const AsamiCore = await hre.ethers.getContractFactory("AsamiCore");
  const MockDoc = await hre.ethers.getContractFactory("MockDoc");


  const mockDoc = await MockDoc.deploy();
  await mockDoc.deployed();

  const asami = await Asami.deploy(mockDoc.address, "0x0000000000000000000000000000000000000000");
  await asami.deployed();

  const asamiCore = await AsamiCore.deploy(mockDock.address);

  await asamiCore.deployed();


  console.log("Contract deployed to:", asamiCore.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
