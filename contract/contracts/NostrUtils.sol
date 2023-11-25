// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;
import "@openzeppelin/contracts/utils/Strings.sol";
import "./Schnorr.sol";

contract NostrUtils {
    // The first tag of a repost es always "e", so we can reconstruct that part, which saves gas.
    // But the rest of the tags is hard to reconstruct, so we take it as a string.
    // The _tagsAfterPostId is the raw string with the rest of the tags, it's not valid json.
    function verifyRepost(
        string memory _pubkey,
        string memory _postId,
        string memory _tagsAfterPostId,
        uint256 _createdAt,
        uint256 _affineX,
        uint256 _affineY,
        uint256 _sigR,
        uint256 _sigS
    )
        public pure returns (string memory)
    {
        bytes memory message = abi.encodePacked(
            "[0,\"",
            _pubkey,
            "\",",
            Strings.toString(_createdAt),
            ",1,",
            "[[\"e\",\"",
            _postId,
            _tagsAfterPostId,
            "]"
        );

        bytes32 messageId = sha256(message);

        require(Schnorr.verify(
            messageId,
            _affineX,
            _affineY,
            _sigR,
            _sigS
        ), "invalid_proof");

        return Strings.toHexString(uint256(messageId));
    }

    function verifyDeletionMessage(
        string memory _pubkey,
        string memory _content,
        string memory _tags,
        uint256 _createdAt,
        uint256 _affineX,
        uint256 _affineY,
        uint256 _sigR,
        uint256 _sigS
    )
        public pure
    {
        bytes memory message = abi.encodePacked(
            "[0,\"",
            _pubkey,
            "\",",
            Strings.toString(_createdAt),
            ",5,",
            _tags,
            ",\"",
            _content,
            "\"]"
        );

        require(Schnorr.verify(
            sha256(message),
            _affineX,
            _affineY,
            _sigR,
            _sigS
        ), "invalid_deletion_proof");
    }

    function idInTags(
        string memory needleHex,
        string memory haystack
    )
        public pure returns (bool)
    {
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
