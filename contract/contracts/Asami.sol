// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;
import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "./Schnorr.sol";

/*
This smart contract regulates the relationship between a company willing to run an ad, the "Advertiser", an and one or several indpendent collaborators with a significant following, the "collaborators".

The Advertiser will create a campaign, offering DoC rewards to a specific set of preselected collaborators. These offerings are the Offers. The DoC for the rewards will be locked in the contract upfront.

The collaborators will publish their messages through their nostr accounts with the exact requested content, and will get paid automatically after they've run the message for 14 days.

The Collaborators must publish the message before a certain date for the Advertiser to see.
If the Advertiser cannot see the published message they can challenge the collaborator to submit proof. 

In case of a challenge, the Collaborator has 24 hours to submit proof of publication.
If they fail to submit proof of publication, their payment is aborted
(and they're obviously free to delete the message if they've actually published it).

The collaborator must refrain from deleting the message for 14 days, the duration of the campaign.
If an early deletion is detected, the Advertiser may submit proof of deletion which will abort the payment, and will incurr in a penalty for the collaborator.

If the collaborator must delete the message earlier than expected,
they can voluntarily renounce their payment beforehand to avoid the penalty.

For convenience, the Advertiser can undo its challenge and always pay the collaborator.
This is only for convenience, since the Advertiser can always unilateraly send money
to the collaborator reward address outside the contract scope.

Submitting proofs to the contract is costly (~4 usd), so rewards below that amount
make conflict resolution impractical for collaborators. 

Any rewards that are not awarded to collaborators can be refunded to the Advertiser's address.
*/

contract Asami is Ownable {
    event DebugLog(string message, uint256 number);

    enum NostrOfferState {
      // The offer is assumed to be published.
      Assumed,
      // The advertiser has challenged the message publication.
      Challenged, 
      // The collaborator has submitted proof of the message.
      Confirmed,
      // The collaborator has renounced the reward, probably intending to delete the message.
      Renounced,
      // The advertiser has reported the publication has been deleted before the campaign ended.
      DeletionConfirmed,
    }

    modifier inNostrState(NostrOfferState _state) {
        require(state == _state, "Invalid state for this action");
        _;
    }

    /*
      The NewNostrOffer struct is meant to be used by an advertiser when creating a campaign,
      to set the reward amount, the account it will be paid to, and the nostr address of who can claim the content.
      There's a separate representation for the offer once the campaign is created.
    */
    struct NewNostrOffer {
        uint256 rewardAmount;
        address rewardAddress;
        string nostrHexPubkey;
        uint256 nostrAffineX; // The signer's pubkey X coord, must match their nostr pubkey.
        uint256 nostrAffineY; // The signer's pubkey Y.
    }

    /* An offer pointer represents a specific offer made in a campaign */
    struct OfferPointer {
        uint campaignId;
        uint offerId;
    }

    /* The message proof has the required values to rebuild a Nostr message and check its signature */
    struct NostrMessageProof {
        uint256 createdAt;
        uint256 sigR; // The first 32 bytes of the signature.
        uint256 sigS; // The other 32 bytes of the signature.
    }

    struct NostrDeletionProof {
        string content;
        string tags;
        uint256 createdAt;
        uint256 sigR; // The first 32 bytes of the signature.
        uint256 sigS; // The other 32 bytes of the signature.
    }

    /* Represents the offer's state within a stored campaign */
    struct NostrOffer {
        string nostrHexPubkey;
        uint256 rewardAmount;
        address rewardAddress;
        bool awarded;
        bool collected;
        uint256 nostrAffineX;
        uint256 nostrAffineY;
        bytes32 messageId;
        bool renounced;
    }

    struct NostrCampaign {
        string content;
        uint256 funding;
        address creator;
        uint256 deadline;
        uint offerCount;
        mapping(uint => NostrOffer) offers;
    }

    enum LegacyOfferState {
      // The offer is assumed to be published.
      Assumed,
      // The advertiser has challenged the message publication.
      Challenged, 
      // The collaborator (or someone else) has paid the oracle to confirm publication.
      ConfirmationRequested,
      // The oracle has confirmed the message has been published.
      Confirmed,
      // The oracle could not confirm the message publication. The offer will be refunded.
      Refuted,
      // The oracle failed to respond to the confirmation request.
      // The Collaborator gets his money back from the oracle, but the reward is not awarded.
      // Anyone can trigger the transition to this state.
      OracleConfirmationDefaulted,
      // The collaborator has renounced 
      Renounced,
      // The advertiser has reported the publication has been deleted before the campaign ended.
      DeletionReported,
      // The oracle has confirmed the publication was deleted early. The reward will not be paid. The collaborator will be penalized.
      DeletionConfirmed,
      // The oracle did not respond to the deletion request.
      // The payment is prorated, up to the date when the deletion was reported.
      OracleDeletionConfirmationDefaulted,
    }

    modifier inLegacyState(LegacyOfferState _state) {
        require(state == _state, "Invalid state for this action");
        _;
    }

    struct NewLegacyOffer {
        uint256 rewardAmount;
        address rewardAddress;
        string username;
    }

    struct LegacyOffer {
        string username;
        uint256 rewardAmount;
        address rewardAddress;
        bool awarded;
        bool collected;
        string publicationId;
        bool renounced;
    }

    struct LegacyCampaign {
        uint socialNetworkId;
        string description;
        address oracleAddress;
        uint256 oracleFee;
        uint256 funding;
        address creator;
        uint256 deadline;
        uint offerCount;
        mapping(uint => NostrOffer) offers;
    }

    uint constant challengeWindow = 60 * 60 * 24; // A day
    uint constant proofWindow = challengeWindow * 2;
    uint constant campaignDuration = challengeWindow * 14;

    uint256 public feePerOffer = 1e18;
    uint256 public maxFeePerCampaign = 50e18 ;
    uint256 public fees = 0;
    address public feesAddress;

    uint public nostrCampaignCount;
    mapping(uint => NostrCampaign) public nostrCampaigns;

    uint public legacySocialNetworksCount;
    mapping(uint => string) public legacySocialNetworks;

    uint public legacyCampaignCount;
    mapping(uint => LegacyCampaign) public legacyCampaigns;

    struct DeletionPenalty {
        address payable creditor;
        uint256 amount;
    }
    mapping(address => DeletionPenalty[]) public deletionPenalties;

    IERC20 public rewardToken;
    
    constructor(address _dollarOnChainAddress) {
        rewardToken = IERC20(_dollarOnChainAddress);
    }

    function createLegacyCampaign(string memory _description, uint256 _funding, uint256 _deadline, NewLegacyOffer[] memory _offers) public returns (uint) {
        require(_funding > 0, "amount_must_be_greater_than_zero");
        require(_offers.length > 0, "must_set_offers");

        LegacyCampaign storage campaign = legacyCampaigns[legacyCampaignCount];
        campaign.description = _description;
        campaign.funding = _funding;
        campaign.creator = payable(msg.sender);
        campaign.deadline = _deadline;

        uint256 totalRewards = 0;
        uint256 totalFees = 0;
        for (uint i = 0; i < _offers.length; i++) {
            NewLegacyOffer memory source = _offers[i];
            require(source.rewardAmount > 0, "collaborator_must_get_paid");
            totalRewards += source.rewardAmount;
            totalFees += feePerOffer;
            campaign.offers[i] = NostrOffer({
                username: source.username,
                rewardAmount: source.rewardAmount,
                rewardAddress: payable(source.rewardAddress),
                collected: false,
                awarded: true,
                publicationId: source.publicationId,
                renounced: false
            });
            campaign.offerCount = i + 1;
        }

        if (totalFees > maxFeePerCampaign) {
          totalFees = maxFeePerCampaign;
        }

        fees += totalFees;

        require((totalRewards + totalFees) == _funding, "funding_must_match_rewards_amount_exactly");
        require(rewardToken.transferFrom(msg.sender, address(this), _funding), "contract_reward_funding_failed");
        
        return legacyCampaignCount++;
    }

    function createNostrCampaign(string memory _content, uint256 _funding, uint256 _deadline, NewNostrOffer[] memory _offers) public returns (uint) {
        require(_funding > 0, "amount_must_be_greater_than_zero");
        require(_offers.length > 0, "must_set_offers");

        NostrCampaign storage campaign = nostrCampaigns[nostrCampaignCount];
        campaign.content = _content;
        campaign.funding = _funding;
        campaign.creator = payable(msg.sender);
        campaign.deadline = _deadline;

        uint256 totalRewards = 0;
        uint256 totalFees = 0;
        for (uint i = 0; i < _offers.length; i++) {
            NewNostrOffer memory source = _offers[i];
            require(source.rewardAmount > 0, "collaborator_must_get_paid");
            totalRewards += source.rewardAmount;
            totalFees += feePerOffer;
            campaign.offers[i] = NostrOffer({
                nostrHexPubkey: source.nostrHexPubkey,
                rewardAmount: source.rewardAmount,
                rewardAddress: payable(source.rewardAddress),
                collected: false,
                awarded: true,
                nostrAffineX: source.nostrAffineX,
                nostrAffineY: source.nostrAffineY,
                messageId: bytes32(0),
                renounced: false
            });
            campaign.offerCount = i + 1;
        }

        if (totalFees > maxFeePerCampaign) {
          totalFees = maxFeePerCampaign;
        }

        fees += totalFees;

        require((totalRewards + totalFees) == _funding, "funding_must_match_rewards_amount_exactly");
        require(rewardToken.transferFrom(msg.sender, address(this), _funding), "contract_reward_funding_failed");
        
        return nostrCampaignCount++;
    }

    function legacyChallenge(OfferPointer calldata _p) public {
        LegacyCampaign storage campaign = legacyCampaigns[_p.campaignId];
        require(campaign.creator == msg.sender, "only_campaign_owner_can_challenge");

        require(block.timestamp <= (campaign.deadline + challengeWindow), "cannot_challenge_after_24_of_deadline");
        NostrOffer storage offer = campaign.offers[_p.offerId];
        require(offer.rewardAmount > 0, "no_such_offer");
        offer.awarded = false;
    }

    function nostrChallenge(OfferPointer calldata _p) public {
        NostrCampaign storage campaign = nostrCampaigns[_p.campaignId];
        require(campaign.creator == msg.sender, "only_campaign_owner_can_challenge");

        require(block.timestamp <= (campaign.deadline + challengeWindow), "cannot_challenge_after_24_of_deadline");
        NostrOffer storage offer = campaign.offers[_p.offerId];
        require(offer.rewardAmount > 0, "no_such_offer");
        offer.awarded = false;
    }

    function requestPostAttestation(OfferPointer calldata _p) public {
        LegacyCampaign storage campaign = legacyCampaigns[_p.campaignId];
        require(block.timestamp <= (campaign.deadline + campaignDuration), "campaign_already_ended");
        require(rewardToken.transferFrom(msg.sender, address(this), campaign.oracleFee), "could_not_pay_oracle_fee");
        NostrOffer storage offer = campaign.offers[_p.offerId];
        require(offer.rewardAmount > 0, "no_such_offer");
        offer.attestationRequested = true
    }

    function submitPostAttestation(OfferPointer calldata _p, bool found) public {
        // Can only be called by the proposed oracle.
        // Gets paid.
    }

    function requestPostDeletionAttestation(OfferPointer calldata _p) public {
        // Can be called by anyone.
        // Pays the oracle fee.
    }

    function submitPostDeletionAttestation(OfferPointer calldata _p) public {
        // ...
    }

    // Anyone can submit proof of a message being published, likely a Collaborator or someone on their behalf.
    // The r value is the first 32 bytes of the signature, and the s value are the remaining 32,
    function submitNostrProof(
      OfferPointer calldata _p,
      NostrMessageProof calldata _proof
    ) 
      public
    {
        NostrCampaign storage campaign = nostrCampaigns[_p.campaignId];
        require(block.timestamp <= (campaign.deadline + proofWindow), "cannot_send_proof_48_hs_after_deadline");

        NostrOffer storage offer = campaign.offers[_p.offerId];
        require(offer.rewardAmount > 0, "no_such_offer");

        require(!offer.awarded, "offer_will_be_paid_no_need_for_proof_yet");

        bytes32 messageId = verifyNostrCampaignMessage(
          campaign.content,
          offer.nostrHexPubkey,
          _proof,
          offer.nostrAffineX,
          offer.nostrAffineY
        );

        offer.awarded = true;
        offer.messageId = messageId;
    }

    // Submitting a deletion proof is very costly on the advertiser, so if it comes to that,
    // the collaborator gets penalized and is banned from the system until he pays for the penalties.
    // To reduce gas usage, this method does not parse the tags json, it just does a raw string search,
    // which initially may be enough.
    // The r value is the first 32 bytes of the signature, and the s value are the remaining 32,
    function submitNostrDeletionProof(
        OfferPointer calldata _p,
        NostrDeletionProof calldata _deletionProof
    )
        public
    {
        uint256 startGas = gasleft();

        NostrCampaign storage campaign = nostrCampaigns[_p.campaignId];
        require(block.timestamp <= (campaign.deadline + campaignDuration), "campaign_already_ended");

        NostrOffer storage offer = campaign.offers[_p.offerId];
        require(offer.awarded, "no_need_as_offer_is_not_awarded");
        require(offer.messageId > bytes32(0), "offer_proof_has_not_been_submitted");

        require(idInTags(offer.messageId, _deletionProof.tags), "message_id_not_found_in_tags");

        verifyDeletionMessage(
          offer.nostrHexPubkey,
          _deletionProof,
          offer.nostrAffineX,
          offer.nostrAffineY
        );

        offer.awarded = false;

        uint256 refund = ((startGas - gasleft()) * 11) / 10;
        deletionPenalties[offer.rewardAddress].push(DeletionPenalty(payable(msg.sender), refund));
    }

    function submitNostrProofAndDeletionProof (
        OfferPointer calldata _p,
        NostrMessageProof calldata _messageProof,
        NostrDeletionProof calldata _deletionProof
    )
        public 
    {
        uint256 startGas = gasleft();

        NostrCampaign storage campaign = nostrCampaigns[_p.campaignId];
        require(block.timestamp <= (campaign.deadline + campaignDuration), "campaign_already_ended");

        NostrOffer storage offer = campaign.offers[_p.offerId];
        require(offer.awarded, "no_need_as_offer_is_not_awarded");

        bytes32 messageId = verifyNostrCampaignMessage(
            campaign.content,
            offer.nostrHexPubkey,
            _messageProof,
            offer.nostrAffineX,
            offer.nostrAffineY
        );

        offer.messageId = messageId;

        require(idInTags(offer.messageId, _deletionProof.tags), "message_id_not_found_in_tags");

        verifyDeletionMessage(
          offer.nostrHexPubkey,
          _deletionProof,
          offer.nostrAffineX,
          offer.nostrAffineY
        );

        offer.awarded = false;

        uint256 refund = ((startGas - gasleft()) * 11) / 10;
        deletionPenalties[offer.rewardAddress].push(DeletionPenalty(payable(msg.sender), refund));
    }

    function payDeletionPenalties(address _debtor) public payable {
        DeletionPenalty[] storage penalties = deletionPenalties[_debtor];
        uint256 total = 0;

        for (uint256 i = 0; i < penalties.length; i++) {
            total += penalties[i].amount;
        }

        require(total > 0, "no_penalties_to_pay");

        require(msg.value != total, "value_and_penalty_amount_mismatch");

        for (uint256 i = 0; i < penalties.length; i++) {
            payable(penalties[i].creditor).transfer(penalties[i].amount);
        }

        delete deletionPenalties[_debtor];
    }

    /*
      This function is usually called by the Collaborator to collect outstanding payments
      on campaigns they have participated, but it may be called by just anyone to pay the collaborator.
      The caller must provide a known list of offers that are ready to be collected.
      The list can be built by calling the getCollectableOffers function in the contract.
      The list is passed in to save gas. No point in walking over and evaluating
      offers that are not ready to collect, or never will, which may even be spam.
    */
    function collectRewards(address _collaborator, OfferPointer[] calldata _offers) public {
        require(deletionPenalties[_collaborator].length == 0, "you_need_to_pay_your_penalties_before_you_collect_rewards");

        uint256 totalRewards = 0;
        for (uint i = 0; i < _offers.length; i++) {
            OfferPointer memory p = _offers[i];

            NostrCampaign storage campaign = nostrCampaigns[p.campaignId];
            require(campaign.funding > 0, "no_such_campaign");
            require(block.timestamp >= (campaign.deadline + campaignDuration), "campaign_is_not_over_yet");

            NostrOffer storage offer = campaign.offers[p.offerId];
            require(offer.rewardAmount > 0, "no_such_offer");
            require(offer.awarded, "this_offers_reward_was_not_awarded");
            require(offer.rewardAddress == _collaborator, "that_offer_is_not_for_this_collaborator");

            require(!offer.collected, "already_collected");
            totalRewards = offer.rewardAmount;
            offer.collected = true;
        }

        require(rewardToken.transfer(_collaborator, totalRewards), "reward_payment_failed");
    }

    /*
      This function is called by the Advertiser to collect refunds from non-awarded offers.
      It may also be called by third parties to give the refunds to the Advertiser.
      The Advertiser must provide a known list of offers that are ready to be refunded.
      The list can be built by calling the getCollectableRefunds function in the contract.
    */
    function collectRefunds(address payable _advertiser, OfferPointer[] calldata _offers) public {
        uint256 totalRefund = 0;

        for (uint i = 0; i < _offers.length; i++) {
            OfferPointer memory p = _offers[i];

            NostrCampaign storage campaign = nostrCampaigns[p.campaignId];
            require(campaign.funding > 0, "no_such_campaign");
            require(campaign.creator == _advertiser, "this_campaign_does_not_belong_to_that_advertiser");

            require(block.timestamp >= (campaign.deadline + proofWindow), "not_refundable_yet");

            NostrOffer storage offer = campaign.offers[p.offerId];
            require(offer.rewardAmount > 0, "no_such_offer");
            require(!offer.awarded, "awarded_offers_are_not_refundable");

            require(!offer.collected, "already_collected");
            totalRefund += offer.rewardAmount;
            offer.collected = true;
        }

        require(rewardToken.transfer(_advertiser, totalRefund), "refund_payment_failed");
    }

    /*
     The campaign creator can manually set a reward as payable.
     This is meant to be used when the advertiser mistakenly challenges.
     It's just for convenience, as the advertiser may contain 
      if the advertiser is willing to pay, they can always send money bypassing the contract.
     This works unless the reward has been collected or refunded.
    */
    function forceReward(OfferPointer calldata _p) public {
        NostrCampaign storage campaign = nostrCampaigns[_p.campaignId];
        require(campaign.creator == msg.sender, "only_campaign_owner_can_force_rewards");
        NostrOffer storage offer = campaign.offers[_p.offerId];
        require(offer.rewardAmount > 0, "no_such_offer");
        require(!offer.renounced, "offer_was_renounced_by_collaborator");
        offer.awarded = true;
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
    function renonuce(OfferPointer calldata _p) public {
        NostrCampaign storage campaign = nostrCampaigns[_p.campaignId];
        NostrOffer storage offer = campaign.offers[_p.offerId];
        require(offer.rewardAmount > 0, "no_such_offer");
        require(offer.rewardAddress == msg.sender, "only_collaborator_can_force_rewards");
        offer.awarded = false;
        offer.renounced = true;
    }

    function verifyNostrCampaignMessage(
        string memory _content,
        string memory _pubkey,
        NostrMessageProof memory _proof,
        uint256 _affineX,
        uint256 _affineY
    )
        public pure returns (bytes32)
    {
        bytes memory message = abi.encodePacked(
            "[0,\"",
            _pubkey,
            "\",",
            Strings.toString(_proof.createdAt),
            ",1,[],\"",
            _content,
            "\"]"
        );

        bytes32 messageId = sha256(message);

        require(Schnorr.verify(
            messageId,
            _affineX,
            _affineY,
            _proof.sigR,
            _proof.sigS
        ), "invalid_proof");

        return messageId;
    }

    function verifyDeletionMessage(
        string memory _pubkey,
        NostrDeletionProof memory _proof,
        uint256 _affineX,
        uint256 _affineY
    )
        public pure
    {
        bytes memory message = abi.encodePacked(
            "[0,\"",
            _pubkey,
            "\",",
            Strings.toString(_proof.createdAt),
            ",5,",
            _proof.tags,
            ",\"",
            _proof.content,
            "\"]"
        );

        require(Schnorr.verify(
            sha256(message),
            _affineX,
            _affineY,
            _proof.sigR,
            _proof.sigS
        ), "invalid_deletion_proof");
    }

    function setFeePerOffer(uint256 _fee) public onlyOwner {
      feePerOffer = _fee;
    }

    function setMaxFeePerCampaign(uint256 _fee) public onlyOwner {
      maxFeePerCampaign = _fee;
    }

    function setFeesAddress(address _feesAddress) public onlyOwner {
        feesAddress = _feesAddress;
    }

    function collectFees() public {
        require(fees > 0, "no_fee_to_collect");
        require(rewardToken.transferFrom(address(this), feesAddress, fees), "could_not_pay_fees");
        fees = 0;
    }

    function getNostrCampaignOffer(OfferPointer memory _p) public view returns (NostrOffer memory) {
        return nostrCampaigns[_p.campaignId].offers[_p.offerId];
    }

    function getRewardsPendingCollection(address _collaborator)
        public view returns (OfferPointer[] memory) 
    {
        OfferPointer[] memory pointers = new OfferPointer[](0);

        for(uint i = 0; i < nostrCampaignCount; i++) {
          for(uint j = 0; j < nostrCampaigns[i].offerCount; j++) {
            NostrOffer memory offer = nostrCampaigns[i].offers[j];
            if(offer.rewardAddress == _collaborator && offer.awarded && !offer.collected) {
              OfferPointer[] memory tmp = new OfferPointer[](pointers.length + 1);
              for (uint k = 0; k < pointers.length; k++) {
                  tmp[k] = pointers[k];
              }
              tmp[pointers.length] = OfferPointer(i,j);
              pointers = tmp;
            }
          }
        }
        return pointers;
    }

    function getRefundableOffers(address _advertiser)
        public view returns (OfferPointer[] memory) 
    {
        OfferPointer[] memory pointers = new OfferPointer[](0);

        for(uint i = 0; i < nostrCampaignCount; i++) {
          if( nostrCampaigns[1].creator != _advertiser ){
            continue;
          }
          for(uint j = 0; j < nostrCampaigns[i].offerCount; j++) {
            NostrOffer memory offer = nostrCampaigns[i].offers[j];
            if(!offer.awarded && !offer.collected) {
              OfferPointer[] memory tmp = new OfferPointer[](pointers.length + 1);
              for (uint k = 0; k < pointers.length; k++) {
                  tmp[k] = pointers[k];
              }
              tmp[pointers.length] = OfferPointer(i,j);
              pointers = tmp;
            }
          }
        }
        return pointers;
    }

    function getCampaignFees(uint _offerCount) public view returns (uint256) {
        uint256 totalFees = _offerCount * feePerOffer;

        if (totalFees > maxFeePerCampaign) {
          totalFees = maxFeePerCampaign;
        }

        return totalFees;
    }

    function idInTags(
        bytes32 needle,
        string memory haystack
    )
        public pure returns (bool)
    {
        string memory needleHex = Strings.toHexString(uint256(needle));
        bytes memory needleHexBytes = bytes(needleHex);
        bytes memory haystackBytes = bytes(haystack);

        for(uint i = 0; i < haystackBytes.length - needleHexBytes.length + 1; i++) {
            bool found = true;
            for(uint j = 0; j < needleHexBytes.length - 2; j++) {
                if(haystackBytes[i + j] != needleHexBytes[j+2]) {
                    found = false;
                    break;
                }
            }
            if(found) {
                return true;
            }
        }
        return false;
    }
}
