// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import { Test } from "forge-std/Test.sol";
import { Asami } from "../src/Asami.sol";
import { AsamiV2 } from "../src/AsamiV2.sol";
import { MockToken } from "../src/MockToken.sol";
import {
    ITransparentUpgradeableProxy,
    TransparentUpgradeableProxy
} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import { ProxyAdmin } from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import { ERC1967Utils } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";

contract RejectEther {
    receive() external payable {
        revert("no");
    }
}

contract AsamiTest is Test {
    Asami internal implementation;
    Asami internal asami;
    MockToken internal token;
    TransparentUpgradeableProxy internal proxy;

    address internal proxyAdminOwner = makeAddr("proxyAdminOwner");
    address internal owner = makeAddr("owner");
    address internal automator = makeAddr("automator");
    address internal alice = makeAddr("alice");
    address internal bob = makeAddr("bob");

    function setUp() public {
        implementation = new Asami();
        proxy = new TransparentUpgradeableProxy(
            address(implementation),
            proxyAdminOwner,
            abi.encodeCall(Asami.initialize, (owner, automator))
        );
        asami = Asami(payable(address(proxy)));
        token = new MockToken("Test Token", "TEST");
    }

    function testImplementationCannotBeInitialized() public {
        vm.expectRevert();
        implementation.initialize(owner, automator);
    }

    function testInitializationAndRoles() public view {
        assertEq(asami.owner(), owner);
        assertEq(asami.automator(), automator);
        assertTrue(proxyAdminOwner != owner);
        assertTrue(owner != automator);
    }

    function testMemberLifecycleKeepsPermanentIds() public {
        vm.startPrank(owner);
        uint256 aliceId = asami.addMember(alice, 10, "@alice");
        uint256 bobId = asami.addMember(bob, 20, "Bob");
        assertEq(aliceId, 0);
        assertEq(bobId, 1);

        asami.updateMember(aliceId, alice, 30, "Alice");
        asami.removeMember(aliceId);
        address charlie = makeAddr("charlie");
        uint256 charlieId = asami.addMember(charlie, 40, "Charlie");
        vm.stopPrank();

        assertEq(charlieId, 2);
        assertEq(asami.memberCount(), 3);
        (address removed, uint96 score, string memory name) = asami.member(0);
        assertEq(removed, address(0));
        assertEq(score, 0);
        assertEq(name, "");
    }

    function testDuplicateActiveMemberAddressReverts() public {
        vm.startPrank(owner);
        asami.addMember(alice, 10, "Alice");
        vm.expectRevert(Asami.DuplicateActiveAddress.selector);
        asami.addMember(alice, 20, "Also Alice");
        vm.stopPrank();
    }

    function testMemberPagination() public {
        vm.startPrank(owner);
        asami.addMember(alice, 10, "Alice");
        asami.addMember(bob, 20, "Bob");
        vm.stopPrank();
        Asami.Member[] memory page = asami.members(1, 1);
        assertEq(page.length, 1);
        assertEq(page[0].account, bob);
    }

    function testAssetConfigurationAndSenderNames() public {
        vm.prank(owner);
        asami.setAssetConfig(address(token), true, 100 ether);
        (bool enabled, uint256 minimum) = asami.assetConfig(address(token));
        assertTrue(enabled);
        assertEq(minimum, 100 ether);

        vm.prank(alice);
        asami.setSenderName("Alice Example");
        assertEq(asami.senderName(alice), "Alice Example");
        vm.prank(owner);
        asami.setSenderNameFor(alice, "@alice");
        assertEq(asami.senderName(alice), "@alice");
    }

    function testReceivesAndDisbursesNativeIndividually() public {
        vm.deal(alice, 10 ether);
        vm.prank(alice);
        (bool received,) = address(asami).call{ value: 10 ether }("");
        assertTrue(received);

        uint256 before = bob.balance;
        vm.prank(automator);
        asami.disburseNative(payable(bob), 2 ether);
        assertEq(bob.balance - before, 2 ether);
        assertEq(address(asami).balance, 8 ether);
    }

    function testOnlyAutomatorCanDisburse() public {
        vm.deal(address(asami), 1 ether);
        vm.prank(owner);
        vm.expectRevert(Asami.OnlyAutomator.selector);
        asami.disburseNative(payable(bob), 1 ether);
    }

    function testNativePaymentFailureLeavesBalance() public {
        RejectEther rejector = new RejectEther();
        vm.deal(address(asami), 1 ether);
        vm.prank(automator);
        vm.expectRevert(Asami.NativeTransferFailed.selector);
        asami.disburseNative(payable(address(rejector)), 1 ether);
        assertEq(address(asami).balance, 1 ether);
    }

    function testDisbursesTokenIndividually() public {
        token.mint(address(asami), 100 ether);
        vm.prank(automator);
        asami.disburseToken(token, alice, 30 ether);
        assertEq(token.balanceOf(alice), 30 ether);
        assertEq(token.balanceOf(address(asami)), 70 ether);
    }

    function testPauseAndAutomatorReplacement() public {
        vm.startPrank(owner);
        asami.pause();
        asami.setAutomator(bob);
        vm.stopPrank();
        vm.deal(address(asami), 1 ether);
        vm.prank(bob);
        vm.expectRevert();
        asami.disburseNative(payable(alice), 1 ether);
        vm.prank(owner);
        asami.unpause();
        vm.prank(bob);
        asami.disburseNative(payable(alice), 1 ether);
    }

    function testOwnerWithdrawals() public {
        vm.deal(address(asami), 2 ether);
        token.mint(address(asami), 3 ether);
        vm.startPrank(owner);
        asami.withdrawNative(payable(alice), 1 ether);
        asami.withdrawToken(token, alice, 2 ether);
        vm.stopPrank();
        assertEq(alice.balance, 1 ether);
        assertEq(token.balanceOf(alice), 2 ether);
    }

    function testUpgradePreservesStorage() public {
        vm.prank(owner);
        asami.addMember(alice, 10, "Alice");
        vm.prank(owner);
        asami.setAssetConfig(address(token), true, 123);

        AsamiV2 next = new AsamiV2();
        address adminAddress =
            address(uint160(uint256(vm.load(address(proxy), ERC1967Utils.ADMIN_SLOT))));
        vm.prank(proxyAdminOwner);
        ProxyAdmin(adminAddress)
            .upgradeAndCall(ITransparentUpgradeableProxy(address(proxy)), address(next), "");

        AsamiV2 upgraded = AsamiV2(payable(address(proxy)));
        assertEq(upgraded.version(), 2);
        (address account, uint96 score, string memory name) = upgraded.member(0);
        assertEq(account, alice);
        assertEq(score, 10);
        assertEq(name, "Alice");
        (bool enabled, uint256 minimum) = upgraded.assetConfig(address(token));
        assertTrue(enabled);
        assertEq(minimum, 123);
    }
}
