// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./AuctionState.sol";

abstract contract AuctionEscrow is AuctionState {

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

    function claimAfterTimeout() external escrowActive {
        require(msg.sender == seller, "Only the seller can claim timeout");
        require(escrowAmount > 0, "No funds in escrow");
        require(block.timestamp >= escrowReleaseTimeout, "Confirmation window has not expired");

        escrowSettled = true;

        uint256 amount = escrowAmount;
        escrowAmount = 0;

        emit SellerClaimedAfterTimeout(msg.sender, amount);
        payable(seller).transfer(amount);
    }

    function flagRefund() external onlyAdmin {
        require(ended, "Auction has not ended yet");
        require(!escrowSettled, "Escrow already settled");
        require(!refundFlagged, "Refund already flagged");
        require(escrowAmount > 0, "No funds in escrow to refund");

        refundFlagged = true;
        escrowSettled = true;

        uint256 amount = escrowAmount;
        escrowAmount = 0;

        pendingReturns[highestBidder] += amount;

        emit RefundFlagTripped(msg.sender, amount);
    }

    function timeRemainingForConfirmation() external view returns (uint256) {
        if (escrowSettled || !ended || escrowReleaseTimeout == 0) return 0;
        if (block.timestamp >= escrowReleaseTimeout) return 0;
        return escrowReleaseTimeout - block.timestamp;
    }
}