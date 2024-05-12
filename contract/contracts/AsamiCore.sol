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
      return accounts[_account].subAccounts[_index];
    }

    struct Campaign {
      uint256 budget;
      uint256 validUntil;
      uint256 reportHash;
    }

    function getCampaign(address _account, uint256 _briefingHash) public view returns (Campaign memory) {
      return accounts[_account].campaigns[_briefingHash];
    }

    /* To make the protocol more predictable, changes are applied only once every cycle */
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

    event CampaignCreated(address account, uint256 campaignId);
    event CampaignExtended(address account, uint256 campaignId);
    event CampaignToppedUp(address account, uint256 campaignId);
    event CampaignReimbursed(address account, uint256 campaignId);

    constructor(
        address _dollarOnChainAddress
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

		// Anyone acting as an Admin can onboard web2 users as sub-accounts.
		// The admin still has full control of the subaccount funds until giving them away.
		// This function is only for convenience, it could easily be replaced by a custom admin
		// process that uses this function to send sub-account funds to the admin's own address,
		// and then decides how much to forward to the claiming user.
    function promoteSubAccounts(PromoteSubAccountsParam[] calldata _params) external nonReentrant {
      for(uint256 i = 0; i < _params.length; i++) {
        PromoteSubAccountsParam calldata param = _params[i];
        SubAccount storage sub = accounts[msg.sender].subAccounts[param.id];
        Account storage account = accounts[param.addr];
        require(account.trustedAdmin == msg.sender || account.trustedAdmin == address(0), "psa0");

        account.unclaimedAsamiBalance += sub.unclaimedAsamiBalance;
        sub.unclaimedAsamiBalance = 0;

				uint256 promotedDocAmount = sub.unclaimedDocBalance;
        account.unclaimedDocBalance += sub.unclaimedDocBalance;
        sub.unclaimedDocBalance = 0;

				// Accounts not claimed by anyone yet can be claimed by any admin.
				// The TrustedAdmin has no permissions other than for funding the account payouts.
				// The account holder can always configure this at will after getting some RBTC.
				if (account.trustedAdmin == address(0)) {
					account.trustedAdmin = msg.sender;
					// The amount left in the sub account is the highest amount we know the 
					// user is comfortable leaving to the admin's best judgement.
					account.maxGaslessDocToSpend = promotedDocAmount;

					// The admin must send at least enough RBTC for 100.000 gas.
					// 100.000 gas is more than enough for the user to reconfigure their account
					// selecting another admin and reducing these numbers.
					// Also to send their DOC and spend them somewhere.
					account.minGaslessRbtcToReceive = 6e12;
				}
      }
    }

    function configAccount(address _trustedAdmin, uint256 _maxGaslessDocToSpend, uint256 _minGaslessRbtcToReceive, uint256 _feeRateWhenAdmin) external {
      Account storage account = accounts[msg.sender];
      account.trustedAdmin = _trustedAdmin;
      account.maxGaslessDocToSpend = _maxGaslessDocToSpend;
      account.minGaslessRbtcToReceive = _minGaslessRbtcToReceive;
      account.feeRateWhenAdmin = _feeRateWhenAdmin;
    }

    function claimBalances() external nonReentrant {
      Account storage account = accounts[msg.sender];

      if (account.unclaimedAsamiBalance > 0) {
        uint256 balance = account.unclaimedAsamiBalance;
        account.unclaimedAsamiBalance = 0;
        _safeMint(msg.sender, balance);
      }

      if (account.unclaimedDocBalance > 0) {
        uint256 balance = account.unclaimedDocBalance;
        account.unclaimedDocBalance = 0;
        doc.transfer(msg.sender, balance);
      }
    }

    function adminClaimBalancesFree(address[] calldata _addresses) external nonReentrant {
      for (uint256 i = 0; i < _addresses.length; i++){
        Account storage account = accounts[_addresses[i]];
        require(account.trustedAdmin == msg.sender, "acb0");

        if (account.unclaimedAsamiBalance > 0) {
          uint256 balance = account.unclaimedAsamiBalance;
          account.unclaimedAsamiBalance = 0;
          _safeMint(_addresses[i], balance);
        }

        if (account.unclaimedDocBalance > 0) {
          uint256 balance = account.unclaimedDocBalance;
          account.unclaimedDocBalance = 0;
          doc.transfer(_addresses[i], balance);
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
        doc.transfer(_addresses[i], docs);
        accounts[msg.sender].unclaimedDocBalance += _fee;
        payable(_addresses[i]).transfer(_rbtcAmount);

        if (account.unclaimedAsamiBalance > 0) {
          uint256 balance = account.unclaimedAsamiBalance;
          account.unclaimedAsamiBalance = 0;
          _safeMint(_addresses[i], balance);
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

    /* A campaign's expiration can be extended, and this will mean that
       people will be able to participate for longer, but also that the funds
       locked in it will be held longer.
       That's why only the advertiser can call this, knowing their money will be held longer.
    */
    struct ExtendCampaignsParam {
        uint256 validUntil;
        uint256 briefingHash;
    }

    function extendCampaigns(
        ExtendCampaignsParam[] calldata _inputs
    ) public nonReentrant {
        Account storage account = accounts[msg.sender];

        for (uint i = 0; i < _inputs.length; i++) {
            ExtendCampaignsParam memory input = _inputs[i];
            Campaign storage c = account.campaigns[input.briefingHash];
            require(c.validUntil != 0, "ec1");
            require(input.validUntil > c.validUntil, "ec2");
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
        address addr;
        uint256 briefingHash;
    }

    function reimburseCampaigns(
        ReimburseCampaignsParam[] calldata _inputs
    ) external nonReentrant {
        for (uint i = 0; i < _inputs.length; i++) {
            ReimburseCampaignsParam memory input = _inputs[i];

            Account storage account = accounts[input.addr];
            Campaign storage campaign = account.campaigns[input.briefingHash];
            require(campaign.budget > 0, "rc0");
            require(campaign.validUntil < block.timestamp, "rc1");

            campaign.budget = 0;
            require(doc.transfer(input.addr, campaign.budget), "rc2");
            emit CampaignReimbursed(input.addr, input.briefingHash);
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
        }
    }

    struct MakeCollabsParam {
        address advertiserAddr;
        uint256 briefingHash;
        MakeCollabsParamItem[] collabs;
    }

    struct MakeCollabsParamItem {
        address accountAddr;
        uint256 docReward;
    }

    /* Make sure all the accounts involved are marked as claimed (?) */
    function adminMakeCollabs(
        MakeCollabsParam[] calldata _input
    ) external nonReentrant {
        /* It's impossible to make collabs if the fees would exceed the reward */
        require((feeRate + accounts[msg.sender].feeRateWhenAdmin) < 100e18, "amc0");

        for (uint256 i = 0; i < _input.length; i++) {
          Account storage advertiser = accounts[_input[i].advertiserAddr];
          Campaign storage campaign = advertiser.campaigns[_input[i].briefingHash];
          require(advertiser.trustedAdmin == msg.sender, "amc1");
          require(campaign.validUntil > block.timestamp, "amc2");

          uint256 newFees = 0;

          for (uint256 j = 0; j < _input[i].collabs.length; j++) {
            Account storage admin = accounts[msg.sender];
            MakeCollabsParamItem calldata item = _input[i].collabs[j];
            require(item.accountAddr != address(0), "amc3");

            uint256 gross = item.docReward;
            require(campaign.budget >= gross, "amc4");
            campaign.budget -= gross;

            uint256 fee = (gross * feeRate) / 1e20;
            uint256 adminFee = (gross * admin.feeRateWhenAdmin) / 1e20;
            uint256 reward = gross - fee - adminFee;

            uint256 memberTokens = _tryAssigningTokensAndReturnMemberTokens(fee, advertiser, admin);
            if(memberTokens > 0 ) {
               accounts[item.accountAddr].unclaimedAsamiBalance += memberTokens;
            }
            accounts[item.accountAddr].unclaimedDocBalance += reward;
            newFees += fee;
            admin.unclaimedDocBalance += adminFee;
          }

          /* To save on gas costs, values are added in memory then stored at once */
          changeFeePool(newFees, true);

          /* 
            Storing the fee amount collected in this cycle is only
            useful to adjust the issuance rate on the next cycle.
            If we've issued al tokens, there's no point in adjusting the
            issuance rate on next cycle, therefore, no point in storing this value
          */
          if(assignedAsamiTokens < cap()) {
            feesCollectedPerCycleDuringTokenIssuance[getCurrentCycle()] += newFees;
          }
      }
    }

    struct MakeSubAccountCollabsParam {
        address advertiserAddr;
        uint256 briefingHash;
        MakeSubAccountCollabsParamItem[] collabs;
    }

    struct MakeSubAccountCollabsParamItem {
        uint256 subAccountId;
        uint256 docReward;
    }


    /* Make sure all the accounts involved are marked as claimed (?) */
    function adminMakeSubAccountCollabs(
        MakeSubAccountCollabsParam[] calldata _input
    ) external nonReentrant {
        /* It's impossible to make collabs if the fees would exceed the reward */
        require((feeRate + accounts[msg.sender].feeRateWhenAdmin) < 100e18, "amc0");

        for (uint256 i = 0; i < _input.length; i++) {
          Account storage advertiser = accounts[_input[i].advertiserAddr];
          Campaign storage campaign = advertiser.campaigns[_input[i].briefingHash];
          require(advertiser.trustedAdmin == msg.sender, "amc1");
          require(campaign.validUntil > block.timestamp, "amc2");

          uint256 newFees = 0;

          for (uint256 j = 0; j < _input[i].collabs.length; j++) {
            Account storage admin = accounts[msg.sender];
            MakeSubAccountCollabsParamItem calldata item = _input[i].collabs[j];

            uint256 gross = item.docReward;
            require(campaign.budget >= gross, "amc4");
            campaign.budget -= gross;

            uint256 fee = (gross * feeRate) / 1e20;
            uint256 adminFee = (gross * admin.feeRateWhenAdmin) / 1e20;
            uint256 reward = gross - fee - adminFee;

            uint256 memberTokens = _tryAssigningTokensAndReturnMemberTokens(fee, advertiser, admin);
            if(memberTokens > 0 ) {
               admin.subAccounts[item.subAccountId].unclaimedAsamiBalance += memberTokens;
            }
            admin.subAccounts[item.subAccountId].unclaimedDocBalance += reward;
            newFees += fee;
            admin.unclaimedDocBalance += adminFee;
          }

          /* To save on gas costs, values are added in memory then stored at once */
          changeFeePool(newFees, true);

          /* 
            Storing the fee amount collected in this cycle is only
            useful to adjust the issuance rate on the next cycle.
            If we've issued al tokens, there's no point in adjusting the
            issuance rate on next cycle, therefore, no point in storing this value
          */
          if(assignedAsamiTokens < cap()) {
            feesCollectedPerCycleDuringTokenIssuance[getCurrentCycle()] += newFees;
          }
      }
    }

    function _tryAssigningTokensAndReturnMemberTokens(
      uint256 _fee,
      Account storage _advertiser,
      Account storage _admin
    ) private returns (uint256) {
      uint256 remainingToCap = cap() - assignedAsamiTokens;

      if (remainingToCap > 0) {
        uint256 tokensAtRate = getIssuanceFor(_fee);
        uint256 newTokens = (tokensAtRate > remainingToCap) ? remainingToCap : tokensAtRate;
        uint256 advertiserTokens = (newTokens * 30 * 1e18) / 100e18;
        uint256 memberTokens = (newTokens * 30 * 1e18) / 100e18;

        if(newTokens > 0) {
          _advertiser.unclaimedAsamiBalance += advertiserTokens;
          _admin.unclaimedAsamiBalance += newTokens - advertiserTokens - memberTokens;
          assignedAsamiTokens += newTokens; 
          return memberTokens;
        }
      }

      return 0;
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
    uint256 public tokensTargetPerCycle = 100000e18;
    mapping(uint256 => uint256) public feesCollectedPerCycleDuringTokenIssuance;

    // The amount of tokens to issue for a given number of DOC collected as fee.
    //  According to last cycle's collected fees and the tokens target per cycle.
    function getIssuanceFor(uint256 _docAmount) public view returns (uint256) {
			uint256 current = getCurrentCycle();

      /* The initial value is roughly based on the fees collected by V1 during it's lifetime */
      if (current == 0) {
        return _docAmount * 4000;
      }

			// If for whatever reason we have a cycle in which we didn't collect fees,
			// we issue tokens like we had collected 100 DOC in fees for that period.
			// This is an intentionally low amount mostly to avoid dividing by zero.
			uint256 prev = feesCollectedPerCycleDuringTokenIssuance[current - 1];
      return (_docAmount * tokensTargetPerCycle) / (prev == 0 ? 100e18 : prev );
    }

    function getFeesCollected(uint256 _cycle) public view returns (uint256) {
			return feesCollectedPerCycleDuringTokenIssuance[_cycle];
		}

    /* The adminTreasury address is the deployer address which can only be used to retrieve accidentally sent RBTC into the contract. */
    address public adminTreasury;

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

    function migrateTokensFromOldContract(address _oldContract, address _adminAddr, uint256 _items) external {
      require(!migrationHoldersDone || !migrationAccountsDone, "mig0");
      Asami oldContract = Asami(_oldContract);
      assignedAsamiTokens = oldContract.claimedAsamiTokens();
      Account storage admin = accounts[_adminAddr];

      if (!migrationHoldersDone) {
        for (uint256 i = 0; i < _items; i++) {
          try oldContract.holders(migrationCursor+i) returns (address holder) {
            uint256 balance = getIssuanceFor(oldContract.balanceOf(holder));
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
              admin.subAccounts[oldAccountId].unclaimedAsamiBalance = getIssuanceFor(oldUnclaimedAsami);
            }
						if(addr != address(0)){
							// These values are set only once during the migration, for accounts
							// that existed in the previous contract.
							// This ensures a smooth migration and that the admin is immediately able
							// to manage these accounts, provinding gasless claims at reasonable prices.
							accounts[addr].trustedAdmin = _adminAddr;
							accounts[addr].maxGaslessDocToSpend = 1e18;
							accounts[addr].minGaslessRbtcToReceive = 6e12;
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
