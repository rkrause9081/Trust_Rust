// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./AuctionState.sol";

abstract contract AuctionSettlement is AuctionState {
    function withdraw() external returns (bool) {
        uint256 amount = pendingReturns[msg.sender];
        require(amount > 0, "No funds to withdraw");

        pendingReturns[msg.sender] = 0;

        (bool success, ) = payable(msg.sender).call{value: amount}("");
        require(success, "Transfer failed");

        return true;
    }

    function endAuction() external {
    require(block.timestamp >= endTime, "Auction not yet ended");
    require(!ended, "Auction already closed");
    require(highestBid > 0, "No bids placed");

    ended = true;

    escrowAmount = highestBid;
    escrowReleaseTimeout = block.timestamp + confirmationWindow;

    emit AuctionEnded(highestBidder, highestBid);
    emit EscrowFunded(highestBidder, highestBid);
}
}