// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/*
 * AuctionState.sol
 *
 * Purpose:
 *     Defines shared auction, bidding, escrow, and settlement state
 *     for the SimpleAuction inheritance tree.
 *
 * Responsibilities:
 *     - Store auction metadata
 *     - Store highest bid state
 *     - Store pending withdrawal balances
 *     - Store escrow lifecycle state
 *     - Define common auction and escrow events
 *
 * Non-Responsibilities:
 *     - Accepting bids
 *     - Ending auctions
 *     - Confirming receipt
 *     - Processing refunds or withdrawals
 *
 * Architecture:
 *
 *      AuctionState
 *          ↑
 *      SimpleAuction
 *          ↑
 *      AuctionSettlement / AuctionEscrow
 */

/* -------------------------------------------------------------------------- */
/*                                Auction State                               */
/* -------------------------------------------------------------------------- */

/**
 * @title AuctionState
 * @notice Shared storage and events used by all auction behavior modules.
 * @dev This contract is abstract and is intended to be inherited by
 *      SimpleAuction, AuctionSettlement, and AuctionEscrow.
 */
abstract contract AuctionState {
    /* ---------------------------------------------------------------------- */
    /*                                  Enums                                  */
    /* ---------------------------------------------------------------------- */

    /**
     * @notice High-level escrow lifecycle states exposed to frontend/Rust.
     */
    enum EscrowStatus {
        ActiveAuction,
        AwaitingFinalization,
        AwaitingBuyerConfirmation,
        Complete,
        Refunded
    }

    /* ---------------------------------------------------------------------- */
    /*                              Auction Metadata                          */
    /* ---------------------------------------------------------------------- */

    /// @notice Seller that created the auction.
    address public seller;

    /// @notice Timestamp when bidding ends.
    uint256 public endTime;

    /// @notice Minimum bid required to participate.
    uint256 public startingBid;

    /// @notice Human-readable auction title.
    string public title;

    /// @notice Human-readable auction description.
    string public description;

    /// @notice Placeholder image metadata used until image support is added.
    string public imagePlaceholder;

    /* ---------------------------------------------------------------------- */
    /*                                Bid State                                */
    /* ---------------------------------------------------------------------- */

    /// @notice Current highest bidder.
    address public highestBidder;

    /// @notice Current highest bid amount.
    uint256 public highestBid;

    /// @notice Total number of accepted bids.
    uint256 public bidCount;

    /// @notice True once the auction has been finalized.
    bool public ended;

    /// @notice Pull-payment refunds owed to outbid users or refunded winners.
    mapping(address => uint256) public pendingReturns;

    /* ---------------------------------------------------------------------- */
    /*                               Escrow State                              */
    /* ---------------------------------------------------------------------- */

    /// @notice Admin authorized to flag refunds.
    address public admin;

    /// @notice ETH currently held in escrow for the winning bid.
    uint256 public escrowAmount;

    /// @notice Timestamp after which seller can claim escrow if buyer is silent.
    uint256 public escrowReleaseTimeout;

    /// @notice Buyer confirmation window duration in seconds.
    uint256 public confirmationWindow;

    /// @notice True once buyer confirms receipt.
    bool public buyerConfirmed;

    /// @notice True once escrow has been resolved.
    bool public escrowSettled;

    /// @notice True once admin flags a refund path.
    bool public refundFlagged;

    /* ---------------------------------------------------------------------- */
    /*                                  Events                                 */
    /* ---------------------------------------------------------------------- */

    /// @notice Emitted when a new highest bid is accepted.
    event HighestBidIncreased(address indexed bidder, uint256 amount);

    /// @notice Emitted when the auction is finalized.
    event AuctionEnded(address indexed winner, uint256 amount);

    /// @notice Emitted when the buyer confirms receipt and seller is paid.
    event BuyerConfirmedReceipt(address indexed buyer, uint256 amount);

    /// @notice Emitted when seller claims escrow after buyer timeout.
    event SellerClaimedAfterTimeout(address indexed seller, uint256 amount);

    /// @notice Emitted when admin flags escrow funds for buyer refund.
    event RefundFlagTripped(address indexed admin, uint256 amountRefunded);

    /// @notice Emitted when winning bid funds are moved into escrow.
    event EscrowFunded(address indexed winner, uint256 amount);

    /* ---------------------------------------------------------------------- */
    /*                              View Functions                            */
    /* ---------------------------------------------------------------------- */

    /**
     * @notice Returns the current escrow lifecycle status.
     *
     * @return Escrow status enum used by Rust and frontend clients.
     */
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
