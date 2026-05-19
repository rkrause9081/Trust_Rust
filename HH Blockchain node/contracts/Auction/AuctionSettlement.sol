// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./AuctionState.sol";

/**
 * AuctionSettlement.sol
 *
 * Purpose:
 *     Handles auction finalization and fund withdrawals.
 *
 *     On endAuction(), the winning bid is NO LONGER sent directly to the seller.
 *     Instead it is locked into escrow (escrowAmount) and a confirmation deadline
 *     is set. Release of those funds is handled by AuctionEscrow.sol.
 *
 *     This contract does NOT:
 *         - Define state variables (inherited from AuctionState)
 *         - Handle bid placement logic
 *         - Release escrow funds (handled by AuctionEscrow)
 *
 *     It is inherited by:
 *         SimpleAuction.sol
 *
 * System Position:
 *
 *     SimpleAuction.sol  (orchestrator)
 *         ↓
 *     AuctionSettlement.sol  ← THIS FILE (settlement logic)
 *         ↓
 *     AuctionState.sol  (shared storage)
 */

abstract contract AuctionSettlement is AuctionState {

    function withdraw() external returns (bool) {
        uint256 amount = pendingReturns[msg.sender];
        require(amount > 0, "No funds to withdraw");

        pendingReturns[msg.sender] = 0;

        if (!payable(msg.sender).send(amount)) {
            pendingReturns[msg.sender] = amount;
            return false;
        }
        return true;
    }

    /**
     * @notice Ends the auction and locks the winning bid into escrow.
     *         Does NOT pay the seller — that happens via AuctionEscrow
     *         once the buyer confirms receipt (or the timeout passes).
     */
    function endAuction() external {
        require(block.timestamp >= endTime, "Auction not yet ended");
        require(!ended, "Auction already closed");

        ended = true;

        // Lock the winning bid into escrow
        escrowAmount = highestBid;

        // Start the buyer confirmation countdown from now
        escrowReleaseTimeout = block.timestamp + confirmationWindow;

        emit AuctionEnded(highestBidder, highestBid);
        emit EscrowFunded(highestBidder, highestBid);
    }
}