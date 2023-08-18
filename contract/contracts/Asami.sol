// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;
import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "./AsamiClassic.sol";
import "./AsamiNostr.sol";

/*
   Asami methods are like "controllers", all public.
   Libraries are models, fully internal.
   Common structs and checks in campaigns may stay in asami or go to a Campaigns library.
      (getCampaignForChallenge -> get campaign in challenge window.)
      (getCampaignForChallenge -> get campaign in confirmation window.)
      (getActiveCampaign -> not expired).
      (always get active campaign).
   Penalize classic collaborators that delete without renouncing.
*/

contract Asami is Ownable {
    uint constant aDay = 60 * 60 * 24; // A day
    uint constant challengeWindow = 60 * 60 * 24; // A day
    uint constant proofWindow = challengeWindow * 2;
    uint constant campaignDuration = challengeWindow * 14;

    uint256 internal feePerOffer = 1e18;
    uint256 internal maxFeePerCampaign = 50e18 ;
    uint256 internal fees = 0;
    address internal feesAddress;

    struct Campaign {
        uint256 funding;
        address advertiser;
        uint256 deadline;
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

    /* An offer pointer represents a specific offer made in a campaign */
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
      uint256 _deadline,
      string calldata _requestedContent,
      AsamiNostr.NewOffer[] calldata _newOffers
    ) public {
        require(_newOffers.length > 0, "1");
        require(_funding > 0, "2");
        require(bytes(_requestedContent).length > 0, "3");

        Campaign storage campaign = campaigns.push();
        campaign.funding = _funding;
        campaign.deadline = _deadline;
        campaign.advertiser = payable(msg.sender);
        campaign.nostrTerms.requestedContent = _requestedContent;

        uint256 totalRewards = 0;
        uint256 totalFees = 0;
        for (uint i = 0; i < _newOffers.length; i++) {
            require(_newOffers[i].rewardAmount > 0, "4");
            totalRewards += _newOffers[i].rewardAmount;
            totalFees += feePerOffer;
            AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[i];
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

        require((totalRewards + totalFees) == _funding, "5");
        require(rewardToken.transferFrom(msg.sender, address(this), _funding), "6");
    }

    struct NewClassicCampaign{
      uint256 funding;
      uint256 deadline;
      uint socialNetworkId;
      string rulesUrl;
      bytes32 rulesHash;
      address oracleAddress;
      uint256 oracleFee;
      AsamiClassic.NewOffer[] newOffers;
    }

    function classicCreateCampaign(
      NewClassicCampaign calldata _c
    ) public {
        require(_c.newOffers.length > 0, "1");
        require(_c.funding > 0, "2");
        require(bytes(_c.rulesUrl).length > 5, "3");
        require(_c.rulesHash != bytes32(0), "4");
        require(_c.oracleAddress != address(0), "5");
        require(_c.oracleFee > 0, "6");
        require(_c.socialNetworkId < socialNetworks.length, "7");

        Campaign storage campaign = campaigns.push();
        campaign.funding = _c.funding;
        campaign.deadline = _c.deadline;
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
            require(_c.newOffers[i].rewardAmount > 0, "8");
            totalRewards += _c.newOffers[i].rewardAmount;
            totalFees += feePerOffer;

            AsamiClassic.Offer storage offer = campaign.classicTerms.offers[i];
            offer.state = AsamiClassic.OfferState.Assumed;
            offer.username = _c.newOffers[i].username;
            offer.rewardAmount = _c.newOffers[i].rewardAmount;
            offer.rewardAddress = payable(_c.newOffers[i].rewardAddress);
        }

        if (totalFees > maxFeePerCampaign) {
          totalFees = maxFeePerCampaign;
        }

        require((totalRewards + totalFees) == _c.funding, "8");
        require(rewardToken.transferFrom(msg.sender, address(this), _c.funding), "9");
    }

    function getCampaignForChallenge(OfferPointer calldata _p) internal view returns (Campaign storage) {
        Campaign storage campaign = campaigns[_p.campaignId];
        require(campaign.classicTerms.offers.length > 0, "1");

        require(campaign.advertiser == msg.sender, "2");
        require(block.timestamp <= (campaign.deadline + challengeWindow), "3");
        return campaign;
    }

    function nostrChallenge(OfferPointer calldata _p) public {
        Campaign storage campaign = getCampaignForChallenge(_p);
        AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[_p.offerId];
        require(offer.state == AsamiNostr.OfferState.Assumed, "1");
        offer.state = AsamiNostr.OfferState.Challenged;
    }

    // Anyone can submit proof of a message being published, likely a Collaborator or someone on their behalf.
    // The r value is the first 32 bytes of the signature, and the s value are the remaining 32,
    function nostrConfirm(
        OfferPointer calldata _p,
        AsamiNostr.MessageProof calldata _proof
    ) public {
        Campaign storage campaign = campaigns[_p.campaignId];
        require(block.timestamp <= (campaign.deadline + proofWindow), "1");

        AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[_p.offerId];
        require(offer.state == AsamiNostr.OfferState.Challenged, "2");

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

    /*
     The collaborator can always force a refund.
     If the collaborator deletes a message, the advertiser can submit proof of deletion
     to prevent payment from happening.
     Submitting the proof of deletion for a message may be very costly for the advertiser,
     so when this happens a penalty is applied on the collaborator in favor of the advertiser.
     To prevent this penalty, the collaborator may forfeit the reward
     voluntarily before deleting the message.
    */
    function nostrRenounce(OfferPointer calldata _p) public {
        Campaign storage campaign = campaigns[_p.campaignId];

        AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[_p.offerId];
        require(offer.rewardAddress == msg.sender, "1");

        require(
          offer.state != AsamiNostr.OfferState.Renounced &&
          offer.state != AsamiNostr.OfferState.ReportedDeletion,
          "2"
        );

        offer.state = AsamiNostr.OfferState.Renounced;
    }

    // Submitting a deletion proof is very costly on the advertiser, so if it comes to that,
    // the collaborator gets penalized and is banned from the system until he pays for the penalties.
    // To reduce gas usage, this method does not parse the tags json, it just does a raw string search,
    // which initially may be enough.
    // The r value is the first 32 bytes of the signature, and the s value are the remaining 32,
    function nostrReportConfirmed(
        OfferPointer calldata _p,
        AsamiNostr.DeletionProof calldata _deletionProof
    ) public {
        uint256 startGas = gasleft();

        Campaign storage campaign = campaigns[_p.campaignId];
        require(block.timestamp <= (campaign.deadline + campaignDuration), "1");

        AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[_p.offerId];
        require(offer.state == AsamiNostr.OfferState.Confirmed, "2");

        require(AsamiNostr.idInTags(offer.messageId, _deletionProof.tags), "3");

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
        public 
    {
        uint256 startGas = gasleft();

        Campaign storage campaign = campaigns[_p.campaignId];
        require(block.timestamp <= (campaign.deadline + campaignDuration), "1");

        AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[_p.offerId];
        require(offer.state == AsamiNostr.OfferState.Assumed, "2");

        bytes32 messageId = AsamiNostr.verifyCampaignMessage(
            campaign.nostrTerms.requestedContent,
            offer.nostrHexPubkey,
            _messageProof,
            offer.nostrAffineX,
            offer.nostrAffineY
        );

        offer.messageId = messageId;

        require(AsamiNostr.idInTags(offer.messageId, _deletionProof.tags), "3");

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

    function classicChallenge(OfferPointer calldata _p) public {
        Campaign storage campaign = getCampaignForChallenge(_p);
        AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_p.offerId];

        require(offer.state == AsamiClassic.OfferState.Assumed, "1");
        offer.state = AsamiClassic.OfferState.Challenged;
    }

    function classicRequestConfirmation(OfferPointer calldata _p) public {
        Campaign storage campaign = campaigns[_p.campaignId];
        require(block.timestamp <= (campaign.deadline + proofWindow), "1");

        AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_p.offerId];
        require(offer.state == AsamiClassic.OfferState.Challenged, "2");

        require(rewardToken.transferFrom(msg.sender, address(this), campaign.classicTerms.oracleFee), "3");

        offer.state = AsamiClassic.OfferState.ConfirmationRequested;
    }

    function classicConfirm(OfferPointer calldata _p, bool _found) public {
        Campaign storage campaign = campaigns[_p.campaignId];
        require(campaign.classicTerms.oracleAddress == msg.sender, "1");

        require(block.timestamp <= (campaign.deadline + proofWindow + aDay), "2");

        AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_p.offerId];
        require(offer.state == AsamiClassic.OfferState.ConfirmationRequested, "3");
        
        oracleFees[campaign.classicTerms.oracleAddress] += campaign.classicTerms.oracleFee;

        if (_found) {
          offer.state = AsamiClassic.OfferState.Confirmed;
        } else {
          offer.state = AsamiClassic.OfferState.Refuted;
        }
    }

    function classicConfirmFree(OfferPointer calldata _p, bool _found) public {
        Campaign storage campaign = campaigns[_p.campaignId];
        require(campaign.classicTerms.oracleAddress == msg.sender, "1");

        require(block.timestamp <= (campaign.deadline + proofWindow + aDay), "2");

        AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_p.offerId];
        require(offer.state == AsamiClassic.OfferState.Challenged, "3");
        
        if (_found) {
          offer.state = AsamiClassic.OfferState.Confirmed;
        } else {
          offer.state = AsamiClassic.OfferState.Refuted;
        }
    }

    function classicRenounce(OfferPointer calldata _p) public {
        Campaign storage campaign = campaigns[_p.campaignId];

        AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_p.offerId];
        require(offer.rewardAddress == msg.sender, "1");

        require(
          offer.state == AsamiClassic.OfferState.Assumed ||
          offer.state == AsamiClassic.OfferState.Challenged ||
          offer.state == AsamiClassic.OfferState.ConfirmationRequested ||
          offer.state == AsamiClassic.OfferState.Confirmed,
          "2"
        );

        offer.state = AsamiClassic.OfferState.Renounced;
    }

    function classicReportDeletion(OfferPointer calldata _p) public {
        Campaign storage campaign = campaigns[_p.campaignId];
        require(block.timestamp <= (campaign.deadline + proofWindow), "1");

        AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_p.offerId];

        require(
          offer.state == AsamiClassic.OfferState.Assumed ||
          offer.state == AsamiClassic.OfferState.Confirmed,
          "2"
        );

        require(rewardToken.transferFrom(msg.sender, address(this), campaign.classicTerms.oracleFee), "3");

        offer.deletionReportedAt = block.timestamp;
        offer.state = AsamiClassic.OfferState.DeletionReported;
    }

    function classicConfirmDeletion(OfferPointer calldata _p, bool _wasDeleted) public {
        Campaign storage campaign = campaigns[_p.campaignId];
        require(campaign.classicTerms.oracleAddress == msg.sender, "1");
        require(block.timestamp <= (campaign.deadline + campaignDuration), "2");

        AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_p.offerId];

        require(offer.state == AsamiClassic.OfferState.DeletionReported, "3");

        oracleFees[campaign.classicTerms.oracleAddress] += campaign.classicTerms.oracleFee;

        if (_wasDeleted) {
          offer.state = AsamiClassic.OfferState.DeletionConfirmed;
        } else {
          offer.state = AsamiClassic.OfferState.Confirmed;
        }
    }

    function payDeletionPenalties(address _debtor) public payable {
        DeletionPenalty[] storage penalties = deletionPenalties[_debtor];
        uint256 total = 0;

        for (uint256 i = 0; i < penalties.length; i++) {
            total += penalties[i].amount;
        }

        require(total > 0, "1");

        require(msg.value != total, "2");

        for (uint256 i = 0; i < penalties.length; i++) {
            payable(penalties[i].creditor).transfer(penalties[i].amount);
        }

        delete deletionPenalties[_debtor];
    }

    /*
      Collects pointers to all offers which are interesting for a collaborator.
      It could be because the collaborator has a reward to receive or a refund of oracle fees.
    */
    function getActiveCollaboratorOffers(address _collaborator)
        public view returns (OfferPointer[] memory, OfferPointer[] memory) 
    {
        OfferPointer[] memory nostrPointers = new OfferPointer[](0);
        OfferPointer[] memory classicPointers = new OfferPointer[](0);

        for(uint i = 0; i < campaigns.length; i++) {
          Campaign memory campaign = campaigns[i];

          if (campaign.nostrTerms.offers.length > 0) {
            for(uint j = 0; j < campaign.nostrTerms.offers.length; j++) {
              AsamiNostr.Offer memory offer = campaign.nostrTerms.offers[j];
              if(
                offer.rewardAddress == _collaborator && 
                !offer.collected &&
                offer.state != AsamiNostr.OfferState.Challenged &&
                offer.state != AsamiNostr.OfferState.Renounced &&
                offer.state != AsamiNostr.OfferState.ReportedDeletion
              ) {
                OfferPointer[] memory tmp = new OfferPointer[](nostrPointers.length + 1);
                for (uint k = 0; k < nostrPointers.length; k++) {
                    tmp[k] = nostrPointers[k];
                }
                tmp[nostrPointers.length] = OfferPointer(i,j);
                nostrPointers = tmp;
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
                OfferPointer[] memory tmp = new OfferPointer[](classicPointers.length + 1);
                for (uint k = 0; k < classicPointers.length; k++) {
                    tmp[k] = classicPointers[k];
                }
                tmp[classicPointers.length] = OfferPointer(i,j);
                classicPointers = tmp;
              }
            }
          }
        }
        return (nostrPointers, classicPointers);
    }

    /*
      This function can be called by anyone to settle offers for a collaborator.
    */
    function settleCollaboratorOffers(
        address _collaborator,
        OfferPointer[] calldata _nostrPointers,
        OfferPointer[] calldata _classicPointers
    ) public {
        require(deletionPenalties[_collaborator].length == 0, "1");

        uint256 total = 0;

        for (uint i = 0; i < _nostrPointers.length; i++) {
            Campaign storage campaign = campaigns[_nostrPointers[i].campaignId];
            require(campaign.funding > 0, "2");
            require(block.timestamp >= (campaign.deadline + campaignDuration), "3");

            AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[_nostrPointers[i].offerId];

            require(offer.rewardAddress == _collaborator, "4");
            require(!offer.collected, "5");

            require(
              offer.state != AsamiNostr.OfferState.Challenged &&
              offer.state != AsamiNostr.OfferState.Renounced &&
              offer.state != AsamiNostr.OfferState.ReportedDeletion,
              "6"
            );

            total = offer.rewardAmount;
            offer.collected = true;
        }

        for (uint i = 0; i < _classicPointers.length; i++) {
            Campaign storage campaign = campaigns[_classicPointers[i].campaignId];
            require(campaign.funding > 0, "7");
            require(block.timestamp >= (campaign.deadline + campaignDuration), "8");

            AsamiClassic.Offer storage offer = campaign.classicTerms.offers[_classicPointers[i].offerId];

            require(offer.rewardAddress == _collaborator, "9");
            require(!offer.collaboratorCollected, "10");

            if( offer.state == AsamiClassic.OfferState.Assumed || offer.state == AsamiClassic.OfferState.Confirmed) {
              total += offer.rewardAmount;
            } else if( offer.state == AsamiClassic.OfferState.ConfirmationRequested ) {
              total += campaign.classicTerms.oracleFee;
            } else if( offer.state == AsamiClassic.OfferState.DeletionReported ) {
              total += (offer.deletionReportedAt - campaign.deadline) * offer.rewardAmount / campaignDuration;
            } else {
              revert("11");
            }
            offer.collaboratorCollected = true;
        }

        require(rewardToken.transfer(_collaborator, total), "12");
    }

    function getActiveAdvertiserOffers(address _advertiser)
        public view returns (OfferPointer[] memory, OfferPointer[] memory) 
    {
        OfferPointer[] memory nostrPointers = new OfferPointer[](0);
        OfferPointer[] memory classicPointers = new OfferPointer[](0);

        for(uint i = 0; i < campaigns.length; i++) {
          Campaign memory campaign = campaigns[i];

          if( campaign.advertiser != _advertiser ){
            continue;
          }

          if (campaign.nostrTerms.offers.length > 0) {
            for(uint j = 0; j < campaign.nostrTerms.offers.length; j++) {
              AsamiNostr.Offer memory offer = campaign.nostrTerms.offers[j];
              if(
                !offer.collected &&
                offer.state != AsamiNostr.OfferState.Assumed &&
                offer.state != AsamiNostr.OfferState.Confirmed
              ) {
                OfferPointer[] memory tmp = new OfferPointer[](nostrPointers.length + 1);
                for (uint k = 0; k < nostrPointers.length; k++) {
                    tmp[k] = nostrPointers[k];
                }
                tmp[nostrPointers.length] = OfferPointer(i,j);
                nostrPointers = tmp;
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
                OfferPointer[] memory tmp = new OfferPointer[](classicPointers.length + 1);
                for (uint k = 0; k < classicPointers.length; k++) {
                    tmp[k] = classicPointers[k];
                }
                tmp[classicPointers.length] = OfferPointer(i,j);
                classicPointers = tmp;
              }
            }
          }
        }
        return (nostrPointers, classicPointers);
    }

    /*
      This function is called by the Advertiser to collect refunds from non-awarded offers.
      It may also be called by third parties to give the refunds to the Advertiser.
      The Advertiser must provide a known list of offers that are ready to be refunded.
      The list can be built by calling the getCollectableRefunds function in the contract.
    */
    function settleAdvertiserOffers(
      address payable _advertiser,
      OfferPointer[] calldata _nostrOffers,
      OfferPointer[] calldata _classicOffers
    ) public {
        uint256 total = 0;

        for (uint i = 0; i < _nostrOffers.length; i++) {
            OfferPointer memory p = _nostrOffers[i];

            Campaign storage campaign = campaigns[p.campaignId];
            require(campaign.advertiser == _advertiser, "1");
            require(block.timestamp >= (campaign.deadline + proofWindow), "2");

            AsamiNostr.Offer storage offer = campaign.nostrTerms.offers[p.offerId];
            require(!offer.collected, "3");

            require(
              offer.state != AsamiNostr.OfferState.Assumed &&
              offer.state != AsamiNostr.OfferState.Confirmed,
              "4"
            );
            total += offer.rewardAmount;
            offer.collected = true;
        }

        for (uint i = 0; i < _classicOffers.length; i++) {
            OfferPointer memory p = _classicOffers[i];

            Campaign storage campaign = campaigns[p.campaignId];
            require(campaign.advertiser == _advertiser, "5");
            require(block.timestamp >= (campaign.deadline + proofWindow), "6");

            AsamiClassic.Offer storage offer = campaign.classicTerms.offers[p.offerId];
            require(!offer.advertiserCollected, "7");

            if(
              offer.state == AsamiClassic.OfferState.Challenged ||
              offer.state == AsamiClassic.OfferState.ConfirmationRequested ||
              offer.state == AsamiClassic.OfferState.Refuted ||
              offer.state == AsamiClassic.OfferState.Renounced ||
              offer.state == AsamiClassic.OfferState.DeletionConfirmed
            ) {
              total += offer.rewardAmount;
            } else if( offer.state == AsamiClassic.OfferState.DeletionReported ) {
              total += (campaignDuration - (offer.deletionReportedAt - campaign.deadline)) * offer.rewardAmount / campaignDuration;
            } else {
              revert("8");
            }
            offer.advertiserCollected = true;
        }

        require(rewardToken.transfer(_advertiser, total), "9");
    }

    function setAsamiFeePerOffer(uint256 _fee) public onlyOwner {
        feePerOffer = _fee;
    }

    function setAsamiMaxFeePerCampaign(uint256 _fee) public onlyOwner {
        maxFeePerCampaign = _fee;
    }

    function setAsamiFeesAddress(address _feesAddress) public onlyOwner {
        feesAddress = _feesAddress;
    }

    function addSocialNetwork(string calldata _name) public onlyOwner {
        socialNetworks.push(_name);
    }

    function collectAsamiFees() public {
        require(fees > 0, "1");
        require(rewardToken.transferFrom(address(this), feesAddress, fees), "2");
        fees = 0;
    }

    function collectOracleFees(address _oracle) public {
        require(oracleFees[_oracle] > 0, "1");
        require(rewardToken.transferFrom(address(this), _oracle, oracleFees[_oracle]), "2");
        oracleFees[_oracle] = 0;
    }

    function nostrGetCampaignOffer(OfferPointer memory _p) public view returns (AsamiNostr.Offer memory) {
        return campaigns[_p.campaignId].nostrTerms.offers[_p.offerId];
    }

    function classicGetCampaignOffer(OfferPointer memory _p) public view returns (AsamiClassic.Offer memory) {
        return campaigns[_p.campaignId].classicTerms.offers[_p.offerId];
    }

    function calculateCampaignFees(uint _offerCount) public view returns (uint256) {
        uint256 total = _offerCount * feePerOffer;
        return total > maxFeePerCampaign ? maxFeePerCampaign : total;
    }
}
