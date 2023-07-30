// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;
import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "./Schnorr.sol";

/*
This smart contract regulates the relationship between a company willing to run an ad, the "Advertiser",
an and one or several indpendent collaborators with a significant following, the "collaborators".

The Advertiser will create a campaign, offering DoC rewards to a specific set of preselected collaborators.
The DoC for the rewards will be locked in the contract upfront.

The collaborators will publish their messages through their nostr accounts with the exact requested content,
and will get paid automatically after they've run the message for 14 days.

The Collaborators must publish the message before a certain date for the Advertiser to see.
If the Advertiser cannot see the published message they can challenge the collaborator to submit proof. 

In case of a challenge, the Collaborator has 24 hours to submit proof of publication.
If they fail to submit proof of publication, their payment is aborted
(and they're obviously free to delete the message if they've actually published it).

The collaborator must refrain from deleting the message for 14 days, the duration of the campaign.
If an early deletion is detected, the Advertiser may submit proof of deletion which will abort the payment,
and will incurr in a penalty for the collaborator.

If the collaborator must delete the message earlier than expected,
they can voluntarily renounce their payment beforehand to avoid the penalty.

For convenience, the Advertiser can undo its challenge and always pay the collaborator.
This is only for convenience, since the Advertiser can always unilateraly send money
to the collaborator reward address outside the contract scope.

Submitting proofs to the contract is costly (~4 usd), so rewards below that amount
make conflict resolution impractical for collaborators. 

Any rewards that are not awarded to collaborators can be refunded to the Advertiser's address.
*/

contract NostrAds {
    /*
      The NewOffer struct is meant to be used by an advertiser when creating a campaign,
      to set the reward amount, the account it will be paid to, and the nostr address of who can claim the content.
      There's a separate representation for the offer once the campaign is created.
    */
    struct NewOffer {
        string nostrHexPubkey;
        uint256 rewardAmount;
        address rewardAddress;
    }

    struct OfferPointer {
        uint campaignId;
        uint offerId;
    }

    /* Represents the offer's state within a stored campaign */
    struct Offer {
        string nostrHexPubkey;
        uint256 rewardAmount;
        address payable rewardAddress;
        bool awarded;
        bool collected;
        uint256 nostrAffineX;
        uint256 nostrAffineY;
        bytes32 messageId;
    }

    struct Campaign {
        string content;
        uint256 funding;
        address payable creator;
        uint256 deadline;
        uint offerCount;
        mapping(uint => Offer) offers;
    }

    uint constant challengeWindow = 60 * 60 * 24; // A day
    uint constant proofWindow = challengeWindow * 2;
    uint constant campaignDuration = challengWindow * 14;

    mapping(uint => Campaign) public campaigns;

    struct DeletionPenalty {
        address payable creditor;
        uint256 amount;
    }
    mapping(address => DeletionPenalty[]) public deletionPenalties;

    uint public campaignCount;

    IERC20 public rewardTokenContract;
    
    constructor(address _rewardTokenAddress) {
        rewardTokenContract = IERC20(_rewardTokenAddress);
    }

    function createCampaign(string memory _content, uint256 _funding, uint256 _deadline, NewCollaborator[] memory _collaborators) public returns (uint) {
        require(_funding > 0, "amount_must_be_greater_than_zero");
        require(_collaborators.length > 0, "must_set_collaborators");

        Campaign storage campaign = campaigns[campaignCount];
        campaign.content = _content;
        campaign.funding = _funding;
        campaign.creator = payable(msg.sender);
        campaign.deadline = _deadline;
        campaign.payoutDate = _payoutDate;
        campaign.payoutsDone = false;

        uint256 totalRewards = 0;
        for (uint i = 0; i < _collaborators.length; i++) {
            NewCollaborator memory source = _collaborators[i];
            require(source.rewardAmount > 0, "collaborator_must_get_paid");
            totalRewards += source.rewardAmount;
            campaign.offers[i] = Offer({
                nostrHexPubkey: source.nostrHexPubkey,
                rewardAmount: source.rewardAmount,
                rewardAddress: payable(source.rewardAddress),
                collected: false,
                awarded: true
            });
            campaign.collaboratorCount = i + 1;
        }

        require(totalRewards == _funding, "funding_must_match_rewards_amount_exactly");
        require(rewardTokenContract.transferFrom(msg.sender, address(this), _funding), "contract_reward_funding_failed");
        
        return campaignCount++;
    }

    function challenge(OfferPointer _pointer) public {
        Campaign storage campaign = campaigns[pointer.campaignId];
        require(campaign.creator == msg.sender, "only_campaign_owner_can_challenge");

        require(block.timestamp <= (campaign.deadline + challengeWindow), "cannot_challenge_after_24_of_deadline");
        Offer storage offer = campaign.offers[_pointer.offerId];
        require(offer.rewardAmount > 0, "no_such_offer");
        offer.awarded = false;
    }

    // Anyone can submit proof of a message being published, likely a Collaborator or someone on their behalf.
    // The r value is the first 32 bytes of the signature, and the s value are the remaining 32,
    function submitProof(
      OfferPointer _pointer,
      uint256 _createdAt,
      uint256 _r,
      uint256 _s,
      uint256 _affineX,
      uint256 _affineY
    ) 
      public
    {
        Campaign storage campaign = campaigns[_pointer._campaignId];
        require(block.timestamp <= (campaign.deadline + proofWindow), "cannot_send_proof_48_hs_after_deadline");

        Offer storage offer = campaign.offers[_pointer.offerId];
        require(offer.rewardAmount > 0, "no_such_offer");

        require(!offer.awarded, "offer_will_be_paid_no_need_for_proof_yet");

        bytes32 messageId = verifyCampaignMessage(
          campaign.content,
          offer.nostrHexPubkey,
          _createdAt,
          _r,
          _s,
          _affineX,
          _affineY
        );

        offer.awarded = true;
        offer.messageId = messageId;
    }

    // Submitting a deletion proof is very costly on the advertiser, so if it comes to that,
    // the collaborator gets penalized and is banned from the system until he pays for the penalties.
    // To reduce gas usage, this method does not parse the tags json, it just does a raw string search,
    // which initially may be enough.
    // The r value is the first 32 bytes of the signature, and the s value are the remaining 32,
    function submitDeletionProof(
        OfferPointer _pointer,
        string _content,
        string _tags,
        uint256 _createdAt,
        uint256 _r, 
        uint256 _s,
        uint256 _affineX,
        uint256 _affineY
    )
        public
    {
        uint256 startGas = gasleft();

        Campaign storage campaign = campaigns[_pointer._campaignId];
        require(block.timestamp <= (campaign.deadline + campaignDuration), "campaign_already_ended");

        Offer storage offer = campaign.offers[_pointer.offerId];
        require(offer.awarded, "no_need_as_offer_is_not_awarded");
        require(offer.messageId > bytes32(0), "offer_proof_has_not_been_submitted");
        require(strFind(offer.messageId, _tags), "message_id_not_found_in_tags");

        verifyDeletionMessage(
          offer.nostrHexPubkey,
          _content,
          _tags,
          _createdAt,
          _r,
          _s,
          _affineX,
          _affineY,
        );

        collaborator.awarded = false;

        uint256 refund = (startGas - gasleft()) * 1.1;
        deletionPenalties[offer.rewardAddress].push(DeletionPenalty(campaign.creator, refund));
    }

    function submitProofAndDeletionProof (
        OfferPointer _pointer,
        uint256 _msgCreatedAt,
        uint256 _msgR,
        uint256 _msgS,
        string _deletionContent,
        string _deletionTags,
        uint256 _deletionCreatedAt,
        uint256 _deletionR, 
        uint256 _deletionS,
        uint256 _affineX,
        uint256 _affineY
    )
        public 
    {
        uint256 startGas = gasleft();

        Campaign storage campaign = campaigns[_pointer._campaignId];
        require(block.timestamp <= (campaign.deadline + campaignDuration), "campaign_already_ended");

        Offer storage offer = campaign.offers[_pointer.offerId];
        require(offer.awarded, "no_need_as_offer_is_not_awarded");

        bytes32 messageId = verifyCampaignMessage(
          campaign.content,
          offer.nostrHexPubkey,
          _msgCreatedAt,
          _msgR,
          _msgS,
          _affineX,
          _affineY
        );

        offer.messageId = messageId;

        require(strFind(offer.messageId, _tags), "message_id_not_found_in_tags");

        verifyDeletionMessage(
          offer.nostrHexPubkey,
          _deletionContent,
          _deletionTags,
          _deletionCreatedAt,
          _deletionR,
          _deletionS,
          _affineX,
          _affineY,
        );

        collaborator.awarded = false;

        uint256 refund = (startGas - gasleft()) * 1.1;
        deletionPenalties[offer.rewardAddress].push(DeletionPenalty(campaign.creator, refund));
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

        delete penalties;
    }

    /*
      This function is usually called by the Collaborator to collect outstanding payments
      on campaigns they have participated, but it may be called by just anyone to pay the collaborator.
      The caller must provide a known list of offers that are ready to be collected.
      The list can be built by calling the getCollectableOffers function in the contract.
      The list is passed in to save gas. No point in walking over and evaluating
      offers that are not ready to collect, or never will, which may even be spam.
    */
    function collectRewards(address payable _collaborator, OfferPointer[] _offers) public {
        uint256 totalRewards = 0;
        for (uint i = 0; i < _offers.length; i++) {
            OfferPointer p = _offers[i];

            Campaign storage campaign = campaigns[p._campaignId];
            require(campaign.funding > 0, "no_such_campaign");
            require(block.timestamp >= (campaign.deadline + campaignDuration), "campaign_is_not_over_yet");

            Offer storage offer = campaign.offers[_pointer.offerId];
            require(offer.rewardAmount > 0, "no_such_offer");
            require(offer.awarded, "this_offers_reward_was_not_awarded");
            require(offer.rewardAddress == _collaborator, "this_offer_is_not_for_collaborator");

            require(!offer.collected, "already_collected");
            totalRewards = offer.rewardAmount;
            offer.collected = true;
        }

        require(rewardTokenContract.transfer(_collaborator, totalRewards), "reward_payment_failed");
    }

    /*
      This function is called by the Advertiser to collect refunds from non-awarded offers.
      It may also be called by third parties to give the refunds to the Advertiser.
      The Advertiser must provide a known list of offers that are ready to be refunded.
      The list can be built by calling the getCollectableRefunds function in the contract.
    */
    function collectRefunds(address payable _advertiser, OfferPointer[] _offers) public {
        uint256 totalRefund = 0;
        for (uint i = 0; i < _offers.length; i++) {
            OfferPointer p = _offers[i];

            Campaign storage campaign = campaigns[p._campaignId];
            require(campaign.funding > 0, "no_such_campaign");
            require(campaign.creator == _advertiser, "non_advertiser_campaign");

            require(block.timestamp >= (campaign.deadline + proofWindow), "not_refundable_yet");

            Offer storage offer = campaign.offers[_pointer.offerId];
            require(offer.rewardAmount > 0, "no_such_offer");
            require(!offer.awarded, "awarded_offers_are_not_refundable");

            require(!offer.collected, "already_collected");
            totalRefund = offer.rewardAmount;
            offer.collected = true;
        }

        require(rewardTokenContract.transfer(_advertiser, totalRefund), "refund_payment_failed");
    }

    // The campaign creator can manually set a reward as payable.
    // This is meant to be used when the advertiser mistakenly challenges.
    // It's just for convenience, as the advertiser may contain 
    //  if the advertiser is willing to pay, they can always send money bypassing the contract.
    // This works unless the reward has been collected or refunded.
    function forceReward(OfferPointer p) public {
      //pending
    }

    // The collaborator can always force a refund.
    // If the collaborator deletes a message, the advertiser can submit proof of deletion to prevent payment from happening.
    // Submitting the proof of deletion for a message may be very costly for the advertiser, so when this happens
    // a penalty is applied on the collaborator in favor of the advertiser.
    // To prevent this penalty, the collaborator may forfeit the reward voluntarily before deleting the message.
    function forceRefund(OfferPointer p) public {
      //pending
    }


    function getCampaignCollaborator(uint _a, uint _b) public view returns (Collaborator memory) {
        return campaigns[_a].collaborators[_b];
    }

    function verifyCampaignMessage(
        string _content,
        string _pubkey,
        uint256 _createdAt,
        uint256 _r,
        uint256 _s,
        uint256 _affineX,
        uint256 _affineY
    )
        public pure returns (bytes32)
    {
        bytes memory message = abi.encodePacked(
            "[0,\"",
            collaborator.nostrHexPubkey,
            "\",",
            Strings.toString(_createdAt),
            ",1,[],\"",
            campaign.content,
            "\"]"
        );

        bytes memory messageId = sha256(message);

        require(Schnorr.verify(
            messageId,
            _affineX,
            _affineY,
            _r,
            _s
        ), "invalid_proof");

        return messageId;
    }

    function verifyDeletionMessage(
        string _pubkey,
        string _content,
        string _tags,
        uint256 _createdAt,
        uint256 _r, 
        uint256 _s,
        uint256 _affineX,
        uint256 _affineY
    )
        public pure
    {
        bytes memory message = abi.encodePacked(
            "[0,\"",
            _pubkey,
            "\",",
            Strings.toString(_createdAt),
            ",3,",
            _tags,
            ",\"",
            _content,
            "\"]"
        );

        require(Schnorr.verify(
            sha256(message),
            _affineX,
            _affineY,
            _r,
            _s
        ), "invalid_deletion_proof");
    }

    function strFind(bytes32 memory needle, string memory haystack) public pure returns (bool) {
        string memory needleHex = Strings.toHex(needle, 32); // Or is it 64
        bytes memory needleHexBytes = bytes(needleHex);
        bytes memory haystackBytes = bytes(haystack);

        for(uint i = 0; i < haystackBytes.length - needleHexBytes.length + 1; i++) {
            bool found = true;
            for(uint j = 0; j < needleHexBytes.length; j++) {
                if(haystackBytes[i + j] != needleHexBytes[j]) {
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
