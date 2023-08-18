// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;
import "@openzeppelin/contracts/utils/Strings.sol";

library AsamiClassic {
    enum OfferState {
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

      // The collaborator has renounced 
      Renounced,

      // The advertiser has reported the publication has been deleted before the campaign ended.
      // The oracle will either confirm the deletion or will move the offer back to the confirmed state.
      DeletionReported,

      // The oracle has confirmed the publication was deleted early. The reward will not be paid. The collaborator will be penalized.
      DeletionConfirmed
    }

    struct NewOffer {
        uint256 rewardAmount;
        address rewardAddress;
        string username;
    }

    struct Offer {
        OfferState state;
        string username;
        uint256 rewardAmount;
        address rewardAddress;
        string publicationUrl;
        uint deletionReportedAt;
        bool advertiserCollected;
        bool collaboratorCollected;
        bool oracleCollected;
    }

    struct Terms {
        uint socialNetworkId;
        string rulesUrl;
        bytes32 rulesHash;
        address oracleAddress;
        uint256 oracleFee;
        Offer[] offers;
    }
}
