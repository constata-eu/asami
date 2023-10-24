// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;
import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
//import "./AsamiClassic.sol";
//import "./AsamiNostr.sol";

contract Asami is Ownable {
  IERC20 internal rewardToken;
  
  address admin;

  string[] public topics;

  mapping(uint256 => Account) public accounts;
  uint256[] public accountIds;
  mapping(address => Account) public accountByAddress;

  PendingPriceUpdate[] pendingPriceUpdates;

  struct PendingPriceUpdate {
    uint256 accountId;
    uint256 price;
  }

  PendingAppraisalUpdate[] pendingAppraisalUpdates;

  struct PendingAppraisalUpdate {
    uint256 accountId;
    uint256 score;
    uint256[] addTopics;
    uint256[] removeTopics;
  }

  event XHandleAdded(uint256 accountId, Handle handle);
  event XCampaignAdded(uint256 accountId, uint256 campaignId, Campaign campaign);
  event XCollabAdded(uint256 accountId, Collab collab);

  struct Account {
      bool active;
      uint256 docBalance;
      uint256 asamiTokens;
      address addr;
      Handle x;
      Handle nostr;
      Handle instagram;
      Campaign[] xCampaigns;
      Campaign[] nostrCampaigns;
      Campaign[] instagramCampaigns;
      Collab[] xCollabs;
      Collab[] nostrCollabs;
      Collab[] instagramCollabs;
  }

  struct Handle {
    string value;
    string fixedId;
    uint256 price;
    uint256 score;
    uint256[] topics;
    string verificationMessageId;
  }

  struct Campaign {
    uint256 budget;
    uint256 remaining;
    string contentId;
  }

  struct Collab {
    uint256 accountId;
    uint256 campaignId;
    uint256 reward;
  }

  constructor(address _dollarOnChainAddress) {
      rewardToken = IERC20(_dollarOnChainAddress);
  }

  function getTopics() external view returns (string[] memory) {
    return topics;
  }

  function setAdmin(address _admin) external onlyOwner {
    admin = _admin;
  }

  function addTopic(string calldata _name) external {
    require(msg.sender == admin || msg.sender == owner());
    topics.push(_name);
  }

  function addXHandle (
    uint256 _accountId,
    Handle calldata _handle
  ) external {
    require(msg.sender == admin || msg.sender == owner());
    Account storage account = accounts[_accountId];
    account.active = true;
    account.x = _handle;
    emit XHandleAdded(_accountId, _handle);
  }

  function addRequestedXCampaign(
    uint256 _accountId,
    Campaign calldata _campaign
  ) external {
    require(msg.sender == admin || msg.sender == owner());

    require(_campaign.budget > 0);
    require(bytes(_campaign.contentId).length > 0);

    Account storage account = accounts[_accountId];
    require(account.active && account.addr == address(0));
    
    require(rewardToken.transferFrom(msg.sender, address(this), _campaign.budget));

    uint256 campaignId = account.xCampaigns.length;
    account.xCampaigns.push(_campaign);
    emit XCampaignAdded(_accountId, campaignId, _campaign);
  }

  /* A self-managed account can add its own campaign too function addXCampaign() { } */

  function addXCollab(
    uint256 _accountId,
    Collab calldata _collab
  ) external {
    require(msg.sender == admin || msg.sender == owner());

    Account storage member = accounts[_accountId];
    require(member.active);

    Account storage advertiser = accounts[_collab.accountId];
    require(advertiser.active);

    Campaign storage campaign = advertiser.xCampaigns[_collab.campaignId];
    require(bytes(campaign.contentId).length > 0);

    require(campaign.remaining > member.x.price);

    member.docBalance += member.x.price;
    campaign.remaining -= member.x.price;

    emit XCollabAdded(_accountId, _collab);
  }

  /*
  function addHandleTopicsForX(
    uint256 calldata _accountId,
    Handle calldata _handle
  ) external onlyOwner {
  }
  */
  
  /*
    uint constant oneDay = 60 * 60 * 24;
    uint constant campaignDuration = oneDay * 14;

    uint internal feePerOffer = 1e18;
    uint internal maxFeePerCampaign = 50e18 ;
    uint internal fees = 0;
    address internal feesAddress;

    event DebugEvent(string);

    struct Campaign {
        uint256 funding;
        address advertiser;
        uint256 startDate;
        AsamiNostr.Terms nostrTerms;
        AsamiClassic.Terms classicTerms;
    }

    Campaign[] internal campaigns;

    string[] internal socialNetworks;

    struct DeletionPenalty {
        address payable creditor;
        uint256 amount;
    }
    mapping(address => DeletionPenalty[]) internal deletionPenalties;

    mapping(address => uint256) internal oracleFees;

    struct OfferPointer {
        uint campaignId;
        uint offerId;
    }

    IERC20 internal rewardToken;
    
    constructor(address _dollarOnChainAddress) {
        rewardToken = IERC20(_dollarOnChainAddress);
    }

    function nostrCreateCampaign(
      uint256 _funding,
      uint256 _startDate,
      string calldata _requestedContent,
      AsamiNostr.NewOffer[] calldata _newOffers
    ) external {
        require(_newOffers.length > 0);
        require(_funding > 0);
        require(bytes(_requestedContent).length > 0);

        Campaign storage campaign = campaigns.push();
        campaign.funding = _funding;
        campaign.startDate = _startDate;
        campaign.advertiser = payable(msg.sender);
        campaign.nostrTerms.requestedContent = _requestedContent;

        uint256 totalRewards = 0;
        uint256 totalFees = 0;
        for (uint i = 0; i < _newOffers.length; i++) {
            require(_newOffers[i].rewardAmount > 0);
            totalRewards += _newOffers[i].rewardAmount;
            totalFees += feePerOffer;
            AsamiNostr.Offer storage offer = campaign.nostrTerms.offers.push();
            offer.state = AsamiNostr.OfferState.Assumed;
            offer.collected = false;
            offer.nostrHexPubkey = _newOffers[i].nostrHexPubkey;
            offer.rewardAmount = _newOffers[i].rewardAmount;
            offer.rewardAddress = payable(_newOffers[i].rewardAddress);
            offer.nostrAffineX = _newOffers[i].nostrAffineX;
            offer.nostrAffineY = _newOffers[i].nostrAffineY;
            offer.messageId = bytes32(0);
        }

        if (totalFees > maxFeePerCampaign) {
          totalFees = maxFeePerCampaign;
        }

        require((totalRewards + totalFees) == _funding);
        require(rewardToken.transferFrom(msg.sender, address(this), _funding));
    }

    struct NewClassicCampaign{
      uint256 funding;
      uint256 startDate;
      uint socialNetworkId;
      string rulesUrl;
      bytes32 rulesHash;
      address oracleAddress;
      uint256 oracleFee;
      AsamiClassic.NewOffer[] newOffers;
    }

    function classicCreateCampaign(
      NewClassicCampaign calldata _c
    ) external {
        require(_c.newOffers.length > 0, "0");
        require(_c.funding > 0, "1");
        require(bytes(_c.rulesUrl).length > 5, "2");
        require(_c.rulesHash != bytes32(0), "3");
        require(_c.oracleAddress != address(0), "4");
        require(_c.oracleFee > 0, "5");
        require(_c.socialNetworkId < socialNetworks.length, "6");

        Campaign storage campaign = campaigns.push();
        campaign.funding = _c.funding;
        campaign.startDate = _c.startDate;
        campaign.advertiser = payable(msg.sender);

        AsamiClassic.Terms storage terms = campaign.classicTerms;
        terms.socialNetworkId = _c.socialNetworkId;
        terms.rulesUrl = _c.rulesUrl;
        terms.rulesHash = _c.rulesHash;
        terms.oracleAddress = _c.oracleAddress;
        terms.oracleFee = _c.oracleFee;

        uint256 totalRewards = 0;
        uint256 totalFees = 0;
        for (uint i = 0; i < _c.newOffers.length; i++) {
            require(_c.newOffers[i].rewardAmount > 0);
            totalRewards += _c.newOffers[i].rewardAmount;
            totalFees += feePerOffer;

            AsamiClassic.Offer storage offer = campaign.classicTerms.offers.push();
            offer.state = AsamiClassic.OfferState.Assumed;
            offer.username = _c.newOffers[i].username;
            offer.rewardAmount = _c.newOffers[i].rewardAmount;
            offer.rewardAddress = payable(_c.newOffers[i].rewardAddress);
        }

        if (totalFees > maxFeePerCampaign) {
          totalFees = maxFeePerCampaign;
        }

        require((totalRewards + totalFees) == _c.funding, "7");
        require(rewardToken.transferFrom(msg.sender, address(this), _c.funding), "8");
    }

    function nostrChallenge(OfferPointer calldata _p) public {
        Campaign storage campaign = getCampaignInTimeWindow(_p, 0, oneDay);
        require(campaign.advertiser == msg.sender);

        AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[_p.offerId];
        require(offer.state == AsamiNostr.OfferState.Assumed, "1");
        offer.state = AsamiNostr.OfferState.Challenged;
    }

    // Anyone can submit proof of a message being published, likely a Collaborator or someone on their behalf.
    // The r value is the first 32 bytes of the signature, and the s value are the remaining 32,
    function nostrConfirm(
        OfferPointer calldata _p,
        AsamiNostr.MessageProof calldata _proof
    ) external {
        Campaign storage campaign = getCampaignInTimeWindow(_p, 0, oneDay * 2);

        AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[_p.offerId];
        require(offer.state == AsamiNostr.OfferState.Challenged);

        bytes32 messageId = AsamiNostr.verifyCampaignMessage(
          campaign.nostrTerms.requestedContent,
          offer.nostrHexPubkey,
          _proof,
          offer.nostrAffineX,
          offer.nostrAffineY
        );

        offer.messageId = messageId;
        offer.state = AsamiNostr.OfferState.Confirmed;
    }

    function nostrRenounce(OfferPointer calldata _p) external {
        Campaign storage campaign = getCampaignInTimeWindow(_p, 0, campaignDuration);
        AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[_p.offerId];

        require(offer.rewardAddress == msg.sender);

        require(
          offer.state == AsamiNostr.OfferState.Assumed ||
          offer.state == AsamiNostr.OfferState.Challenged ||
          offer.state == AsamiNostr.OfferState.Confirmed
        );

        offer.state = AsamiNostr.OfferState.Renounced;
    }

    function nostrReportConfirmed(
        OfferPointer calldata _p,
        AsamiNostr.DeletionProof calldata _deletionProof
    ) external {
        uint256 startGas = gasleft();

        Campaign storage campaign = getCampaignInTimeWindow(_p, 0, campaignDuration);

        AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[_p.offerId];
        require(offer.state == AsamiNostr.OfferState.Confirmed);

        require(AsamiNostr.idInTags(offer.messageId, _deletionProof.tags));

        AsamiNostr.verifyDeletionMessage(
          offer.nostrHexPubkey,
          _deletionProof,
          offer.nostrAffineX,
          offer.nostrAffineY
        );

        offer.state = AsamiNostr.OfferState.ReportedDeletion;

        uint256 refund = ((startGas - gasleft()) * 11) / 10;
        deletionPenalties[offer.rewardAddress].push(DeletionPenalty(payable(msg.sender), refund));
    }

    function nostrReportAssumed (
        OfferPointer calldata _p,
        AsamiNostr.MessageProof calldata _messageProof,
        AsamiNostr.DeletionProof calldata _deletionProof
    )
        external 
    {
        uint256 startGas = gasleft();

        Campaign storage campaign = getCampaignInTimeWindow(_p, 0, campaignDuration);

        AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[_p.offerId];
        require(offer.state == AsamiNostr.OfferState.Assumed);

        bytes32 messageId = AsamiNostr.verifyCampaignMessage(
            campaign.nostrTerms.requestedContent,
            offer.nostrHexPubkey,
            _messageProof,
            offer.nostrAffineX,
            offer.nostrAffineY
        );

        offer.messageId = messageId;

        require(AsamiNostr.idInTags(offer.messageId, _deletionProof.tags));

        AsamiNostr.verifyDeletionMessage(
          offer.nostrHexPubkey,
          _deletionProof,
          offer.nostrAffineX,
          offer.nostrAffineY
        );

        offer.state = AsamiNostr.OfferState.ReportedDeletion;

        uint256 refund = ((startGas - gasleft()) * 11) / 10;
        deletionPenalties[offer.rewardAddress].push(DeletionPenalty(payable(msg.sender), refund));
    }

    function classicChallenge(
      OfferPointer calldata _p
    ) external {
        Campaign storage campaign = getCampaignInTimeWindow(_p, 0, oneDay);
        require(campaign.advertiser == msg.sender);

        AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_p.offerId];
        require(offer.state == AsamiClassic.OfferState.Assumed);

        offer.state = AsamiClassic.OfferState.Challenged;
    }

    function classicRequestConfirmation(
      OfferPointer calldata _p
    ) external {
        Campaign storage campaign = getCampaignInTimeWindow(_p, 0, oneDay * 2 );

        AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_p.offerId];
        require(offer.state == AsamiClassic.OfferState.Challenged);

        require(rewardToken.transferFrom(msg.sender, address(this), campaign.classicTerms.oracleFee));

        offer.state = AsamiClassic.OfferState.ConfirmationRequested;
    }

    function classicConfirm(
      OfferPointer calldata _p,
      bool _found
    ) external {
        Campaign storage campaign = getCampaignInTimeWindow(_p, 0, oneDay * 3 );
        require(campaign.classicTerms.oracleAddress == msg.sender);

        AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_p.offerId];
        require(offer.state == AsamiClassic.OfferState.ConfirmationRequested);
        
        oracleFees[campaign.classicTerms.oracleAddress] += campaign.classicTerms.oracleFee;

        if (_found) {
          offer.state = AsamiClassic.OfferState.Confirmed;
        } else {
          offer.state = AsamiClassic.OfferState.Refuted;
        }
    }

    function classicConfirmFree(
      OfferPointer calldata _p,
      bool _found
    ) external {
        Campaign storage campaign = getCampaignInTimeWindow(_p, 0, oneDay * 3 );

        require(campaign.classicTerms.oracleAddress == msg.sender);

        AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_p.offerId];
        require(offer.state == AsamiClassic.OfferState.Challenged);
        
        if (_found) {
          offer.state = AsamiClassic.OfferState.Confirmed;
        } else {
          offer.state = AsamiClassic.OfferState.Refuted;
        }
    }

    function classicRenounce(
      OfferPointer calldata _p
    ) external {
        Campaign storage campaign = getCampaignInTimeWindow(_p, 0, campaignDuration);
        AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_p.offerId];

        require(offer.rewardAddress == msg.sender);

        require(
          offer.state == AsamiClassic.OfferState.Assumed ||
          offer.state == AsamiClassic.OfferState.Challenged ||
          offer.state == AsamiClassic.OfferState.Confirmed
        );

        offer.state = AsamiClassic.OfferState.Renounced;
    }

    function classicReportDeletion(
      OfferPointer calldata _p
    ) external {
        Campaign storage campaign = getCampaignInTimeWindow(_p, 0, campaignDuration);
        require(campaign.advertiser == msg.sender);

        AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_p.offerId];

        require(
          offer.state == AsamiClassic.OfferState.Assumed ||
          offer.state == AsamiClassic.OfferState.Confirmed
        );

        require(rewardToken.transferFrom(msg.sender, address(this), campaign.classicTerms.oracleFee));

        offer.deletionReportedAt = block.timestamp;
        offer.state = AsamiClassic.OfferState.DeletionReported;
    }

    function classicConfirmDeletion(
      OfferPointer calldata _p,
      bool _wasDeleted
    ) external {
        Campaign storage campaign = getCampaignInTimeWindow(_p, 0, campaignDuration);
        require(campaign.classicTerms.oracleAddress == msg.sender);

        AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_p.offerId];
        require(offer.state == AsamiClassic.OfferState.DeletionReported);

        oracleFees[campaign.classicTerms.oracleAddress] += campaign.classicTerms.oracleFee;

        if (_wasDeleted) {
          offer.state = AsamiClassic.OfferState.DeletionConfirmed;
        } else {
          offer.state = AsamiClassic.OfferState.Confirmed;
        }
    }

    function getCollectableCollaboratorOffers(
        address _collaborator
    )
        external view returns (OfferPointer[] memory, OfferPointer[] memory) 
    {
        OfferPointer[] memory nostrPointers = new OfferPointer[](0);
        OfferPointer[] memory classicPointers = new OfferPointer[](0);

        for(uint i = 0; i < campaigns.length; i++) {
          Campaign storage campaign = campaigns[i];
          if( block.timestamp < (campaign.startDate + campaignDuration) ) {
            continue;
          }

          if (campaign.nostrTerms.offers.length > 0) {
            for(uint j = 0; j < campaign.nostrTerms.offers.length; j++) {
              AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[j];
              if(
                offer.rewardAddress == _collaborator && 
                AsamiNostr.collaboratorCanCollect(offer)
              ) {
                nostrPointers = pushPointer(nostrPointers, i, j);
              }
            }
          } else {
            for(uint j = 0; j < campaign.classicTerms.offers.length; j++) {
              AsamiClassic.Offer memory offer = campaign.classicTerms.offers[j];
              if(
                offer.rewardAddress == _collaborator && 
                !offer.collaboratorCollected &&
                offer.state != AsamiClassic.OfferState.Challenged &&
                offer.state != AsamiClassic.OfferState.Refuted &&
                offer.state != AsamiClassic.OfferState.Renounced &&
                offer.state != AsamiClassic.OfferState.DeletionConfirmed
              ) {
                classicPointers = pushPointer(classicPointers, i, j);
              }
            }
          }
        }
        return (nostrPointers, classicPointers);
    }

    function collectCollaboratorOffers(
        address _collaborator,
        OfferPointer[] calldata _nostrPointers,
        OfferPointer[] calldata _classicPointers
    ) external {
        require(deletionPenalties[_collaborator].length == 0);

        uint256 total = 0;

        for (uint i = 0; i < _nostrPointers.length; i++) {
            Campaign storage campaign = campaigns[_nostrPointers[i].campaignId];
            require(block.timestamp >= (campaign.startDate + campaignDuration), "1"); // Campaign not ended.

            AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[_nostrPointers[i].offerId];

            require(offer.rewardAddress == _collaborator, "2"); // Wrong collaborator.
            require(AsamiNostr.collaboratorCanCollect(offer));

            total = offer.rewardAmount;
            offer.collected = true;
        }

        for (uint i = 0; i < _classicPointers.length; i++) {
            Campaign storage campaign = campaigns[_classicPointers[i].campaignId];
            require(block.timestamp >= (campaign.startDate + campaignDuration));

            AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_classicPointers[i].offerId];

            require(offer.rewardAddress == _collaborator);
            require(!offer.collaboratorCollected);

            if( offer.state == AsamiClassic.OfferState.Assumed || offer.state == AsamiClassic.OfferState.Confirmed) {
              total += offer.rewardAmount;
            } else if( offer.state == AsamiClassic.OfferState.ConfirmationRequested ) {
              total += campaign.classicTerms.oracleFee;
            } else if( offer.state == AsamiClassic.OfferState.DeletionReported ) {
              total += (offer.deletionReportedAt - campaign.startDate) * offer.rewardAmount / campaignDuration;
            } else {
              revert();
            }
            offer.collaboratorCollected = true;
        }

        require(rewardToken.transfer(_collaborator, total));
    }

    function getCollectableAdvertiserOffers(
      address _advertiser
    )
        external view returns (OfferPointer[] memory, OfferPointer[] memory) 
    {
        OfferPointer[] memory nostrPointers = new OfferPointer[](0);
        OfferPointer[] memory classicPointers = new OfferPointer[](0);

        for(uint i = 0; i < campaigns.length; i++) {
          Campaign storage campaign = campaigns[i];

          if( 
            campaign.advertiser != _advertiser ||
            block.timestamp < (campaign.startDate + campaignDuration)
          ){
            continue;
          }

          if (campaign.nostrTerms.offers.length > 0) {
            for(uint j = 0; j < campaign.nostrTerms.offers.length; j++) {
              AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[j];
              if( AsamiNostr.advertiserCanCollect(offer) ) {
                nostrPointers = pushPointer(nostrPointers, i, j);
              }
            }
          } else {
            for(uint j = 0; j < campaign.classicTerms.offers.length; j++) {
              AsamiClassic.Offer memory offer = campaign.classicTerms.offers[j];
              if(
                !offer.advertiserCollected &&
                offer.state != AsamiClassic.OfferState.Assumed &&
                offer.state != AsamiClassic.OfferState.Confirmed
              ) {
                nostrPointers = pushPointer(nostrPointers, i, j);
              }
            }
          }
        }
        return (nostrPointers, classicPointers);
    }

    function collectAdvertiserOffers(
      address payable _advertiser,
      OfferPointer[] calldata _nostrOffers,
      OfferPointer[] calldata _classicOffers
    ) external {
        uint256 total = 0;

        for (uint i = 0; i < _nostrOffers.length; i++) {
            OfferPointer memory p = _nostrOffers[i];

            Campaign storage campaign = campaigns[p.campaignId];
            require(campaign.advertiser == _advertiser);
            require(block.timestamp >= (campaign.startDate + campaignDuration));

            AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[p.offerId];

            require(AsamiNostr.advertiserCanCollect(offer));

            total += offer.rewardAmount;
            offer.collected = true;
        }

        for (uint i = 0; i < _classicOffers.length; i++) {
            OfferPointer memory p = _classicOffers[i];

            Campaign storage campaign = campaigns[p.campaignId];
            require(campaign.advertiser == _advertiser);
            require(block.timestamp >= (campaign.startDate + campaignDuration));

            AsamiClassic.Offer storage offer = campaign.classicTerms.offers[p.offerId];
            require(!offer.advertiserCollected);

            if(
              offer.state == AsamiClassic.OfferState.Challenged ||
              offer.state == AsamiClassic.OfferState.ConfirmationRequested ||
              offer.state == AsamiClassic.OfferState.Refuted ||
              offer.state == AsamiClassic.OfferState.Renounced ||
              offer.state == AsamiClassic.OfferState.DeletionConfirmed
            ) {
              total += offer.rewardAmount;
            } else if( offer.state == AsamiClassic.OfferState.DeletionReported ) {
              total += (campaignDuration - (offer.deletionReportedAt - campaign.startDate)) * offer.rewardAmount / campaignDuration;
            } else {
              revert();
            }
            offer.advertiserCollected = true;
        }

        require(rewardToken.transfer(_advertiser, total));
    }

    function payDeletionPenalties(
      address _debtor
    ) external payable {
        DeletionPenalty[] storage penalties = deletionPenalties[_debtor];
        uint256 total = 0;

        for (uint256 i = 0; i < penalties.length; i++) {
            total += penalties[i].amount;
        }

        require(total > 0);
        require(msg.value == total);

        for (uint256 i = 0; i < penalties.length; i++) {
            payable(penalties[i].creditor).transfer(penalties[i].amount);
        }

        delete deletionPenalties[_debtor];
    }


    function setAsamiFeePerOffer(uint256 _fee) external onlyOwner {
        feePerOffer = _fee;
    }

    function setAsamiMaxFeePerCampaign(uint256 _fee) external onlyOwner {
        maxFeePerCampaign = _fee;
    }

    function setAsamiFeesAddress(address _feesAddress) external onlyOwner {
        feesAddress = _feesAddress;
    }

    function collectAsamiFees() external {
        require(fees > 0);
        require(rewardToken.transferFrom(address(this), feesAddress, fees));
        fees = 0;
    }

    function collectOracleFees(address _oracle) external {
        require(oracleFees[_oracle] > 0);
        require(rewardToken.transferFrom(address(this), _oracle, oracleFees[_oracle]));
        oracleFees[_oracle] = 0;
    }

    function nostrGetCampaignOffer(OfferPointer memory _p) external view returns (AsamiNostr.Offer memory) {
        return campaigns[_p.campaignId].nostrTerms.offers[_p.offerId];
    }

    function classicGetCampaignOffer(OfferPointer memory _p) external view returns (AsamiClassic.Offer memory) {
        return campaigns[_p.campaignId].classicTerms.offers[_p.offerId];
    }

    function calculateCampaignFees(uint _offerCount) external view returns (uint256) {
        uint256 total = _offerCount * feePerOffer;
        return total > maxFeePerCampaign ? maxFeePerCampaign : total;
    }

    function getCampaigns() external view returns (Campaign[] memory) {
        return campaigns;
    }

    function getSocialNetworks() external view returns (string[] memory) {
        return socialNetworks;
    }

    function getCampaignInTimeWindow(
      OfferPointer calldata _p,
      uint256 _offset,
      uint256 _duration
    ) internal view returns (Campaign storage) {
        Campaign storage campaign = campaigns[_p.campaignId];
        require(block.timestamp >= (campaign.startDate + _offset), "99");
        require(block.timestamp <= (campaign.startDate + _offset + _duration), "99");
        return campaign;
    }

    function pushPointer(
      OfferPointer[] memory _pointers,
      uint256 _campaignId,
      uint256 _offerId
    ) internal pure returns (OfferPointer[] memory) {
      OfferPointer[] memory tmp = new OfferPointer[](_pointers.length + 1);
      for (uint k = 0; k < _pointers.length; k++) {
          tmp[k] = _pointers[k];
      }
      tmp[_pointers.length] = OfferPointer(_campaignId, _offerId);
      return tmp;
    }
  */
}
