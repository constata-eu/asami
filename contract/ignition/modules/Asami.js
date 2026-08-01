const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");
const { ethers } = require("hardhat");

// Addresses are returned by "node"
module.exports = buildModule("LocalAsami", (m) => {
  const memberAddress =
    process.env.MEMBER_ADDRESS ?? "0xbe992ec27E90c07caDE70c6C3CD26eECC8CadCfE";
  const adminAddress =
    process.env.ADMIN_ADDRESS ?? "0xed4e67213c7a375af60893fe8e0852d0f7040913";

  const mockDoc = m.contract("MockDoc", []);

  const asamiOld = m.contract("Asami", [mockDoc, adminAddress]);

  const asami = m.contract("AsamiCore", [mockDoc]);

  for (let a of [adminAddress, memberAddress]) {
    m.call(mockDoc, "transfer", [a, ethers.parseEther("420000000")], {
      id: `transfer_doc_to_${a}`,
    });
    m.send(`send_eth_to_${a}`, a, ethers.parseEther("1000"));
  }

  return { asamiOld, asami, mockDoc };
});
