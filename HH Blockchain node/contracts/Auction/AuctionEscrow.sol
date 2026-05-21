// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./AuctionState.sol";

abstract contract AuctionEscrow is AuctionState {
    function confirmReceipt() external {
        require(ended, "Auction not ended");
        require(msg.sender == highestBidder, "Only winner can confirm");
        require(!escrowSettled, "Escrow already settled");
        require(escrowAmount > 0, "No escrow funds");

        buyerConfirmed = true;
        escrowSettled = true;

        uint256 amount = escrowAmount;
        escrowAmount = 0;

        (bool success, ) = payable(seller).call{value: amount}("");
        require(success, "Transfer failed");

        emit BuyerConfirmedReceipt(msg.sender, amount);
    }

    function claimAfterTimeout() external {
        require(ended, "Auction not ended");
        require(msg.sender == seller, "Only seller can claim");
        require(!escrowSettled, "Escrow already settled");
        require(escrowAmount > 0, "No escrow funds");
        require(
            block.timestamp >= escrowReleaseTimeout,
            "Confirmation window still active"
        );

        escrowSettled = true;

        uint256 amount = escrowAmount;
        escrowAmount = 0;

        (bool success, ) = payable(seller).call{value: amount}("");
        require(success, "Transfer failed");

        emit SellerClaimedAfterTimeout(msg.sender, amount);
    }

    function flagRefund() external {
        require(ended, "Auction not ended");
        require(msg.sender == admin, "Only admin can flag refund");
        require(!escrowSettled, "Escrow already settled");
        require(escrowAmount > 0, "No escrow funds");

        escrowSettled = true;
        refundFlagged = true;

        uint256 amount = escrowAmount;
        escrowAmount = 0;

        pendingReturns[highestBidder] += amount;

        emit RefundFlagTripped(msg.sender, amount);
    }

    function timeRemainingForConfirmation() external view returns (uint256) {
        if (!ended || escrowSettled || block.timestamp >= escrowReleaseTimeout) {
            return 0;
        }

        return escrowReleaseTimeout - block.timestamp;
    }

    function canConfirmReceipt(address caller) external view returns (bool) {
        return (
            caller == highestBidder &&
            ended &&
            !escrowSettled &&
            escrowAmount > 0
        );
    }

    function canClaimTimeout(address caller) external view returns (bool) {
        return (
            caller == seller &&
            ended &&
            !escrowSettled &&
            escrowAmount > 0 &&
            block.timestamp >= escrowReleaseTimeout
        );
    }

    function canFlagRefund(address caller) external view returns (bool) {
        return (
            caller == admin &&
            ended &&
            !escrowSettled &&
            escrowAmount > 0
        );
    }
}