// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * AuctionState.sol
 *
 * Purpose:
 *     Base contract that defines all shared state variables and events
 *     for the auction system.
 *
 *     This contract does NOT:
 *         - Contain any business logic
 *         - Handle bids or settlements
 *
 *     It is inherited by:
 *         AuctionBidding.sol
 *         AuctionSettlement.sol
 *         AuctionEscrow.sol
 *         SimpleAuction.sol
 *
 * System Position:
 *
 *     SimpleAuction.sol  (orchestrator)
 *         ↓
 *     AuctionBidding.sol / AuctionSettlement.sol / AuctionEscrow.sol
 *         ↓
 *     AuctionState.sol  ← THIS FILE (shared storage)
 */

abstract contract AuctionState {

    // ─── Auction Parameters ───────────────────────────────────────────────
    address public seller;
    uint256 public endTime;
    uint256 public startingBid;

    // ─── Bid Tracking ─────────────────────────────────────────────────────
    address public highestBidder;
    uint256 public highestBid;

    // ─── Settlement ───────────────────────────────────────────────────────
    bool public ended;

    // ─── Pending Withdrawals ──────────────────────────────────────────────
    mapping(address => uint256) public pendingReturns;

    // ─── Escrow ───────────────────────────────────────────────────────────

    // Address with admin privileges (set at deployment via AuctionFactory)
    address public admin;

    // Amount held in escrow after auction ends — released to seller on buyer confirmation
    uint256 public escrowAmount;

    // Deadline by which the buyer must confirm receipt; after this seller can claim
    uint256 public escrowReleaseTimeout;

    // How long (in seconds) the buyer has to confirm receipt after auction ends
    uint256 public confirmationWindow;

    // True once the buyer has confirmed receipt of the item
    bool public buyerConfirmed;

    // True once escrow has been fully settled (seller paid or refunded)
    bool public escrowSettled;

    // ─── Emergency Refund Flag ────────────────────────────────────────────

    // When tripped by admin: escrow funds move to pendingReturns for the winner,
    // and the auction is considered cancelled
    bool public refundFlagged;

    // ─── Events ───────────────────────────────────────────────────────────
    event HighestBidIncreased(address indexed bidder, uint256 amount);
    event AuctionEnded(address indexed winner, uint256 amount);
    event BuyerConfirmedReceipt(address indexed buyer, uint256 amount);
    event SellerClaimedAfterTimeout(address indexed seller, uint256 amount);
    event RefundFlagTripped(address indexed admin, uint256 amountRefunded);
    event EscrowFunded(address indexed winner, uint256 amount);
}