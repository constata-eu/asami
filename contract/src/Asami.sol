// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {
    OwnableUpgradeable
} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {
    PausableUpgradeable
} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import { ReentrancyGuard } from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/// @notice A centrally managed Rootstock club registry and single-payment treasury.
/// @dev Distribution calculations and payment idempotency intentionally live off-chain.
contract Asami is Initializable, OwnableUpgradeable, PausableUpgradeable, ReentrancyGuard {
    using SafeERC20 for IERC20;

    uint256 public constant MAX_NAME_BYTES = 64;
    uint256 public constant MAX_PAGE_SIZE = 200;

    struct Member {
        address account;
        uint96 score;
        string name;
    }

    struct AssetConfig {
        bool enabled;
        uint256 minimumAmount;
    }

    Member[] private _members;
    mapping(address => uint256) private _memberIdPlusOne;
    mapping(address => AssetConfig) private _assetConfigs;
    mapping(address => string) private _senderNames;

    address public automator;

    event MemberUpdated(
        uint256 indexed memberId, address indexed account, uint96 score, string name
    );
    event AssetConfigUpdated(address indexed asset, bool enabled, uint256 minimumAmount);
    event SenderNameUpdated(address indexed account, string name);
    event NativeFundsReceived(address indexed sender, uint256 amount);
    event PaymentSent(address indexed asset, address indexed recipient, uint256 amount);
    event AutomatorUpdated(address indexed previousAutomator, address indexed newAutomator);
    event NativeWithdrawn(address indexed recipient, uint256 amount);
    event TokenWithdrawn(address indexed token, address indexed recipient, uint256 amount);

    error OnlyAutomator();
    error ZeroAddress();
    error ZeroAmount();
    error ZeroScore();
    error EmptyName();
    error NameTooLong();
    error InvalidMemberId();
    error MemberNotActive();
    error DuplicateActiveAddress();
    error PageTooLarge();
    error InsufficientBalance();
    error NativeTransferFailed();

    modifier onlyAutomator() {
        _checkAutomator();
        _;
    }

    function _checkAutomator() internal view {
        if (msg.sender != automator) revert OnlyAutomator();
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address initialOwner, address initialAutomator) external initializer {
        if (initialOwner == address(0) || initialAutomator == address(0)) revert ZeroAddress();
        __Ownable_init(initialOwner);
        __Pausable_init();
        automator = initialAutomator;
        emit AutomatorUpdated(address(0), initialAutomator);
    }

    receive() external payable {
        emit NativeFundsReceived(msg.sender, msg.value);
    }

    function memberCount() external view returns (uint256) {
        return _members.length;
    }

    function member(uint256 memberId)
        external
        view
        returns (address account, uint96 score, string memory name)
    {
        if (memberId >= _members.length) revert InvalidMemberId();
        Member storage value = _members[memberId];
        return (value.account, value.score, value.name);
    }

    function members(uint256 offset, uint256 limit) external view returns (Member[] memory result) {
        if (limit > MAX_PAGE_SIZE) revert PageTooLarge();
        if (offset >= _members.length) return new Member[](0);
        uint256 end = offset + limit;
        if (end > _members.length) end = _members.length;
        result = new Member[](end - offset);
        for (uint256 index = offset; index < end; index++) {
            result[index - offset] = _members[index];
        }
    }

    function addMember(address account, uint96 score, string calldata name)
        external
        onlyOwner
        whenNotPaused
        returns (uint256 memberId)
    {
        _validateMember(account, score, name);
        if (_memberIdPlusOne[account] != 0) revert DuplicateActiveAddress();
        memberId = _members.length;
        _members.push(Member({ account: account, score: score, name: name }));
        _memberIdPlusOne[account] = memberId + 1;
        emit MemberUpdated(memberId, account, score, name);
    }

    function updateMember(uint256 memberId, address account, uint96 score, string calldata name)
        external
        onlyOwner
        whenNotPaused
    {
        if (memberId >= _members.length) revert InvalidMemberId();
        Member storage previous = _members[memberId];
        if (previous.account == address(0)) revert MemberNotActive();
        _validateMember(account, score, name);
        uint256 assignedIdPlusOne = _memberIdPlusOne[account];
        if (assignedIdPlusOne != 0 && assignedIdPlusOne != memberId + 1) {
            revert DuplicateActiveAddress();
        }
        if (previous.account != account) {
            delete _memberIdPlusOne[previous.account];
            _memberIdPlusOne[account] = memberId + 1;
        }
        previous.account = account;
        previous.score = score;
        previous.name = name;
        emit MemberUpdated(memberId, account, score, name);
    }

    function removeMember(uint256 memberId) external onlyOwner whenNotPaused {
        if (memberId >= _members.length) revert InvalidMemberId();
        Member storage previous = _members[memberId];
        if (previous.account == address(0)) revert MemberNotActive();
        delete _memberIdPlusOne[previous.account];
        delete _members[memberId];
        emit MemberUpdated(memberId, address(0), 0, "");
    }

    function setAssetConfig(address asset, bool enabled, uint256 minimumAmount) external onlyOwner {
        _assetConfigs[asset] = AssetConfig({ enabled: enabled, minimumAmount: minimumAmount });
        emit AssetConfigUpdated(asset, enabled, minimumAmount);
    }

    function assetConfig(address asset)
        external
        view
        returns (bool enabled, uint256 minimumAmount)
    {
        AssetConfig storage value = _assetConfigs[asset];
        return (value.enabled, value.minimumAmount);
    }

    function setSenderName(string calldata name) external {
        _setSenderName(msg.sender, name);
    }

    function setSenderNameFor(address account, string calldata name) external onlyOwner {
        if (account == address(0)) revert ZeroAddress();
        _setSenderName(account, name);
    }

    function senderName(address account) external view returns (string memory) {
        return _senderNames[account];
    }

    function setAutomator(address newAutomator) external onlyOwner {
        if (newAutomator == address(0)) revert ZeroAddress();
        address previous = automator;
        automator = newAutomator;
        emit AutomatorUpdated(previous, newAutomator);
    }

    function pause() external onlyOwner {
        _pause();
    }

    function unpause() external onlyOwner {
        _unpause();
    }

    function disburseNative(address payable recipient, uint256 amount)
        external
        onlyAutomator
        whenNotPaused
        nonReentrant
    {
        if (recipient == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();
        if (address(this).balance < amount) revert InsufficientBalance();
        (bool success,) = recipient.call{ value: amount }("");
        if (!success) revert NativeTransferFailed();
        emit PaymentSent(address(0), recipient, amount);
    }

    function disburseToken(IERC20 token, address recipient, uint256 amount)
        external
        onlyAutomator
        whenNotPaused
        nonReentrant
    {
        if (address(token) == address(0) || recipient == address(0)) {
            revert ZeroAddress();
        }
        if (amount == 0) revert ZeroAmount();
        if (token.balanceOf(address(this)) < amount) revert InsufficientBalance();
        token.safeTransfer(recipient, amount);
        emit PaymentSent(address(token), recipient, amount);
    }

    function withdrawNative(address payable recipient, uint256 amount)
        external
        onlyOwner
        nonReentrant
    {
        if (recipient == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();
        if (address(this).balance < amount) revert InsufficientBalance();
        (bool success,) = recipient.call{ value: amount }("");
        if (!success) revert NativeTransferFailed();
        emit NativeWithdrawn(recipient, amount);
    }

    function withdrawToken(IERC20 token, address recipient, uint256 amount)
        external
        onlyOwner
        nonReentrant
    {
        if (address(token) == address(0) || recipient == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();
        if (token.balanceOf(address(this)) < amount) revert InsufficientBalance();
        token.safeTransfer(recipient, amount);
        emit TokenWithdrawn(address(token), recipient, amount);
    }

    function _validateMember(address account, uint96 score, string calldata name) private pure {
        if (account == address(0)) revert ZeroAddress();
        if (score == 0) revert ZeroScore();
        uint256 length = bytes(name).length;
        if (length == 0) revert EmptyName();
        if (length > MAX_NAME_BYTES) revert NameTooLong();
    }

    function _setSenderName(address account, string calldata name) private {
        if (bytes(name).length > MAX_NAME_BYTES) revert NameTooLong();
        _senderNames[account] = name;
        emit SenderNameUpdated(account, name);
    }

    uint256[44] private __gap;
}

