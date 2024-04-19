// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Capped.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "./Asami.sol";

contract AsamiCore is ERC20Capped, ReentrancyGuard {
    /* 
      Contract tracks time in 14 day cycles from the moment it was deployed.
      This cycle marks how often new voted fee rates are applied, how often revenue sharing happens,
      and the new asami token issuance.
    */
    uint256 initialCycle;
    uint256 cycleLength = 60 * 60 * 24 * 15;

    function getCurrentCycle() public view returns (uint256) {
        return (block.timestamp / cycleLength) - initialCycle;
    }

    struct Account {
      /* 
        The trusted admin is used to register collaborations in the campaigns this account creates.
        Accounts with to trusted admin cannot run campaigns.

        It can also perform gasless claims.
        Accounts with no trusted admin can have their gasless claims executed by anyone.
      */
      address trustedAdmin;
      uint256 maxGaslessDocToSpend;
      uint256 minGaslessRbtcToReceive;
      uint256 unclaimedAsamiBalance;
      uint256 unclaimedDocBalance;
      uint256 feeRateWhenAdmin;
      FeeRateVote feeRateVote;
      mapping( uint256 => Campaign ) campaigns;
      mapping( uint256 => SubAccount ) subAccounts;
    }
    mapping(address => Account) public accounts;
    
    /*
      Sub accounts can be used by someone acting as an admin to collect rewards on behalf
      of someone who does not have an account yet, usually a web2 user.
      Sub accounts can then be promoted to accounts by the admin.
    */
    struct SubAccount {
      uint256 unclaimedAsamiBalance;
      uint256 unclaimedDocBalance;
    }

    function getSubAccount(address _account, uint256 _index) public view returns (SubAccount memory) {
      return accounts[_account][_index];
    }

    struct Campaign {
      uint256 budget;
      uint256 validUntil;
      uint256 reportHash;
    }

    function getCampaign(address _account, uint256 _briefingHash) public view returns (Campaign memory) {
      return accounts[_account].campaigns[_briefingHash];
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

    IERC20 internal doc;
    uint256 public assignedAsamiTokens;

    event AccountUpdated(uint256 accountId);
    event CampaignCreated(uint256 accountId, uint256 campaignId);
    event CampaignSaved(uint256 accountId, uint256 campaignId);
    event CampaignExtended(uint256 accountId, uint256 campaignId);
    event CampaignToppedUp(uint256 accountId, uint256 campaignId);
    event CampaignReimbursed(uint256 accountId, uint256 campaignId);
    event ReportSubmitted(uint256 accountId, uint256 campaignId);
    event CollabSaved(uint256 advertiserId, uint256 briefingHash, uint256 accountId, uint256 gross, uint256 fee);

    constructor(
        address _dollarOnChainAddress,
        address _adminAddress
    ) ERC20("Asami Core", "ASAMI") ERC20Capped(21 * 1e24) {
        doc = IERC20(_dollarOnChainAddress);
        adminTreasury = msg.sender;
        initialCycle = block.timestamp / cycleLength;
    }

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override {
        super._beforeTokenTransfer(from, to, amount);
        if (from != address(0)) {
          _registerRecentBalanceChange(from, amount, false);

          if (accounts[from].feeRateVote.votes > 0) {
              removeFeeRateVoteHelper(from);
          }
        }

        if (to != address(0)) {
          _registerRecentBalanceChange(to, amount, true);
        }
    }

    struct PromoteSubAccountsParam {
        uint256 id;
        address addr;
    }

    /* Admin that had some sub-accounts can now promote those */
    function promoteSubAccounts(PromoteSubAccountsParam[] _params) external nonReentrant {
      for(uint256 i = 0; i < _params.length; i++) {
        PromoteSubAccountsParam calldata param = _params[i];
        SubAccount storage sub = accounts[msg.sender].subAccounts[param.id];
        Account storage account = accounts[param.addr];
        account.unclaimedAsamiBalance += sub.unclaimedAsamiBalance;
        sub.unclaimedAsamiBalance = 0;
        account.unclaimedDocBalance += sub.unclaimedDocBalance;
        sub.unclaimedDocBalance = 0;
      }
    }

    function configAccount(address _trustedAdmin, uint256 _maxGaslessDocToSpend, uint256 _minGaslessRbtcToReceive, uint256 _feeRateWhenAdmin) external {
      Account storage account = accounts[msg.sender];
      account.trustedAdmin = _trustedAdmin;
      account.maxGaslessDocToSpend = _maxGaslessDocToSpend;
      account.minGaslessDocToReceive = _minGaslessRbtcToReceive;
      account.feeRateWhenAdmin = _feeRateWhenAdmin;
    }

    function claimBalances() external nonReentrant {
      Account storage account = accounts[msg.sender];

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

    function adminClaimBalancesFree(address[] calldata _addresses) external nonReentrant {
      for (uint256 i = 0; i < _addresses.length; i++){
        Account storage account = accounts[_addresses[i]];
        require(account.trustedAdmin != msg.sender || account.trustedAdmin == address(0), "acb0");

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

    function gaslessClaimBalances(
      uint256 _fee,
      uint256 _rbtcAmount,
      address[] calldata _addresses
    ) external payable nonReentrant {
      require(_rbtcAmount > 1e11, "gcb0");
      require(msg.value == (_rbtcAmount * _addresses.length), "gcb1");

      for (uint256 i = 0; i < _addresses.length; i++){
        Account storage account = accounts[_addresses[i]];
        require(account.trustedAdmin == msg.sender || account.trustedAdmin == address(0), "gcb2");
        require(account.unclaimedDocBalance > _fee, "gcb3");
        require(account.maxGaslessDocToSpend >= _fee, "gcb4");
        require(_rbtcAmount >= account.minGaslessRbtcToReceive, "gcb5");

        uint256 docs = account.unclaimedDocBalance - _fee;
        account.unclaimedDocBalance = 0;
        doc.transfer(account.addr, docs);
        accounts[msg.sender].unclaimedDocBalance += _fee;
        payable(account.addr).transfer(_rbtcAmount);

        if (account.unclaimedAsamiBalance > 0) {
          uint256 balance = account.unclaimedAsamiBalance;
          account.unclaimedAsamiBalance = 0;
          _safeMint(account.addr, balance);
        }
      }
    }

    struct MakeCampaignsParam {
        uint256 budget;
        uint256 validUntil;
        uint256 briefingHash;
    }

    function setTrustedAdminAndMakeCampaign(
        address _trustedAdmin,
        MakeCampaignsParam[] calldata _inputs
    ) public nonReentrant {
        Account storage account = accounts[msg.sender];
        account.trustedAdmin = _trustedAdmin;
        makeCampaigns(_inputs);
    }

    function makeCampaigns(
        MakeCampaignsParam[] calldata _inputs
    ) public nonReentrant {
        Account storage account = accounts[msg.sender];

        for (uint i = 0; i < _inputs.length; i++) {
            MakeCampaignsParam memory input = _inputs[i];
            require(input.budget > 0, "mc0");
            require(input.validUntil > block.timestamp, "mc1");
            Campaign storage c = account.campaigns[input.briefingHash];
            c.budget = input.budget;
            c.validUntil = input.validUntil;
            require(doc.transferFrom(msg.sender, address(this), input.budget), "mc2");

            emit CampaignCreated(msg.sender, input.briefingHash);
        }
    }

    struct ExtendCampaignsParam {
        uint256 validUntil;
        uint256 briefingHash;
    }

    function extendCampaigns(
        ExtendCampaignsParam[] calldata _inputs
    ) public nonReentrant {
        Account storage account = accounts[msg.sender];
        require(account.addr != address(0), "ec0");

        for (uint i = 0; i < _inputs.length; i++) {
            ExtendCampaignsParam memory input = _inputs[i];
            require(input.validUntil > 0, "ec1");
            Campaign storage c = account.campaigns[input.briefingHash];
            require(c.validUntil != 0, "ec2");
            require(input.validUntil > c.validUntil, "ec3");
            c.validUntil = input.validUntil;
            c.reportHash = 0;

            emit CampaignExtended(msg.sender, input.briefingHash);
        }
    }

    struct TopUpCampaignsParam {
        address account;
        uint256 budget;
        uint256 briefingHash;
    }

    function topUpCampaigns(
        TopUpCampaignsParam[] calldata _inputs
    ) public nonReentrant {
        for (uint i = 0; i < _inputs.length; i++) {
            TopUpCampaignsParam memory input = _inputs[i];
            require(input.budget > 0, "tc0");
            require(input.account != address(0), "tc1");

            Account storage account = accounts[input.account];
            Campaign storage c = account.campaigns[input.briefingHash];

            require(c.validUntil > block.timestamp, "tc2");

            c.budget += input.budget;

            require(doc.transferFrom(msg.sender, address(this), input.budget), "tc3");
            emit CampaignToppedUp(input.account, input.briefingHash);
        }
    }

    struct ReimburseCampaignsParam {
        address account;
        uint256 briefingHash;
    }

    function reimburseCampaigns(
        ReimburseCampaignsParam[] calldata _inputs
    ) external nonReentrant {
        for (uint i = 0; i < _inputs.length; i++) {
            ReimburseCampaignsParam memory input = _inputs[i];

            Account storage account = accounts[input.account];
            Campaign storage campaign = account.campaigns[input.briefingHash];
            require(campaign.budget > 0, "rc1");
            require(campaign.validUntil < block.timestamp, "rc2");

            campaign.budget = 0;
            require(doc.transfer(account, campaign.budget), "rdc 3");
            emit CampaignReimbursed(input.account, input.briefingHash);
        }
    }

    struct SubmitReportsParam {
        address account;
        uint256 briefingHash;
        uint256 reportHash;
    }

    function submitReports(
        SubmitReportsParam[] calldata _inputs
    ) external {
        for (uint i = 0; i < _inputs.length; i++) {
            SubmitReportsParam memory input = _inputs[i];
            require(input.reportHash > 0, "sr0");

            Account storage account = accounts[input.account];
            require(account.trustedAdmin == msg.sender, "sr1");

            Campaign storage campaign = account.campaigns[input.briefingHash];
            require(campaign.validUntil > 0, "sr2");
            require(campaign.validUntil < block.timestamp, "sr3");
            require(campaign.reportHash == 0, "sr4");
            campaign.reportHash = input.reportHash;
            emit ReportSubmitted(input.account, input.briefingHash);
        }
    }

    struct MakeCollabsParam {
        address advertiser;
        uint256 briefingHash;
        MakeCollabsParamItem[] collabs;
    }

    struct MakeCollabsParamItem {
        address account;
        uint256 subAccountId;
        uint256 docReward;
    }

    /* Make sure all the accounts involved are marked as claimed (?) */
    function adminMakeCollabs(
        MakeCollabsParam[] calldata _input
    ) external nonReentrant {
        uint256 asamiTokenCap = cap();
        uint256 current = getCurrentCycle();
        Account storage admin = accounts[msg.sender];
        if( !admin.claimed ){ admin.claimed = true; }

        /* It's impossible to make collabs if the fees would exceed the reward */
        require((feeRate + admin.feeRateWhenAdmin) < 100e18, "amc0");

        for (uint256 i = 0; i < _input.length; i++) {
          Account storage advertiser = accounts[_input[i].advertiserId];
          Campaign storage campaign = advertiser.campaigns[_input[i].briefingHash];
          require(advertiser.trustedAdmin == msg.sender, "amc1");

          uint256 newAdvertiserTokens = 0;
          uint256 newFees = 0;
          uint256 newAdminFees = 0;
          uint256 newAdminTokens = 0;
          uint256 reducedBudget = campaign.budget;
          require(campaign.validUntil > block.timestamp, "amc2");

          for (uint256 j = 0; j < _input[i].collabs.length; j++) {
            MakeCollabsParamItem calldata item = _input[i].collabs[j];
            require(item.accountId > 0, "amc3");

            Account storage member = accounts[item.accountId];

            uint256 gross = item.docReward;
            require(reducedBudget >= gross, "amc4");
            reducedBudget -= gross;

            uint256 fee = (gross * feeRate) / 1e20;
            uint256 adminFee = (gross * admin.feeRateWhenAdmin) / 1e20;
            uint256 reward = gross - fee - adminFee;

            uint256 remainingToCap = asamiTokenCap - assignedAsamiTokens;

            if (remainingToCap > 0) {
              uint256 tokensAtRate = fee * getIssuanceRate();
              uint256 newTokens = (fee > remainingToCap) ? remainingToCap : fee;
              uint256 advertiserTokens = (newTokens * 30 * 1e18) / 100e18;
              uint256 memberTokens = (newTokens * 30 * 1e18) / 100e18;
              uint256 adminTokens = newTokens - advertiserTokens - memberTokens;

              if(newTokens > 0) {
                newAdminTokens += adminTokens;
                newAdvertiserTokens += advertiserTokens;
                if (item.account == address(0)) {
                  admin.subAccounts[item.subAccountId].unclaimedAsamiTokens += memberTokens;
                } else {
                  accounts[item.account].unclaimedAsamiTokens += memberTokens;
                }

                assignedAsamiTokens += newTokens; /* TODO: Test gas usage for a local variable instead */
              }
            }
            
            member.unclaimedDocBalance += reward;
            if (item.account == address(0)) {
              admin.subAccounts[item.subAccountId].unclaimedDocBalance += reward;
            } else {
              accounts[item.account].unclaimedDocBalance += reward;
            }
            newFees += fee;
            newAdminFees += adminFee;
          }

          /* To save on gas costs, values are added in memory then stored at once */
          /* TODO: measure gas cost for just updating the admin.unclaimedDocBalance */
          changeFeePool(newFees, true);
          admin.unclaimedDocBalance += newAdminFees;
          admin.unclaimedAsamiBalance += newAdminTokens;
          advertiser.unclaimedAsamiBalance += newAdvertiserTokens;

          /* 
            Storing the fee amount collected in this cycle is only
            useful to adjust the issuance rate on the next cycle.
            If we've issued al tokens, there's no point in adjusting the
            issuance rate on next cycle, therefore, no point in storing this value
          */
          if(assignedAsamiTokens < asamiTokenCap) {
            feesCollectedPerPeriodDuringTokenIssuance[current] += newFees;
          }

          campaign.budget = reducedBudget;
      }
    }

    function _safeMint(address _addr, uint256 _amount) private {
        recentTokens[getCurrentCycle()] += _amount;
        _mint(_addr, _amount);
    }

    /* 
      The feePool accumulates all the protocol fees to be distributed at the end of each cycle.
      The feePool is distributed based on holder balances at the end of the cycle.
    */
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
        require(_rate > 0 && _rate < 100e18, "srv0");

        FeeRateVote storage vote = accounts[msg.sender].feeRateVote;
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
        FeeRateVote storage vote = accounts[_holder].feeRateVote;
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
        delete accounts[_holder].feeRateVote;
    }

    /* Asami token issuance rate:
      The issuance goal is 100.000 tokens every 15 days until reaching 21.000.000 (a little above 1 year).
      That is, tokens will be issued in 210 cycles.
      Tokens are assigned to people paying into the fees pool.
      For every cycle we calculate the previous 10 cycles average fees, and come up with an equivalence of DoC paid vs tokens awarded.
      This equivalence is stored and updated for every cycle, and should adjust issuance to target 100.000 tokens every cycle.
    */
    uint256 public tokensTargetPerPeriod = 100000;
    uint256 public tokensToIssuePerDoc = 4000;
    mapping(uint256 => uint256) feesCollectedPerPeriodDuringTokenIssuance;

    /* 
       The amount of tokens to issue for a given number of DOC collected as fee.
       According to last period's collected fees and the tokens target per period.
    */
    function getIssuanceRate() public view returns (uint256) {
      uint256 current = getCurrentCycle();
      /* The initial value is roughly based on the fees collected by V1 during it's lifetime */
      if (current == 0) {
        return 4000;
      }
      return tokensTargetPerPeriod / feesCollectedPerPeriodDuringTokenIssuance[current - 1];
    }

    /* The adminTreasury address is the deployer address which can only be used to retrieve accidentally sent RBTC into the contract. */
    address adminTreasury;

    function withdrawAccidentallySentRbtc() external {
      require(msg.sender == adminTreasury, "wasr0");
      payable(adminTreasury).transfer(address(this).balance);
    }

    /* 
       This is the migration from the old contract.
       It only issues new asami tokens to these users again,
       based on the tokens they had in the previous version of the contract.
    */
    uint256 public migrationCursor;
    bool public migrationHoldersDone;
    bool public migrationAccountsDone;

    function migrateTokensFromOldContract(address _oldContract, address _admin, uint256 _items) external {
      require(!migrationHoldersDone || !migrationAccountsDone, "mig0");
      Asami oldContract = Asami(_oldContract);

      assignedAsamiTokens = oldContract.claimedAsamiTokens;
      Account storage admin = accounts[_admin];

      if (!migrationHoldersDone) {
        for (uint256 i = 0; i < _items; i++) {
          try oldContract.holders(migrationCursor+i) returns (address holder) {
            uint256 balance = oldContract.balanceOf(holder) * getIssuanceRate();
            if(balance > 0){
              _safeMint(holder, balance);
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
            if(addr == address(0) && oldUnclaimedAsami > 0) {
              accounts[addr].unclaimedAsamiBalance = oldUnclaimedAsami;
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
