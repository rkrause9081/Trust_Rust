// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./AuctionBidding.sol";
import "./AuctionSettlement.sol";
import "./AuctionEscrow.sol";

contract SimpleAuction is AuctionBidding, AuctionSettlement, AuctionEscrow {
    constructor(
        uint256 _biddingTimeSeconds,
        address _seller,
        uint256 _startingBid,
        address _admin,
        uint256 _confirmationWindow
    ) {
        seller = _seller;
        endTime = block.timestamp + _biddingTimeSeconds;
        startingBid = _startingBid;
        admin = _admin;
        confirmationWindow = _confirmationWindow;
    }
}