// SPDX-License-Identifier: MIT
pragma solidity 0.8.28;

import { Asami } from "./Asami.sol";

contract AsamiV2 is Asami {
    function version() external pure returns (uint256) {
        return 2;
    }
}

