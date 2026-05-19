// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./AuctionBidding.sol";
import "./AuctionSettlement.sol";
import "./AuctionEscrow.sol";

/**
 * SimpleAuction.sol
 *
 * Purpose:
 *     Top-level auction contract. Acts as the orchestrator, wiring together
 *     all auction modules via inheritance.
 *
 *     This contract does NOT:
 *         - Contain bid, settlement, or escrow logic directly
 *         - Define state variables directly (via AuctionState)
 *
 *     It inherits from:
 *         AuctionBidding    → bid()
 *         AuctionSettlement → withdraw(), endAuction()
 *         AuctionEscrow     → confirmReceipt(), claimAfterTimeout(), flagRefund()
 *         AuctionState      → all state variables and events
 *                             (inherited transitively)
 *
 * System Position:
 *
 *     AuctionFactory.sol  (deployment)
 *         ↓
 *     SimpleAuction.sol  ← THIS FILE (orchestrator)
 *         ↓
 *     AuctionBidding.sol / AuctionSettlement.sol / AuctionEscrow.sol
 *         ↓
 *     AuctionState.sol  (shared storage)
 *
 * Equivalent to:
 *     mainauction.py on the Python side
 */

contract SimpleAuction is AuctionBidding, AuctionSettlement, AuctionEscrow {

    /**
     * @param _biddingTimeSeconds   How long the auction runs (in seconds)
     * @param _seller               Address of the seller
     * @param _startingBid          Minimum bid amount (in wei)
     * @param _admin                Address of the admin (can trip emergency refund flag)
     * @param _confirmationWindow   How long (in seconds) the buyer has to confirm receipt
     *                              after the auction ends before the seller can claim timeout
     */
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
