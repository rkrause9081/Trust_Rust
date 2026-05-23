// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/*
 * AuctionSettlement.sol
 *
 * Purpose:
 *     Handles auction finalization and pull-based withdrawals.
 *
 * Responsibilities:
 *     - Allow users to withdraw pending returns
 *     - Finalize ended auctions
 *     - Move winning bid into escrow
 *     - Start the buyer confirmation window
 *
 * Non-Responsibilities:
 *     - Accepting bids
 *     - Confirming receipt
 *     - Flagging refunds
 *     - Registry updates
 *
 * Architecture:
 *
 *      SimpleAuction.bid()
 *            ↓
 *      AuctionSettlement.endAuction()
 *            ↓
 *      AuctionEscrow Resolution
 */

import "./AuctionState.sol";

/* -------------------------------------------------------------------------- */
/*                              Auction Settlement                            */
/* -------------------------------------------------------------------------- */

/**
 * @title AuctionSettlement
 * @notice Auction finalization and withdrawal logic.
 * @dev Uses pull-based withdrawals for outbid users and refunds.
 */
abstract contract AuctionSettlement is AuctionState {
    /* ---------------------------------------------------------------------- */
    /*                            External Functions                          */
    /* ---------------------------------------------------------------------- */

    /**
     * @notice Withdraws pending returns owed to the caller.
     *
     * @dev Uses checks-effects-interactions:
     *      balance is cleared before ETH is transferred.
     *
     * Requirements:
     *     - caller must have pending funds
     *
     * @return True if withdrawal succeeds.
     */
    function withdraw() external returns (bool) {
        uint256 amount = pendingReturns[msg.sender];

        require(amount > 0, "No funds to withdraw");

        pendingReturns[msg.sender] = 0;

        (bool success, ) = payable(msg.sender).call{value: amount}("");
        require(success, "Transfer failed");

        return true;
    }

    /**
     * @notice Finalizes the auction after bidding time expires.
     *
     * Upon finalization:
     *     - auction is marked ended
     *     - highest bid is moved into escrow accounting
     *     - buyer confirmation timeout is started
     *
     * Requirements:
     *     - current time must be at or past endTime
     *     - auction must not already be ended
     *     - at least one bid must exist
     */
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
