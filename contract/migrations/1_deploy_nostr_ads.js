const NostrAds = artifacts.require("NostrAds");

module.exports = function(deployer) {
  deployer.deploy(NostrAds);
};
