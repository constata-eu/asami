// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;
import "@openzeppelin/contracts/utils/Strings.sol";
import "./Schnorr.sol";

library AsamiNostr {
    enum OfferState {
      // The offer is assumed to be published.
      Assumed,

      // The advertiser has challenged the message publication.
      Challenged, 

      // The collaborator has submitted proof of the message.
      Confirmed,

      // The collaborator has renounced the reward, probably intending to delete the message.
      Renounced,

      // The advertiser has reported the publication has been deleted before the campaign ended.
      ReportedDeletion
    }

    /*
      The NewNostrOffer struct is meant to be used by an advertiser when creating a campaign,
      to set the reward amount, the account it will be paid to, and the nostr address of who can claim the content.
      There's a separate representation for the offer once the campaign is created.
    */
    struct NewOffer {
        uint256 rewardAmount;
        address rewardAddress;
        string nostrHexPubkey;
        uint256 nostrAffineX; // The signer's pubkey X coord, must match their nostr pubkey.
        uint256 nostrAffineY; // The signer's pubkey Y.
    }

    /* The message proof has the required values to rebuild a Nostr message and check its signature */
    struct MessageProof {
        uint256 createdAt;
        uint256 sigR; // The first 32 bytes of the signature.
        uint256 sigS; // The other 32 bytes of the signature.
    }

    struct DeletionProof {
        string content;
        string tags;
        uint256 createdAt;
        uint256 sigR; // The first 32 bytes of the signature.
        uint256 sigS; // The other 32 bytes of the signature.
    }

    /* Represents the offer's state within a stored campaign */
    struct Offer {
        OfferState state;
        bool collected;
        string nostrHexPubkey;
        uint256 rewardAmount;
        address rewardAddress;
        uint256 nostrAffineX;
        uint256 nostrAffineY;
        bytes32 messageId;
    }

    struct Terms {
        string requestedContent;
        Offer[] offers;
    }

    function verifyCampaignMessage(
        string memory _content,
        string memory _pubkey,
        MessageProof memory _proof,
        uint256 _affineX,
        uint256 _affineY
    )
        internal pure returns (bytes32)
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
        DeletionProof memory _proof,
        uint256 _affineX,
        uint256 _affineY
    )
        internal pure
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

    function idInTags(
        bytes32 needle,
        string memory haystack
    )
        internal pure returns (bool)
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
