// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

abstract contract AuctionState {
    // Auction Parameters
    address public seller;
    uint256 public endTime;
    uint256 public startingBid;

    // Bid Tracking
    address public highestBidder;
    uint256 public highestBid;

    // Settlement
    bool public ended;

    // Pending Withdrawals
    mapping(address => uint256) public pendingReturns;

    // Escrow
    address public admin;
    uint256 public escrowAmount;
    uint256 public escrowReleaseTimeout;
    uint256 public confirmationWindow;
    bool public buyerConfirmed;
    bool public escrowSettled;
    bool public refundFlagged;

    // Events
    event HighestBidIncreased(address indexed bidder, uint256 amount);
    event AuctionEnded(address indexed winner, uint256 amount);
    event BuyerConfirmedReceipt(address indexed buyer, uint256 amount);
    event SellerClaimedAfterTimeout(address indexed seller, uint256 amount);
    event RefundFlagTripped(address indexed admin, uint256 amountRefunded);
    event EscrowFunded(address indexed winner, uint256 amount);
}