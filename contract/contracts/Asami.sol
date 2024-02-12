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
    bool needsUpdate;
    string newUsername;
    uint256 newPrice;
    uint256 newScore;
    uint256[] newTopics;
    uint256 lastUpdated;
  }
  Handle[] public handles;
  event HandleSaved(Handle handle);

  function getHandles() public view returns (Handle[] memory) {
    return handles;
  }

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
    bool fundedByAdmin;
  }
  Campaign[] public campaigns;
  event CampaignSaved(Campaign campaign);
  function getCampaigns() public view returns (Campaign[] memory) {
    return campaigns;
  }

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

  uint256 public defaultFeeRate = 10 * 1e18;
  uint256 public feeRate = defaultFeeRate;
  uint256 public votedFeeRate = defaultFeeRate;
  uint256 public votedFeeRateVoteCount = 0;
  /* To make the protocol more predictable, changes are applied only once every period */
  uint256 public lastVotedFeeRateAppliedOn = 0;

  struct FeeRateVote {
    uint256 votes;
    uint256 rate;
  }
  mapping(address => FeeRateVote) public feeRateVotes;

  struct AdminVote {
    uint256 votes;
    address candidate;
    uint256 cycle;
    bool vested;
  }
  /* Maps holders to their vote */
  mapping(address => AdminVote) public submittedAdminVotes;
  event AdminVoteSaved(AdminVote vote);

  struct AdminVoteCount {
    uint256 votes;
    address candidate;
  }

  /*
  We're only storing all historic vote counts to have a view function that helps find out
  which candidate to proclaim without relying on events.
  */
  AdminVoteCount[] public adminVoteCounts;
  mapping(address => uint256) public adminVoteCountIdByCandidate;

  function allAdminVoteCounts() public view returns (AdminVoteCount[] memory) {
    return adminVoteCounts;
  }

  uint256 public vestedAdminVotesTotal;
  uint256 public lastAdminElection;
  address[3] public latestAdminElections;

  function getLatestAdminElections() public view returns (address[3] memory) {
    return latestAdminElections;
  }

  string[] public topics;
  event TopicSaved(uint256 id, string name);

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
    if (submittedAdminVotes[from].votes > 0) { removeAdminVoteHelper(from); }
  }

  function setAdmin(address _admin, address _adminTreasury) external onlyOwner {
    require(admin == address(0) && adminTreasury == address(0));
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

  function adminAddTopics(string[] calldata _names) public onlyAdmin {
    for( uint256 i = 0; i < _names.length; i++) {
      topics.push(_names[i]);
      emit TopicSaved(topics.length, _names[i]);
    }
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
        uint256 unclaimed = account.unclaimedAsamiTokens;
        account.unclaimedAsamiTokens = 0;
        _safeMint(claim.addr, unclaimed);
      }

      if(account.unclaimedDocRewards > 0) {
        uint256 unclaimed = account.unclaimedDocRewards;
        account.unclaimedDocRewards = 0;
        doc.transfer(account.addr, unclaimed);
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
    require(account.addr != address(0), "mc 0");

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
        validUntil: input.validUntil,
        fundedByAdmin: false
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
        validUntil: input.attrs.validUntil,
        fundedByAdmin: true
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
      uint256 fee = (gross * feeRate) / 1e20;
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

      uint256 advertiserTokens = (newTokens * 10 * 1e18) / 100e18;
      uint256 memberTokens = (newTokens * 15 * 1e18) / 100e18;
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
      uint256 remaining = campaign.remaining;
      campaign.remaining = 0;
      address fundedBy = campaign.fundedByAdmin ? admin : advertiser.addr;
      require(doc.transfer(fundedBy, remaining));
      emit CampaignSaved(campaign);
    }
  }

  // To prevent race conditions, we never mint while we're in the middle of a payout.
  function _safeMint(address _addr, uint256 _amount) private {
    require(payoutsRemaining == 0, "SM 0");
    _mint(_addr, _amount);
  }

  struct AdminSetPriceInput {
    uint256 handleId;
    uint256 price;
  }

  function adminSetPrice(AdminSetPriceInput[] calldata _inputs) external onlyAdmin {
    for( uint256 i = 0; i < _inputs.length; i++) {
      Handle storage handle = handles[_inputs[i].handleId - 1];
      Account storage account = accounts[handle.accountId];
      require(account.addr == address(0) && account.id > 0);

      _setPriceHelper(handle, _inputs[i].price);
    }
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
    handle.newPrice = _price;
    handle.needsUpdate = true;
    emit HandleSaved(handle);
  }

  struct AdminSetScoreAndTopicsInput {
    uint256 handleId;
    uint256 score;
    uint256[] topics;
  }

  function adminSetScoreAndTopics(
    AdminSetScoreAndTopicsInput[] calldata _inputs
  ) external onlyAdmin {
    for( uint256 i = 0; i < _inputs.length; i++) {
      Handle storage handle = handles[_inputs[i].handleId - 1];
      handle.newScore = _inputs[i].score;
      handle.newTopics = _inputs[i].topics;
      handle.needsUpdate = true;
      emit HandleSaved(handle);
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

  function applyHandleUpdates(uint256[] calldata _indexes) external {
    uint256 currentCycle = getCurrentCycle();

    for (uint256 i = 0; i < _indexes.length; i++) {
      Handle storage handle = handles[_indexes[i]];
      require(handle.lastUpdated != currentCycle, "ahu 0");
      require(handle.needsUpdate, "ahu 0");

      handle.needsUpdate = false;
      handle.lastUpdated = currentCycle;

      if (bytes(handle.newUsername).length > 0) {
        handle.username = handle.newUsername;
      }

      if (handle.newPrice != 0) {
        handle.price = handle.newPrice;
      }

      if (handle.score != 0) {
        handle.score = handle.newScore;
      }

      if (handle.newTopics.length > 0) {
        handle.topics = handle.newTopics;
      }

      emit HandleSaved(handle);
    }
  }

  function applyVotedFeeRate() external {
    uint256 currentCycle = getCurrentCycle();
    require(currentCycle != lastVotedFeeRateAppliedOn);
    feeRate = votedFeeRate;
    lastVotedFeeRateAppliedOn = currentCycle;
  }

  function submitFeeRateVote(uint256 _rate) external {
    require(_rate > 0 && _rate < 1e20, "sfrv 0");

    FeeRateVote storage vote = feeRateVotes[msg.sender];
    if (vote.votes > 0) { removeFeeRateVoteHelper(msg.sender); }

    uint256 votes = balanceOf(msg.sender);
    require(votes > 0, "sfrv 1");

    votedFeeRate = ((votedFeeRate * votedFeeRateVoteCount) + (votes * _rate)) / (votedFeeRateVoteCount + votes);
    votedFeeRateVoteCount += votes;
    vote.votes = votes;
    vote.rate = _rate;
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
      votedFeeRate = defaultFeeRate;
      votedFeeRateVoteCount = 0;
    }
    delete feeRateVotes[_holder];
  }

  function submitAdminVote(address _candidate) external {
    require(_candidate != address(0), "sav 0");
    uint256 votes = balanceOf(msg.sender);
    require(votes > 0, "sav 1");

    AdminVote storage existing = submittedAdminVotes[msg.sender];
    if (existing.votes > 0 ){
      removeAdminVoteHelper(msg.sender);
    }

    submittedAdminVotes[msg.sender] = AdminVote({
      candidate: _candidate,
      votes: votes,
      cycle: getCurrentCycle(),
      vested: false
    });

    emit AdminVoteSaved(submittedAdminVotes[msg.sender]);
  }

  function vestAdminVotes(address[] calldata _holders) external {
    uint256 thisCycle = getCurrentCycle();

    for (uint256 i = 0; i < _holders.length; i++) {
      AdminVote storage vote = submittedAdminVotes[_holders[i]];
      require(vote.cycle > 0, "vav 0");
      require(vote.cycle < thisCycle, "vav 1");
      require(!vote.vested, "vav 2");
      vote.vested = true;
      vestedAdminVotesTotal += vote.votes;

      uint256 voteCountId = adminVoteCountIdByCandidate[vote.candidate];
      if (voteCountId > 0) {
        AdminVoteCount storage count = adminVoteCounts[voteCountId - 1];
        count.votes += vote.votes;
      } else {
        AdminVoteCount storage count = adminVoteCounts.push();
        count.votes = vote.votes;
        count.candidate = vote.candidate; 
        adminVoteCountIdByCandidate[vote.candidate] = adminVoteCounts.length;
      }
      emit AdminVoteSaved(vote);
    }
  }

  function proclaimCycleAdminWinner(address _candidate) external {
    uint256 voteCountId = adminVoteCountIdByCandidate[_candidate];
    require(voteCountId > 0);
    AdminVoteCount storage count = adminVoteCounts[voteCountId - 1];

    require(count.votes > vestedAdminVotesTotal / 2, "pcw 0");
    require(vestedAdminVotesTotal > 0, "pcw 1");
    uint256 thisCycle = getCurrentCycle();
    require(lastAdminElection < thisCycle, "pcw 2");

    lastAdminElection = thisCycle;
    latestAdminElections[2] = latestAdminElections[1];
    latestAdminElections[1] = latestAdminElections[0];
    latestAdminElections[0] = _candidate;

    if (_candidate == latestAdminElections[1] && _candidate == latestAdminElections[2]) {
      admin = _candidate;
      adminTreasury = _candidate;
    }
  }

  function removeAdminVote() external {
    removeAdminVoteHelper(msg.sender);
  }

  function removeAdminVoteHelper(address _holder) private {
    AdminVote storage vote = submittedAdminVotes[_holder];
    require(vote.votes > 0, "ravh 0");
    if ( vote.vested ) {
      vestedAdminVotesTotal -= vote.votes;

      uint256 voteCountId = adminVoteCountIdByCandidate[vote.candidate];
      require(voteCountId > 0);
      adminVoteCounts[voteCountId - 1].votes -= vote.votes;
    }
    delete submittedAdminVotes[_holder];
  }

  function setAdminAddress(address _admin) external {
    require(msg.sender == adminTreasury);
    admin = _admin;
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
