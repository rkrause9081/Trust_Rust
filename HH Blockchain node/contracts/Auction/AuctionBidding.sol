// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./AuctionState.sol";
import "../AuctionRegistry.sol";

abstract contract AuctionBidding is AuctionState, AuctionRegistry {
    function bid() external payable {
        require(block.timestamp < endTime, "Auction already ended");
        require(msg.value >= startingBid, "Bid below starting bid");
        require(msg.value > highestBid, "Bid not high enough");

        if (highestBid != 0) {
            pendingReturns[highestBidder] += highestBid;
        }

        highestBidder = msg.sender;
        highestBid = msg.value;

        _updateHighestBid(address(this), msg.sender, msg.value);

        emit HighestBidIncreased(msg.sender, msg.value);
    }
}