// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/*
 * SimpleAuction.sol
 *
 * Purpose:
 *     Core auction contract that accepts bids and inherits
 *     settlement/escrow behavior.
 *
 * Responsibilities:
 *     - Store auction constructor configuration
 *     - Accept valid bids during the bidding window
 *     - Track highest bidder and highest bid
 *     - Queue refunds for outbid bidders
 *     - Update the factory registry after successful bids
 *
 * Non-Responsibilities:
 *     - Deploying auctions
 *     - Storing global registry lists
 *     - Running frontend authentication
 *     - Managing off-chain metadata storage
 *
 * Architecture:
 *
 *      AuctionFactory
 *            ↓
 *      SimpleAuction
 *        ├── AuctionState
 *        ├── AuctionSettlement
 *        └── AuctionEscrow
 */

import "./AuctionState.sol";
import "./AuctionSettlement.sol";
import "./AuctionEscrow.sol";

/* -------------------------------------------------------------------------- */
/*                              Registry Interface                            */
/* -------------------------------------------------------------------------- */

/**
 * @notice Minimal registry interface used by SimpleAuction.
 * @dev Allows the auction contract to update registry metadata
 *      after each successful bid.
 */
interface IAuctionRegistry {
    /**
     * @notice Updates registry highest bid metadata.
     *
     * @param auctionAddress Auction contract address.
     * @param bidder New highest bidder.
     * @param newBidAmount New highest bid amount.
     */
    function updateHighestBid(
        address auctionAddress,
        address bidder,
        uint256 newBidAmount
    ) external;
}

/* -------------------------------------------------------------------------- */
/*                               Simple Auction                               */
/* -------------------------------------------------------------------------- */

/**
 * @title SimpleAuction
 * @notice Standalone auction contract with bidding, settlement, and escrow.
 * @dev Instances are deployed by AuctionFactory.
 */
contract SimpleAuction is AuctionState, AuctionSettlement, AuctionEscrow {
    /* ---------------------------------------------------------------------- */
    /*                              State Variables                           */
    /* ---------------------------------------------------------------------- */

    /// @notice Auction registry/factory address used for bid metadata updates.
    address public immutable registry;

    /* ---------------------------------------------------------------------- */
    /*                               Constructor                              */
    /* ---------------------------------------------------------------------- */

    /**
     * @notice Initializes a new auction.
     *
     * @param _biddingTimeSeconds Duration of bidding window in seconds.
     * @param _seller Seller wallet address.
     * @param _startingBid Minimum bid required to participate.
     * @param _admin Admin address authorized for refund flows.
     * @param _confirmationWindow Buyer confirmation timeout window.
     * @param _registry Registry/factory address.
     * @param _title Human-readable auction title.
     * @param _description Human-readable auction description.
     */
    constructor(
        uint256 _biddingTimeSeconds,
        address _seller,
        uint256 _startingBid,
        address _admin,
        uint256 _confirmationWindow,
        address _registry,
        string memory _title,
        string memory _description
    ) {
        seller = _seller;
        endTime = block.timestamp + _biddingTimeSeconds;
        startingBid = _startingBid;
        admin = _admin;
        confirmationWindow = _confirmationWindow;
        registry = _registry;

        title = _title;
        description = _description;
        imagePlaceholder = "COMING_SOON";
    }

    /* ---------------------------------------------------------------------- */
    /*                              Bid Functions                             */
    /* ---------------------------------------------------------------------- */

    /**
     * @notice Places a bid on the auction.
     *
     * If a previous highest bidder exists, their funds are moved into
     * pendingReturns so they can withdraw using the pull-payment pattern.
     *
     * Requirements:
     *     - auction must still be active
     *     - bid must meet or exceed starting bid
     *     - bid must exceed current highest bid
     */
    function bid() external payable {
        require(block.timestamp < endTime, "Auction already ended");
        require(msg.value >= startingBid, "Bid below starting bid");
        require(msg.value > highestBid, "Bid not high enough");

        if (highestBid != 0) {
            pendingReturns[highestBidder] += highestBid;
        }

        highestBidder = msg.sender;
        highestBid = msg.value;
        bidCount += 1;

        IAuctionRegistry(registry).updateHighestBid(
            address(this),
            msg.sender,
            msg.value
        );

        emit HighestBidIncreased(msg.sender, msg.value);
    }
}
