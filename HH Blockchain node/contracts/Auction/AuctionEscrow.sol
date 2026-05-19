// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./AuctionState.sol";

/**
 * AuctionEscrow.sol
 *
 * Purpose:
 *     Manages post-auction escrow logic. After endAuction() is called,
 *     the winning bid is held here instead of being immediately sent to
 *     the seller. Funds are only released under one of three conditions:
 *
 *         1. Buyer calls confirmReceipt()     → funds sent to seller
 *         2. Seller calls claimAfterTimeout() → funds sent to seller
 *                                               (only after confirmationWindow expires)
 *         3. Admin calls flagRefund()         → funds added to winner's pendingReturns
 *                                               (winner must call withdraw() to collect)
 *
 *     This contract does NOT:
 *         - Handle bid placement logic
 *         - Handle auction end logic (that triggers escrow funding in AuctionSettlement)
 *
 *     It is inherited by:
 *         SimpleAuction.sol
 *
 * System Position:
 *
 *     SimpleAuction.sol  (orchestrator)
 *         ↓
 *     AuctionEscrow.sol  ← THIS FILE (escrow + refund flag logic)
 *         ↓
 *     AuctionState.sol  (shared storage)
 */

abstract contract AuctionEscrow is AuctionState {

    // ─── Modifiers ────────────────────────────────────────────────────────

    modifier onlyAdmin() {
        require(msg.sender == admin, "Caller is not the admin");
        _;
    }

    modifier escrowActive() {
        require(ended, "Auction has not ended yet");
        require(!escrowSettled, "Escrow already settled");
        require(!refundFlagged, "Refund flag already tripped");
        _;
    }

    // ─── Buyer Confirms Receipt ───────────────────────────────────────────

    /**
     * @notice Called by the winning bidder to confirm they received the item.
     *         Immediately releases escrowed funds to the seller.
     */
    function confirmReceipt() external escrowActive {
        require(msg.sender == highestBidder, "Only the winning bidder can confirm");
        require(escrowAmount > 0, "No funds in escrow");

        buyerConfirmed = true;
        escrowSettled = true;

        uint256 amount = escrowAmount;
        escrowAmount = 0;

        emit BuyerConfirmedReceipt(msg.sender, amount);

        payable(seller).transfer(amount);
    }

    // ─── Seller Claims After Timeout ──────────────────────────────────────

    /**
     * @notice Called by the seller if the buyer never confirmed receipt
     *         and the confirmation window has expired.
     *         Releases escrowed funds to the seller.
     */
    function claimAfterTimeout() external escrowActive {
        require(msg.sender == seller, "Only the seller can claim timeout");
        require(escrowAmount > 0, "No funds in escrow");
        require(
            block.timestamp >= escrowReleaseTimeout,
            "Confirmation window has not expired yet"
        );

        escrowSettled = true;

        uint256 amount = escrowAmount;
        escrowAmount = 0;

        emit SellerClaimedAfterTimeout(msg.sender, amount);

        payable(seller).transfer(amount);
    }

    // ─── Admin Emergency Refund Flag ──────────────────────────────────────

    /**
     * @notice Called only by the admin to cancel the escrow and return
     *         funds to the winning bidder via pendingReturns.
     *         The winner must call withdraw() to collect their refund.
     *         Also flags the auction as cancelled so no further actions
     *         (confirmReceipt, claimAfterTimeout) can be taken.
     */
    function flagRefund() external onlyAdmin {
        require(ended, "Auction has not ended yet");
        require(!escrowSettled, "Escrow already settled");
        require(!refundFlagged, "Refund already flagged");
        require(escrowAmount > 0, "No funds in escrow to refund");

        refundFlagged = true;
        escrowSettled = true;

        uint256 amount = escrowAmount;
        escrowAmount = 0;

        // Queue refund for winner — they must call withdraw() to collect
        pendingReturns[highestBidder] += amount;

        emit RefundFlagTripped(msg.sender, amount);
    }

    // ─── View Helpers ─────────────────────────────────────────────────────

    /**
     * @notice Returns how many seconds remain in the buyer confirmation window.
     *         Returns 0 if the window has already expired or escrow is settled.
     */
    function timeRemainingForConfirmation() external view returns (uint256) {
        if (escrowSettled || !ended || escrowReleaseTimeout == 0) return 0;
        if (block.timestamp >= escrowReleaseTimeout) return 0;
        return escrowReleaseTimeout - block.timestamp;
    }
}