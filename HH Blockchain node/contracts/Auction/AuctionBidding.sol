// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./AuctionState.sol";

/**
 * AuctionBidding.sol
 *
 * Purpose:
 *     Handles all bid-related write logic for the auction.
 *
 *     This contract does NOT:
 *         - Define state variables (inherited from AuctionState)
 *         - Handle auction settlement or withdrawals
 *
 *     It is inherited by:
 *         SimpleAuction.sol
 *
 * System Position:
 *
 *     SimpleAuction.sol  (orchestrator)
 *         ↓
 *     AuctionBidding.sol  ← THIS FILE (write/bid logic)
 *         ↓
 *     AuctionState.sol  (shared storage)
 *
 * Equivalent to:
 *     bidding.py on the Python side
 */

abstract contract AuctionBidding is AuctionState {

    function bid() external payable {
        require(block.timestamp < endTime, "Auction already ended");
        require(msg.value >= startingBid, "Bid is below starting bid");
        require(msg.value > highestBid, "Bid not high enough");

        if (highestBid != 0) {
            pendingReturns[highestBidder] += highestBid;
        }

        highestBidder = msg.sender;
        highestBid = msg.value;

        emit HighestBidIncreased(msg.sender, msg.value);
    }
}