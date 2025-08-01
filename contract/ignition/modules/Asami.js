const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");
const { ethers } = require("hardhat");

// Addresses are returned by "node"
module.exports = buildModule("LocalAsami", (m) => {
  const memberAddress = process.env.MEMBER_ADDRESS;
  const adminAddress = process.env.ADMIN_ADDRESS;

  const mockDoc = m.contract("MockDoc", []);

  const asamiOld = m.contract("Asami", [mockDoc, adminAddress]);

  const asami = m.contract("AsamiCore", [mockDoc]);

  for (let a of [adminAddress, memberAddress]) {
    m.call(mockDoc, "transfer", [a, ethers.parseEther("420000000")], {
      id: `transfer_doc_to_${a}`,
    });
    m.send(`send_eth_to_${a}`, a, ethers.parseEther("1000"));
  }

  return { asami };
});
