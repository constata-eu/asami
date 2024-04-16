// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Capped.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "./Asami.sol";

contract AsamiCore is ERC20Capped, ReentrancyGuard {
    struct Account {
      bool claimed;
      address trustedAdmin;
      uint256 maxGaslessDocToSpend;
      uint256 minGaslessRbtcToReceive;
      uint256 unclaimedAsamiBalance;
      uint256 unclaimedDocBalance;
      bool adminCanMove;
      mapping( uint256 => Campaign ) campaigns;
    }
    mapping(address => Account) public accounts;

    struct Campaign {
        uint256 budget;
        uint256 validUntil;
        uint256 reportHash;
    }

    function getCampaign(uint256 _accountId, uint256 _briefingHash) public view returns (Campaign memory) {
        return accounts[_accountId].campaigns[_briefingHash];
    }

    /* To make the protocol more predictable, changes are applied only once every period */
    uint256 public defaultFeeRate = 10 * 1e18;
    uint256 public feeRate = defaultFeeRate;
    uint256 public votedFeeRate = defaultFeeRate;
    uint256 public votedFeeRateVoteCount = 0;
    uint256 public lastVotedFeeRateAppliedOn = 0;

    struct FeeRateVote {
        uint256 votes;
        uint256 rate;
    }
    mapping(address => FeeRateVote) public feeRateVotes;

    IERC20 internal doc;
    uint256 public assignedAsamiTokens;

    event AccountUpdated(uint256 accountId);
    event CampaignCreated(uint256 accountId, uint256 campaignId);
    event CampaignSaved(uint256 accountId, uint256 campaignId);
    event CampaignExtended(uint256 accountId, uint256 campaignId);
    event CampaignToppedUp(uint256 accountId, uint256 campaignId);
    event CampaignReimbursed(uint256 accountId, uint256 campaignId);
    event ReportSubmitted(uint256 accountId, uint256 campaignId);
    event CollabSaved(uint256 advertiserId, uint256 briefing, uint256 accountId, uint256 gross, uint256 fee);

    constructor(
        address _dollarOnChainAddress,
        address _adminAddress
    ) ERC20("Asami Core", "ASAMI") ERC20Capped(21 * 1e24) {
        doc = IERC20(_dollarOnChainAddress);
        adminTreasury = msg.sender;
        admin = _adminAddress;
    }

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override {
        super._beforeTokenTransfer(from, to, amount);
        if (from != address(0)) {
          _registerRecentBalanceChange(from, amount, false);

          if (feeRateVotes[from].votes > 0) {
              removeFeeRateVoteHelper(from);
          }
          if (submittedAdminVotes[from].votes > 0) {
              removeAdminVoteHelper(from);
          }
        }

        if (to != address(0)) {
          _registerRecentBalanceChange(to, amount, true);
        }
    }

    struct ClaimAccountsParam {
        uint256 accountId;
        address addr;
    }

    function moveAccount(address _to) external {
      require(!accounts[_to].claimed, "ma0");
      accounts[_to] = accounts[msg.sender];
      accounts[msg.sender] = Account(false, address(0), 0, 0, 0, 0, false, false);
    }

    function setTrustedAdmin(address _trustedAdmin, bool _adminCanChangeAddr) external {
      Account storage account = accounts[msg.sender];
      account.trustedAdmin = _newTrustedAdmin;
      account.adminCanChangAddr = _adminCanChangeAddr;
    }

    function setGaslessRequirements(uint256 _maxGaslessDocToSpend, uint256 _minGaslessRbtcToReceive) external {
      Account storage account = accounts[msg.sender];
      account.maxGaslessDocToSpend = _maxGaslessDocToSpend;
      account.minGaslessDocToReceive = _minGaslessRbtcToReceive;
    }

    /* Admin knows about the new address */
    function adminMoveAccount(address _from, address _to) external {
      require(accounts[_from].trustedAdmin == msg.sender, "ama0");
      require(accounts[_from].adminCanMove, "ama1");
      require(!accounts[_to].claimed, "ma2");
      accounts[_to] = accounts[_from];
      accounts[_to].adminCanMove = false;
      accounts[_from] = Account(false, address(0), 0, 0, 0, 0, false, false);
    }

    /* 
      Instead of claiming, the admin can use an account on their behalf,
      then the account owner can transfer ownership.
      Anyone can transfer ownership of their account.
      - The account's admin can have a special permission to delegate.
      - This would make it easier for the account's admin to call any delegated action.
      - Admin delegation becomes reversible too.
    */
    function claimAccounts(
        ClaimAccountsParam[] calldata _input
    ) external nonReentrant {
        for (uint256 i = 0; i < _input.length; i++) {
            ClaimAccountsParam calldata claim = _input[i];

            Account storage account = accounts[claim.accountId];
            require(account.addr == address(0), "ca0");
            require(accountIdByAddress[claim.addr] == 0, "ca1");

            accountIdByAddress[claim.addr] = claim.accountId;
            account.addr = claim.addr;
            account.approvedGaslessAmount = 5e18; 
        }
    }

    function claimBalances(uint256[] calldata _accountIds) external nonReentrant {
      for (uint256 i = 0; i < _accountIds.length; i++){
        Account storage account = accounts[_accountIds[i]];
        require(account.addr != address(0), "cb0");

        if (account.unclaimedAsamiBalance > 0) {
          uint256 balance = account.unclaimedAsamiBalance;
          account.unclaimedAsamiBalance = 0;
          _safeMint(account.addr, balance);
        }

        if (account.unclaimedDocBalance > 0) {
          uint256 balance = account.unclaimedDocBalance;
          account.unclaimedDocBalance = 0;
          doc.transfer(account.addr, balance);
        }
      }
    }

    /*
      The admin becomes just another account.
      An admin can withdraw on behalf of its users.
      An admin is also an account that accumulates unclaimed balances and can withdraw them.
      A gasless claim can be done by any user.
      As long as the member has reasonable max doc to pay, min rbtc to receive.
      New accounts can be created with reasonable defaults.

      Storing an address, an admin address, and a delegation flag
      Storing an address, an admin address.
      Having the admin there is a good safety measure. Always up to the user.
     

      If anyone can be an admin, how can we prevent someone from registering a single collab for 210.000.000 and getting all the tokens issued.
          - We limit the issuance to only the first collaborations in a period?
             That sounds unfair.

          - Blocks of tokens are auctioned to all fee generators. 
         

          - One asami token represents one DOC that was charged in fees and distributed to all holders.

          - Why bother with real collabs if syntetic collabs can work too?
          
          - Fake collabs are like empty blocks, a way to participate in an auction for the 'block reward' 
          - Real admins get direct rewards for their work and some extras for gasless claims.
          - Every tokens holders votes for the base protocol fee.
          - Admins can set their own markup fee which they get directly, which is added to the base protocol fee.
          - User can set the max markup fee they allow from their admin.
          - Users can tweak all values to be less favorable to them.
          - Admins raising prices may need to ask for higher allowance from members.

          - Defaults apply to all three values with pre-set reasonable amounts. (ie: gasless doc fee to 5, gasless rbtc to deliver)
          - Defaults can be updated by anyone calling a SC function that:
              - Gives enough to pay 300.000 gas at tx.gasprice gwei each.
              - Gets the RBTC price from the MOC contract.
              - Gives them that much Rbtc at the MOC reported price. 
              - Collects that much doc plus the admin fee % 

          - A bad actor can still register bogus transactions to try to claim more issuance.
          - But in doing so they will be paying more DOC fees which will be shared among existing holders.
          - When people catch up to this on one period, they will all do it.
          - We're effectivelly auctioning a number of tokens and to buy them you have to pay fees.
          - There's no reason to pay fees just to get tokens issued. (or is there?)
    */

    function gaslessClaimBalances(
      uint256 _fee,
      uint256 _rbtcAmount,
      uint256[] calldata _accountIds
    ) external payable nonReentrant onlyAdmin {
      require(_rbtcAmount > 1e11, "gcb0");
      require(msg.value == (_rbtcAmount * _accountIds.length), "gcb1");

      for (uint256 i = 0; i < _accountIds.length; i++){
        Account storage account = accounts[_accountIds[i]];
        require(account.addr != address(0), "gcb2");
        require(account.unclaimedDocBalance > _fee, "gcb3");
        require(account.approvedGaslessAmount >= _fee, "gcb4");

        uint256 docs = account.unclaimedDocBalance - _fee;
        account.unclaimedDocBalance = 0;
        doc.transfer(account.addr, docs);
        adminUnclaimedDocBalance += _fee;
        payable(account.addr).transfer(_rbtcAmount);

        if (account.unclaimedAsamiBalance > 0) {
          uint256 balance = account.unclaimedAsamiBalance;
          account.unclaimedAsamiBalance = 0;
          _safeMint(account.addr, balance);
        }
      }
    }

    function changeGaslessApproval(uint256 newAmount) external {
        uint256 accountId = accountIdByAddress[msg.sender];
        Account storage account = accounts[accountId];
        require(account.addr != address(0), "cga0");
        account.approvedGaslessAmount = newAmount;
    }

    function claimAdminUnclaimedBalances() external nonReentrant {
      if (adminUnclaimedAsamiBalance > 0) {
        _safeMint(admin, adminUnclaimedAsamiBalance);
        adminUnclaimedAsamiBalance = 0;
      }

      if (adminUnclaimedDocBalance > 0) {
        doc.transfer(adminTreasury, adminUnclaimedDocBalance);
        adminUnclaimedDocBalance = 0;
      }
    }

    function withdrawAccidentallySentRbtc() external onlyAdmin {
      payable(adminTreasury).transfer(address(this).balance);
    }

    struct MakeCampaignsParam {
        uint256 budget;
        uint256 validUntil;
        uint256 briefing;
    }

    function makeCampaigns(
        MakeCampaignsParam[] calldata _inputs
    ) public nonReentrant {
        uint256 accountId = accountIdByAddress[msg.sender];
        Account storage account = accounts[accountId];
        require(account.addr != address(0), "mc0");

        for (uint i = 0; i < _inputs.length; i++) {
            MakeCampaignsParam memory input = _inputs[i];
            require(input.budget > 0, "mc1");
            require(input.validUntil > block.timestamp, "mc2");
            Campaign storage c = account.campaigns[input.briefing];
            c.budget = input.budget;
            c.validUntil = input.validUntil;
            require(doc.transferFrom(msg.sender, address(this), input.budget), "mc3");

            emit CampaignCreated(accountId, input.briefing);
        }
    }

    struct ExtendCampaignsParam {
        uint256 validUntil;
        uint256 briefing;
    }

    function extendCampaigns(
        ExtendCampaignsParam[] calldata _inputs
    ) public nonReentrant {
        uint256 accountId = accountIdByAddress[msg.sender];
        Account storage account = accounts[accountId];
        require(account.addr != address(0), "ec0");

        for (uint i = 0; i < _inputs.length; i++) {
            ExtendCampaignsParam memory input = _inputs[i];
            require(input.validUntil > 0, "ec1");
            Campaign storage c = account.campaigns[input.briefing];
            require(c.validUntil != 0, "ec2");
            require(input.validUntil > c.validUntil, "ec3");
            c.validUntil = input.validUntil;
            c.reportHash = 0;

            emit CampaignExtended(accountId, input.briefing);
        }
    }

    struct TopUpCampaignsParam {
        uint256 accountId;
        uint256 budget;
        uint256 briefing;
    }

    function topUpCampaigns(
        TopUpCampaignsParam[] calldata _inputs
    ) public nonReentrant {
        for (uint i = 0; i < _inputs.length; i++) {
            TopUpCampaignsParam memory input = _inputs[i];
            require(input.budget > 0, "tc0");

            Account storage account = accounts[input.accountId];
            require(account.addr != address(0), "tc1");

            Campaign storage c = account.campaigns[input.briefing];
            require(c.validUntil > block.timestamp, "tc2");
            c.budget += input.budget;

            require(doc.transferFrom(msg.sender, address(this), input.budget), "tc3");
            emit CampaignToppedUp(input.accountId, input.briefing);
        }
    }

    struct ReimburseCampaignsParam {
        uint256 accountId;
        uint256 briefing;
    }

    function reimburseCampaigns(
        ReimburseCampaignsParam[] calldata _inputs
    ) external nonReentrant {
        for (uint i = 0; i < _inputs.length; i++) {
            ReimburseCampaignsParam memory input = _inputs[i];

            Account storage account = accounts[input.accountId];
            require(account.addr != address(0), "rc1");

            Campaign storage campaign = account.campaigns[input.briefing];
            require(campaign.budget > 0, "rc2");
            require(campaign.validUntil < block.timestamp, "rc3");

            campaign.budget = 0;
            require(doc.transfer(account.addr, campaign.budget), "rdc 4");
            emit CampaignReimbursed(input.accountId, input.briefing);
        }
    }

    struct SubmitReportsParam {
        uint256 accountId;
        uint256 briefing;
        uint256 reportHash;
    }

    function submitReports(
        SubmitReportsParam[] calldata _inputs
    ) external onlyAdmin {
        for (uint i = 0; i < _inputs.length; i++) {
            SubmitReportsParam memory input = _inputs[i];
            require(input.reportHash > 0, "sr0");

            Account storage account = accounts[input.accountId];
            require(account.addr != address(0), "sr1");

            Campaign storage campaign = account.campaigns[input.briefing];
            require(campaign.validUntil > 0, "sr2");
            require(campaign.validUntil < block.timestamp, "sr3");
            require(campaign.reportHash == 0, "sr4");
            campaign.reportHash = input.reportHash;
            emit ReportSubmitted(input.accountId, input.briefing);
        }
    }

    struct MakeCollabsParam {
        uint256 advertiserId;
        uint256 briefing;
        MakeCollabsParamItem[] collabs;
    }

    struct MakeCollabsParamItem {
        uint256 accountId;
        uint256 docReward;
    }

    function adminMakeCollabs(
        MakeCollabsParam[] calldata _input
    ) external onlyAdmin nonReentrant {
        uint256 asamiTokenCap = cap();

        for (uint256 i = 0; i < _input.length; i++) {
          Account storage advertiser = accounts[_input[i].advertiserId];
          Campaign storage campaign = advertiser.campaigns[_input[i].briefing];
          uint256 newAdvertiserTokens = 0;
          uint256 newFees = 0;
          uint256 budget = campaign.budget;
          require(campaign.validUntil > block.timestamp, "amc1");

          for (uint256 j = 0; j < _input[i].collabs.length; j++) {
            MakeCollabsParamItem calldata item = _input[i].collabs[j];
            require(item.accountId > 0, "amc2");

            Account storage member = accounts[item.accountId];

            uint256 gross = item.docReward;
            require(budget >= gross, "amc4");
            budget -= gross;

            uint256 fee = (gross * feeRate) / 1e20;
            uint256 reward = gross - fee;

            uint256 remainingToCap = asamiTokenCap - assignedAsamiTokens;
            uint256 newTokens = (fee > remainingToCap) ? remainingToCap : fee;
            uint256 advertiserTokens = (newTokens * 10 * 1e18) / 100e18;
            uint256 memberTokens = (newTokens * 15 * 1e18) / 100e18;
            uint256 adminTokens = newTokens - advertiserTokens - memberTokens;

            adminUnclaimedAsamiBalance += adminTokens;
            newAdvertiserTokens += advertiserTokens;
            member.unclaimedAsamiBalance += memberTokens;
            member.unclaimedDocBalance += reward;
            assignedAsamiTokens += newTokens;
            newFees += fee;
          }
          changeFeePool(newFees, true);
          campaign.budget = budget;
          advertiser.unclaimedAsamiBalance += newAdvertiserTokens;
      }
    }

    function _safeMint(address _addr, uint256 _amount) private {
        recentTokens[getCurrentCycle()] += _amount;
        _mint(_addr, _amount);
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

    /* Accounts with unclaimed DOC rewards always have an option to have their funds sent by the admin paying for the transaction fees. */
    uint256 public adminUnclaimedDoc;
    uint256 public adminUnclaimedTokens;
    uint256 public adminGaslessClaimRewardPrice;
    uint256 public adminGaslessClaimAccountPrice;

    uint256 public feePool;

    struct BalanceChange {
      uint256 added;
      uint256 substracted;
    }
    mapping (uint256 => mapping(address => BalanceChange)) public recentBalanceChanges;
    mapping (uint256 => BalanceChange) public recentFeePoolChanges;
    mapping (uint256 => uint256) public recentTokens;
    mapping(address => uint256) public lastFeePoolShareCycles;

    function getRecentBalanceChange(address _holder) public view returns (BalanceChange memory) {
      return recentBalanceChanges[getCurrentCycle()][_holder];
    }

    function getBalanceBeforeRecentChanges(address _holder) public view returns (uint256) {
      BalanceChange storage recent = recentBalanceChanges[getCurrentCycle()][_holder];
      return (balanceOf(_holder) + recent.substracted) - recent.added;
    }

    function _registerRecentBalanceChange(address _holder, uint256 _amount, bool _added) private {
      BalanceChange storage recent = recentBalanceChanges[getCurrentCycle()][_holder];
      if(_added){
        recent.added += _amount;
      } else {
        recent.substracted += _amount;
      }
    }

    function changeFeePool(uint256 _amount, bool _added) private {
      BalanceChange storage recent = recentFeePoolChanges[getCurrentCycle()];
      if(_added){
        feePool += _amount;
        recent.added += _amount;
      } else {
        feePool -= _amount;
        recent.substracted += _amount;
      }
    }

    function getFeePoolBeforeRecentChanges() public view returns (uint256) {
      BalanceChange storage recent = recentFeePoolChanges[getCurrentCycle()];
      return (feePool + recent.substracted) - recent.added;
    }

    function claimFeePoolShare(address[] calldata _holders) external nonReentrant {
        uint256 current = getCurrentCycle();
        uint256 supply = totalSupply() - recentTokens[current];
        uint256 pool = getFeePoolBeforeRecentChanges();
        require(supply > 0, "cfp0");
        require(pool > 0, "cfp1");

        for (uint256 i = 0; i < _holders.length; i++) {
          address holder = _holders[i];

          require(lastFeePoolShareCycles[holder] < current, "cfp2");

          uint256 balance = getBalanceBeforeRecentChanges(holder);
          require(balance > 0, "cfp3");
          uint256 reward = (balance * pool) / supply;

          lastFeePoolShareCycles[holder] = current;
          changeFeePool(reward, false);

          require(doc.transfer(holder, reward), "cfp4");
        }
    }

    function applyVotedFeeRate() external {
        uint256 currentCycle = getCurrentCycle();
        require(currentCycle != lastVotedFeeRateAppliedOn, "afr0");
        feeRate = votedFeeRate;
        lastVotedFeeRateAppliedOn = currentCycle;
    }

    function submitFeeRateVote(uint256 _rate) external {
        require(_rate > 0 && _rate < 1e20, "srv0");

        FeeRateVote storage vote = feeRateVotes[msg.sender];
        if (vote.votes > 0) {
            removeFeeRateVoteHelper(msg.sender);
        }

        uint256 votes = balanceOf(msg.sender);
        require(votes > 0, "srv1");

        votedFeeRate =
            ((votedFeeRate * votedFeeRateVoteCount) + (votes * _rate)) /
            (votedFeeRateVoteCount + votes);
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
        uint256 nextVolume = currentVolume > removedVolume
            ? currentVolume - removedVolume
            : 0;

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
        require(_candidate != address(0), "sav0");
        uint256 votes = balanceOf(msg.sender);
        require(votes > 0, "sav1");

        AdminVote storage existing = submittedAdminVotes[msg.sender];
        if (existing.votes > 0) {
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
                adminVoteCountIdByCandidate[vote.candidate] = adminVoteCounts
                    .length;
            }
            emit AdminVoteSaved(vote);
        }
    }

    function proclaimCycleAdminWinner(address _candidate) external {
        uint256 voteCountId = adminVoteCountIdByCandidate[_candidate];
        require(voteCountId > 0, "pcw0");
        AdminVoteCount storage count = adminVoteCounts[voteCountId - 1];

        require(vestedAdminVotesTotal > 0, "pcw1");
        require(count.votes > vestedAdminVotesTotal / 2, "pcw2");
        uint256 thisCycle = getCurrentCycle();
        require(lastAdminElection < thisCycle, "pcw3");

        lastAdminElection = thisCycle;
        latestAdminElections[2] = latestAdminElections[1];
        latestAdminElections[1] = latestAdminElections[0];
        latestAdminElections[0] = _candidate;

        if (
            _candidate == latestAdminElections[1] &&
            _candidate == latestAdminElections[2]
        ) {
            admin = _candidate;
            adminTreasury = _candidate;
        }
    }

    function removeAdminVote() external {
        removeAdminVoteHelper(msg.sender);
    }

    function removeAdminVoteHelper(address _holder) private {
        AdminVote storage vote = submittedAdminVotes[_holder];
        require(vote.votes > 0, "ravh0");
        if (vote.vested) {
            vestedAdminVotesTotal -= vote.votes;

            uint256 voteCountId = adminVoteCountIdByCandidate[vote.candidate];
            require(voteCountId > 0);
            adminVoteCounts[voteCountId - 1].votes -= vote.votes;
        }
        delete submittedAdminVotes[_holder];
    }

    function setAdminAddress(address _admin) external {
        require(msg.sender == adminTreasury, "saa0");
        admin = _admin;
    }

    uint256 public migrationCursor;
    bool public migrationHoldersDone;
    bool public migrationAccountsDone;

    function migrateTokensFromOldContract(address _oldContract, uint256 _items) external {
      require(!migrationHoldersDone || !migrationAccountsDone, "mig0");
      Asami oldContract = Asami(_oldContract);

      if (!migrationHoldersDone) {
        for (uint256 i = 0; i < _items; i++) {
          try oldContract.holders(migrationCursor+i) returns (address holder) {
            uint256 balance = oldContract.balanceOf(holder);
            if(balance > 0){
              _safeMint(holder, oldContract.balanceOf(holder));
            }
          } catch {
              migrationHoldersDone = true;
              break;
          }
        }
      }

      if (!migrationAccountsDone) {
        for (uint256 i = 0; i < _items; i++) {
          try oldContract.accountIds(migrationCursor + i) returns (uint256 oldAccountId) {
            ( , address addr, uint256 oldUnclaimedAsami,) = oldContract.accounts(oldAccountId);
            Account storage account = accounts[oldAccountId];
            if(addr != address(0)) {
              accountIdByAddress[addr] = oldAccountId;
              account.addr = addr;
              account.approvedGaslessAmount = 5e18; 
            } else {
              account.unclaimedAsamiBalance = oldUnclaimedAsami;
            }
          } catch {
              migrationAccountsDone = true;
              break;
          }
        }
      }

      migrationCursor += _items;
    }
}
