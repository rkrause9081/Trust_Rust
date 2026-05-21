// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

abstract contract AuctionState {
    enum EscrowStatus {
        ActiveAuction,
        AwaitingFinalization,
        AwaitingBuyerConfirmation,
        Complete,
        Refunded
    }

    address public seller;
    uint256 public endTime;
    uint256 public startingBid;

    string public title;
    string public description;
    string public imagePlaceholder;

    address public highestBidder;
    uint256 public highestBid;
    uint256 public bidCount;

    bool public ended;

    mapping(address => uint256) public pendingReturns;

    address public admin;
    uint256 public escrowAmount;
    uint256 public escrowReleaseTimeout;
    uint256 public confirmationWindow;

    bool public buyerConfirmed;
    bool public escrowSettled;
    bool public refundFlagged;

    event HighestBidIncreased(address indexed bidder, uint256 amount);
    event AuctionEnded(address indexed winner, uint256 amount);
    event BuyerConfirmedReceipt(address indexed buyer, uint256 amount);
    event SellerClaimedAfterTimeout(address indexed seller, uint256 amount);
    event RefundFlagTripped(address indexed admin, uint256 amountRefunded);
    event EscrowFunded(address indexed winner, uint256 amount);

 function getEscrowStatus() public view returns (EscrowStatus) {
    if (!ended) {
        return EscrowStatus.ActiveAuction;
    }

    if (refundFlagged) {
        return EscrowStatus.Refunded;
    }

    if (ended && escrowAmount > 0 && !escrowSettled) {
        return EscrowStatus.AwaitingBuyerConfirmation;
    }

    if (escrowSettled) {
        return EscrowStatus.Complete;
    }

    return EscrowStatus.Complete;
    }   
}