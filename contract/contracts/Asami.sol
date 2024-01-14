// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Capped.sol";

contract Asami is Ownable, ERC20Capped {
  enum Site { X, Instagram, LinkedIn, Facebook, Nostr}

  struct Account {
    uint256 id;
    address addr;
    uint256 unclaimedAsamiTokens;
    uint256 unclaimedDocRewards;
    uint256[] handles;
    uint256[] campaigns;
    uint256[] collabs;
  }
  uint256[] public accountIds;
  mapping(uint256 => Account) public accounts;
  mapping(address => uint256) public accountIdByAddress;
  event AccountSaved(Account account);

  struct Handle {
    uint256 id;
    uint256 accountId;
    Site site;
    uint256 price;
    uint256 score;
    uint256[] topics;
    string username;
    string userId;
  }
  Handle[] public handles;
  event HandleSaved(Handle handle);

  struct HandleUpdate {
    uint256 id;
    uint256 handleId;
    string username;
    uint256 price;
    uint256 score;
    uint256[] topics;
  }
  HandleUpdate[] public handleUpdates;
  mapping(uint256 => uint256) public handleUpdateByHandleId;
  event HandleUpdateSaved(HandleUpdate handleUpdate);

  struct Campaign {
    uint256 id;
    uint256 accountId;
    Site site;
    uint256 budget;
    uint256 remaining;
    string contentId;
    uint256 priceScoreRatio;
    uint256[] topics;
    uint256 validUntil;
  }
  Campaign[] public campaigns;
  event CampaignSaved(Campaign campaign);

  struct Collab {
    uint256 id;
    uint256 handleId;
    uint256 campaignId;
    uint256 gross;
    uint256 fee;
    string proof;
    uint256 createdAt;
  }
  Collab[] public collabs;
  event CollabSaved(Collab collab);

  uint256 public feeRate = 10; // It's a percentage.
  uint256 public defaultVotedFeeRate = 10 * 1e18;
  uint256 public votedFeeRate = defaultVotedFeeRate;
  uint256 public votedFeeRateVoteCount = 0;
  /* To make the protocol more predictable, changes are applied only once every period */
  uint256 public lastVotedFeeRateAppliedOn = 0;

  struct FeeRateVote {
    uint256 votes;
    uint256 rate;
  }
  mapping(address => FeeRateVote) public feeRateVotes;

  string[] public topics;

  IERC20 internal doc;
  address[] public holders;
  mapping(address => bool) public trackedHolders;

  address public admin; // An address that may be stored on a server.
  address public adminTreasury; // A cold storage address for admin money.

  uint256 public claimedAsamiTokens;

  modifier onlyAdmin() {
    require(msg.sender == admin || msg.sender == owner());
    _;
  }

  constructor(address _dollarOnChainAddress) ERC20("AsamiToken", "ASAMI") ERC20Capped(21 * 1e24) {
    doc = IERC20(_dollarOnChainAddress);
  }

  function _beforeTokenTransfer(address from, address to, uint256 amount) internal override {
    super._beforeTokenTransfer(from, to, amount);
    if (!trackedHolders[to] && amount > 0) {
      holders.push(to);
      trackedHolders[to] = true;
    }

    if (feeRateVotes[from].votes > 0) { removeFeeRateVoteHelper(from); }
  }

  function setAdmin(address _admin, address _adminTreasury) external onlyOwner {
    admin = _admin;
    adminTreasury = _adminTreasury;
  }

  function getHolders() public view returns (string[] memory) {
    return topics;
  }

  function getClaimedAsamiTokens() public view returns (uint256) {
    return claimedAsamiTokens;
  }

  function getTopics() public view returns (string[] memory) {
    return topics;
  }

  function addTopic(string calldata _name) public onlyAdmin {
    topics.push(_name);
  }

  function ensureAccount(uint256 _accountId) private {
    if(accounts[_accountId].id == 0) {
      accounts[_accountId].id = _accountId;
      accountIds.push(_accountId);
    }
  }

  struct AdminClaimAccountsInput {
    uint256 accountId;
    address addr;
  }

  function claimAccounts (
    AdminClaimAccountsInput[] calldata _input
  ) public onlyAdmin {
    for( uint256 i = 0; i < _input.length; i++) {
      AdminClaimAccountsInput calldata claim = _input[i];
      ensureAccount(claim.accountId);

      Account storage account = accounts[claim.accountId];
      require(account.addr == address(0));

      accountIdByAddress[claim.addr] = claim.accountId;
      account.addr = claim.addr;

      if(account.unclaimedAsamiTokens > 0) {
        _safeMint(claim.addr, account.unclaimedAsamiTokens);
        account.unclaimedAsamiTokens = 0;
      }

      if(account.unclaimedDocRewards > 0) {
        doc.transfer(account.addr, account.unclaimedDocRewards);
        account.unclaimedDocRewards = 0;
      }

      emit AccountSaved(account);
    }
  }

  function adminMakeHandles (
    Handle[] calldata _inputs
  ) external onlyAdmin {

    for( uint256 i = 0; i < _inputs.length; i++) {
      ensureAccount(_inputs[i].accountId);

      handles.push(_inputs[i]);
      Handle storage h = handles[handles.length - 1];
      h.id = handles.length;
      emit HandleSaved(h);
    }
  }

  struct CampaignInput {
    Site site;
    uint256 budget;
    string contentId;
    uint256 priceScoreRatio;
    uint256[] topics;
    uint256 validUntil;
  }
  function makeCampaigns(
    CampaignInput[] calldata _inputs
  ) public {
    Account storage account = accounts[accountIdByAddress[msg.sender]];
    require(account.addr != address(0));

    for( uint i = 0; i < _inputs.length; i++) {
      CampaignInput memory input = _inputs[i];
      _saveCampaignHelper(account, Campaign({
        id: campaigns.length + 1,
        accountId: account.id,
        site: input.site,
        budget: input.budget,
        remaining: input.budget,
        contentId: input.contentId,
        priceScoreRatio: input.priceScoreRatio,
        topics: input.topics,
        validUntil: input.validUntil
      }));
    }
  }

  struct AdminCampaignInput {
    uint256 accountId;
    CampaignInput attrs;
  }

  function adminMakeCampaigns(
    AdminCampaignInput[] calldata _inputs
  ) public onlyAdmin {

    for( uint i = 0; i < _inputs.length; i++) {
      AdminCampaignInput memory input = _inputs[i];
      ensureAccount(input.accountId);
      Account storage account = accounts[input.accountId];
      require(account.addr == address(0));

      _saveCampaignHelper(account, Campaign({
        id: campaigns.length + 1,
        accountId: account.id,
        site: input.attrs.site,
        budget: input.attrs.budget,
        remaining: input.attrs.budget,
        contentId: input.attrs.contentId,
        priceScoreRatio: input.attrs.priceScoreRatio,
        topics: input.attrs.topics,
        validUntil: input.attrs.validUntil
      }));
    }
  }

  function _saveCampaignHelper(
    Account storage _account,
    Campaign memory _campaign
  ) private {
    require(_campaign.budget > 0);
    require(bytes(_campaign.contentId).length > 0);
    require(doc.transferFrom(msg.sender, address(this), _campaign.budget));

    campaigns.push(_campaign);
    _account.campaigns.push(_campaign.id);
    emit CampaignSaved(_campaign);
  }

  struct AdminMakeCollabsInput {
    uint256 campaignId;
    uint256 handleId;
  }

  function adminMakeCollabs(AdminMakeCollabsInput[] calldata _collabs) external onlyAdmin {
    for( uint256 i = 0; i < _collabs.length; i++) {
      Handle storage handle = handles[_collabs[i].handleId - 1];
      require(handle.id > 0, "1");
      Account storage member = accounts[handle.accountId];

      Campaign storage campaign = campaigns[_collabs[i].campaignId - 1];
      require(campaign.id > 0, "3");
      require(campaign.validUntil > block.timestamp, "4");
      require(isSubset(handle.topics, campaign.topics), "5");

      require(campaign.remaining >= handle.price, "6");
      require(campaign.site == handle.site, "7");
      require((handle.price / handle.score) <= campaign.priceScoreRatio, "8");

      Account storage advertiser = accounts[campaign.accountId];

      uint256 gross = handle.price;
      uint256 fee = (gross * feeRate) / 100;
      uint256 reward = gross - fee;

      Collab memory collab = Collab({
        id: collabs.length + 1,
        handleId: handle.id,
        campaignId: campaign.id,
        gross: gross,
        fee: fee,
        proof: "",
        createdAt: block.timestamp
      });
      collabs.push(collab);
      member.collabs.push(collab.id);

      campaign.remaining -= gross;
      feePool += fee;

      uint256 remainingToCap = cap() - claimedAsamiTokens;
      uint256 newTokens = (fee > remainingToCap) ? remainingToCap : fee;
      claimedAsamiTokens += newTokens;

      uint256 advertiserTokens = (newTokens * 10) / 100;
      uint256 memberTokens = (newTokens * 15) / 100;
      uint256 adminTokens = newTokens - advertiserTokens - memberTokens;

      if (adminTokens > 0) {
        _safeMint(adminTreasury, adminTokens);
      }

      if ( member.addr == address(0) ) {
        if( memberTokens > 0) {
          member.unclaimedAsamiTokens += memberTokens;
        }
        member.unclaimedDocRewards += reward;
        emit AccountSaved(member);
      } else {
        if( memberTokens > 0) {
          _safeMint(member.addr, memberTokens);
        }
        doc.transfer(member.addr, reward);
      }

      if ( advertiserTokens > 0 ) {
        if ( advertiser.addr == address(0) ) {
          advertiser.unclaimedAsamiTokens += advertiserTokens;
          emit AccountSaved(advertiser);
        } else {
          _safeMint(advertiser.addr, advertiserTokens);
        }
      }

      emit CollabSaved(collab);
      emit CampaignSaved(campaign);
    }
  }

  function reimburseDueCampaigns(uint256[] calldata _campaignIds) public {
    for(uint i = 0; i < _campaignIds.length; i++) {
      Campaign storage campaign = campaigns[_campaignIds[i] - 1];
      require(campaign.id > 0);
      require(campaign.remaining > 0);
      require(campaign.validUntil < block.timestamp);

      Account storage advertiser = accounts[campaign.accountId];
      require(doc.transfer(advertiser.addr, campaign.remaining));

      campaign.remaining = 0;
      emit CampaignSaved(campaign);
    }
  }

  // To prevent race conditions, we never mint while we're in the middle of a payout.
  function _safeMint(address _addr, uint256 _amount) private {
    require(payoutsRemaining == 0, "SM 0");
    _mint(_addr, _amount);
  }

  function adminSetPrice(uint256 _handleId, uint256 _price) external onlyAdmin {
    Handle storage handle = handles[_handleId - 1];
    Account storage account = accounts[handle.accountId];
    require(account.addr == address(0) && account.id > 0);

    _setPriceHelper(handle, _price);
  }

  function setPrice(uint256 _handleId, uint256 _price) external {
    Account storage account = accounts[accountIdByAddress[msg.sender]];
    require(account.addr != address(0));

    Handle storage handle = handles[_handleId - 1];
    require(handle.id > 0);
    require(handle.accountId == account.id);

    _setPriceHelper(handle, _price);
  }

  function _setPriceHelper(Handle storage handle, uint256 _price) private {
    require(_price > 0);
    HandleUpdate storage update = _getOrCreateHandleUpdate(handle.id);
    update.price = _price;
    emit HandleUpdateSaved(update);
  }

  function adminSetScoreAndTopics(
    uint256 _handleId,
    uint256 _score,
    uint256[] calldata _topics
  ) external onlyAdmin {
    HandleUpdate storage update = _getOrCreateHandleUpdate(_handleId);
    update.score = _score;
    update.topics = _topics;
    emit HandleUpdateSaved(update);
  }

  function _getOrCreateHandleUpdate(uint256 _handleId) private returns (HandleUpdate storage) {
    // To prevent race conditions in the incremental updating process,
    // handle updates are inaccesible while they are being applied to handles.
    require(updatesRemaining == 0);

    uint256 existingId = handleUpdateByHandleId[_handleId];

    if(existingId > 0) {
      return handleUpdates[existingId - 1];
    } else {
      HandleUpdate storage update = handleUpdates.push();
      update.id = handleUpdates.length;
      update.handleId = _handleId;
      handleUpdateByHandleId[_handleId] = update.id;
      return update;
    }
  }

  function getCycleLength() public pure returns (uint256) {
    return 60 * 60 * 24 * 15;
  }

  function getCurrentCycle() public view returns (uint256) {
     return block.timestamp / getCycleLength();
  }

  function getNextCycle() public view returns (uint256) {
    return getCurrentCycle() + getCycleLength();
  }

  /*
    Distributes the fee pool among token holders.
    To prevent going over the gas limit, it does up to 100 a time, and can be called many times in a row.
    Always returns the pending holders.
    Anyone can call this at any time, but just once per period. It would usually be the admin though.
  */
  uint256 public feePool;
  uint256 public payoutsRemaining = 0;
  uint256 public payoutsTotal = 0;
  uint256 public lastPayoutCycle = 0;

  function distributeFeePool() external {
    uint256 totalSupply = totalSupply();

    if(payoutsRemaining == 0) {
      // We're starting a new payout of the feePool.
      // We can only start one payout in each 15 day period.
      payoutsTotal = holders.length;
      payoutsRemaining = payoutsTotal;
      uint256 currentCycle = getCurrentCycle();
      require(currentCycle != lastPayoutCycle);
      lastPayoutCycle = currentCycle;
    }

    uint256 startAt = payoutsTotal - payoutsRemaining;

    uint256 until = (startAt + 100) <= payoutsRemaining ? (startAt + 100) : payoutsRemaining;

    for (uint256 i = startAt; i < until; i++) {
      address holder = holders[i];
      uint256 balance = balanceOf(holder);
      uint256 reward = (balance * feePool) / totalSupply;
      require(doc.transfer(holder, reward));
    }

    payoutsRemaining -= until;

    if(payoutsRemaining == 0) {
      feePool = 0;
    }
  }

  uint256 public updatesRemaining = 0;
  uint256 public updatesTotal = 0;
  uint256 public lastUpdateCycle = 0;

  function applyHandleUpdates() external {
    if(updatesRemaining == 0) {
      // We're starting a new updates cycle.
      // We can only start one payout in each 15 day period.
      updatesTotal = handleUpdates.length;
      updatesRemaining = updatesTotal;
      uint256 currentCycle = getCurrentCycle();
      require(currentCycle != lastUpdateCycle);
      lastUpdateCycle = currentCycle;
    }

    uint256 startAt = updatesTotal - updatesRemaining;

    uint256 until = (startAt + 100) <= updatesTotal ? (startAt + 100) : updatesTotal;

    for (uint256 i = startAt; i < until; i++) {
      HandleUpdate storage update = handleUpdates[i];
      Handle storage handle = handles[update.handleId - 1];

      if (bytes(update.username).length > 0) {
        handle.username = update.username;
      }

      if (update.price != 0) {
        handle.price = update.price;
      }

      if (update.score != 0) {
        handle.score = update.score;
      }

      if (update.topics.length > 0) {
        handle.topics = update.topics;
      }

      emit HandleSaved(handle);
    }

    updatesRemaining -= until;

    if(updatesRemaining == 0) {
      delete handleUpdates;
    }
  }

  function applyVotedFeeRate() external {
    uint256 currentCycle = getCurrentCycle();
    require(currentCycle != lastVotedFeeRateAppliedOn);
    feeRate = votedFeeRate / 1e18;
    lastVotedFeeRateAppliedOn = currentCycle;
  }

  function submitFeeRateVote(uint256 _rate) external {
    require(_rate > 0 && _rate < 100, "sfrv 0");

    uint256 preciseRate = _rate * 1e18;

    FeeRateVote storage vote = feeRateVotes[msg.sender];
    if (vote.votes > 0) { removeFeeRateVoteHelper(msg.sender); }

    uint256 votes = balanceOf(msg.sender);
    require(votes > 0, "sfrv 1");

    votedFeeRate = ((votedFeeRate * votedFeeRateVoteCount) + (votes * preciseRate)) / (votedFeeRateVoteCount + votes);
    votedFeeRateVoteCount += votes;
    vote.votes = votes;
    vote.rate = preciseRate;
  }

  function removeFeeRateVote() external {
    removeFeeRateVoteHelper(msg.sender);
  }

  function removeFeeRateVoteHelper(address _holder) private {
    FeeRateVote storage vote = feeRateVotes[_holder];
    require(vote.votes > 0, "rfrv 0");
    uint256 nextVoteCount = votedFeeRateVoteCount - vote.votes;
    uint256 currentVolume = votedFeeRate * votedFeeRateVoteCount;
    uint256 removedVolume = vote.votes * vote.rate;
    uint256 nextVolume = currentVolume > removedVolume ? currentVolume - removedVolume : 0;

    if (nextVoteCount > 0) {
      votedFeeRate = nextVolume / nextVoteCount;
      votedFeeRateVoteCount = nextVoteCount;
    } else {
      votedFeeRate = defaultVotedFeeRate;
      votedFeeRateVoteCount = 0;
    }
    delete feeRateVotes[_holder];
  }

  function isSubset(uint256[] memory mainSet, uint256[] memory subset) public pure returns (bool) {
    for (uint256 i = 0; i < subset.length; i++) {
      bool found = false;

      for (uint256 j = 0; j < mainSet.length; j++) {
        if (subset[i] == mainSet[j]) {
          found = true;
          break;
        }
      }

      if (!found) {
        return false;
      }
    }

    return true;
  }
}
