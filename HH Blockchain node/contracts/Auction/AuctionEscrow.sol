// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/*
 * AuctionEscrow.sol
 *
 * Purpose:
 *     Handles post-auction escrow resolution paths.
 *
 * Responsibilities:
 *     - Allow buyer receipt confirmation
 *     - Allow seller timeout claim
 *     - Allow admin refund flagging
 *     - Expose escrow permission helper views
 *     - Expose remaining confirmation time
 *
 * Non-Responsibilities:
 *     - Accepting bids
 *     - Ending auctions
 *     - Deploying auctions
 *     - Registry updates
 *
 * Architecture:
 *
 *      SimpleAuction
 *           ↓
 *      AuctionSettlement.endAuction()
 *           ↓
 *      AuctionEscrow Resolution Paths
 */

import "./AuctionState.sol";

/* -------------------------------------------------------------------------- */
/*                                Auction Escrow                              */
/* -------------------------------------------------------------------------- */

/**
 * @title AuctionEscrow
 * @notice Post-auction escrow settlement and refund logic.
 * @dev Uses the pull-payment pattern for refunds through pendingReturns.
 */
abstract contract AuctionEscrow is AuctionState {
    /* ---------------------------------------------------------------------- */
    /*                           Escrow Mutations                              */
    /* ---------------------------------------------------------------------- */

    /**
     * @notice Allows the winning bidder to confirm item receipt.
     *
     * Upon confirmation:
     *     - escrow is marked settled
     *     - escrow amount is cleared before transfer
     *     - seller receives the winning bid amount
     *
     * Requirements:
     *     - auction must be ended
     *     - caller must be highest bidder
     *     - escrow must not already be settled
     *     - escrow must contain funds
     */
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

    /**
     * @notice Allows the seller to claim escrow after buyer timeout.
     *
     * Requirements:
     *     - auction must be ended
     *     - caller must be seller
     *     - escrow must not already be settled
     *     - escrow must contain funds
     *     - confirmation window must have expired
     */
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

    /**
     * @notice Allows admin to flag escrow funds for buyer refund.
     *
     * @dev Funds are moved into pendingReturns for the highest bidder,
     *      preserving a pull-based withdrawal pattern.
     *
     * Requirements:
     *     - auction must be ended
     *     - caller must be admin
     *     - escrow must not already be settled
     *     - escrow must contain funds
     */
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

    /* ---------------------------------------------------------------------- */
    /*                              View Functions                            */
    /* ---------------------------------------------------------------------- */

    /**
     * @notice Returns remaining buyer confirmation time.
     *
     * @return Seconds remaining before seller timeout claim becomes available.
     */
    function timeRemainingForConfirmation() external view returns (uint256) {
        if (!ended || escrowSettled || block.timestamp >= escrowReleaseTimeout) {
            return 0;
        }

        return escrowReleaseTimeout - block.timestamp;
    }

    /**
     * @notice Checks whether a caller can confirm receipt.
     *
     * @param caller Address being checked.
     * @return True if caller can currently confirm receipt.
     */
    function canConfirmReceipt(address caller) external view returns (bool) {
        return (
            caller == highestBidder &&
            ended &&
            !escrowSettled &&
            escrowAmount > 0
        );
    }

    /**
     * @notice Checks whether a caller can claim escrow after timeout.
     *
     * @param caller Address being checked.
     * @return True if caller can currently claim timeout settlement.
     */
    function canClaimTimeout(address caller) external view returns (bool) {
        return (
            caller == seller &&
            ended &&
            !escrowSettled &&
            escrowAmount > 0 &&
            block.timestamp >= escrowReleaseTimeout
        );
    }

    /**
     * @notice Checks whether a caller can flag a refund.
     *
     * @param caller Address being checked.
     * @return True if caller can currently flag a refund.
     */
    function canFlagRefund(address caller) external view returns (bool) {
        return (
            caller == admin &&
            ended &&
            !escrowSettled &&
            escrowAmount > 0
        );
    }
}
