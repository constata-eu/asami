// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import { Script, console2 } from "forge-std/Script.sol";
import { Asami } from "../src/Asami.sol";
import {
    TransparentUpgradeableProxy
} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { ERC1967Utils } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";

contract Deploy is Script {
    function run() external returns (Asami implementation, TransparentUpgradeableProxy proxy) {
        address deployer = vm.envAddress("DEPLOYER_ADDRESS");
        address proxyAdminOwner = vm.envAddress("PROXY_ADMIN_ADDRESS");
        address owner = vm.envAddress("CONTRACT_OWNER_ADDRESS");
        address automator = vm.envAddress("DISBURSEMENT_AUTOMATOR_ADDRESS");

        // The signer is supplied by Forge (for production, the Trezor account).
        vm.startBroadcast(deployer);
        implementation = new Asami();
        bytes memory initialization = abi.encodeCall(Asami.initialize, (owner, automator));
        proxy = new TransparentUpgradeableProxy(
            address(implementation), proxyAdminOwner, initialization
        );
        vm.stopBroadcast();

        address proxyAdmin =
            address(uint160(uint256(vm.load(address(proxy), ERC1967Utils.ADMIN_SLOT))));
        string memory object = "deployment";
        vm.serializeAddress(object, "implementation", address(implementation));
        vm.serializeAddress(object, "proxy", address(proxy));
        vm.serializeAddress(object, "proxyAdmin", proxyAdmin);
        vm.serializeAddress(object, "owner", owner);
        vm.serializeAddress(object, "automator", automator);
        vm.serializeUint(object, "chainId", block.chainid);
        string memory json = vm.serializeUint(object, "deploymentBlock", block.number);
        vm.writeJson(json, vm.envOr("DEPLOYMENT_OUTPUT", string("deployment.json")));

        console2.log("Asami proxy", address(proxy));
        console2.log("Asami implementation", address(implementation));
        console2.log("ProxyAdmin", proxyAdmin);
    }
}
