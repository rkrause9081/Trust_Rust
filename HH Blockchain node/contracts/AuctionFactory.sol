// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/*
 * AuctionFactory.sol
 *
 * Purpose:
 *     Deploys new SimpleAuction contracts and registers them
 *     in the auction registry.
 *
 * Responsibilities:
 *     - Store the factory admin address
 *     - Deploy new SimpleAuction instances
 *     - Register deployed auctions in AuctionRegistry
 *     - Emit auction creation metadata
 *
 * Non-Responsibilities:
 *     - Managing individual auction bidding logic
 *     - Handling escrow settlement directly
 *     - Processing refunds or withdrawals
 *
 * Architecture:
 *
 *      AuctionFactory
 *            ↓
 *      SimpleAuction Deployment
 *            ↓
 *      AuctionRegistry Storage
 */

import "./Auction/SimpleAuction.sol";
import "./AuctionRegistry.sol";

/* -------------------------------------------------------------------------- */
/*                              Auction Factory                               */
/* -------------------------------------------------------------------------- */

/**
 * @title AuctionFactory
 * @notice Deploys SimpleAuction contracts and stores registry metadata.
 * @dev Inherits AuctionRegistry so newly deployed auctions can be indexed
 *      and queried by the Rust backend and frontend.
 */
contract AuctionFactory is AuctionRegistry {
    /* ---------------------------------------------------------------------- */
    /*                              State Variables                           */
    /* ---------------------------------------------------------------------- */

    /// @notice Address that deployed the factory and acts as auction admin.
    address public immutable factoryAdmin;

    /* ---------------------------------------------------------------------- */
    /*                                  Events                                 */
    /* ---------------------------------------------------------------------- */

    /**
     * @notice Emitted when a new auction is deployed and registered.
     *
     * @param auctionAddress Address of the deployed SimpleAuction contract.
     * @param seller Address that created the auction.
     * @param biddingTimeSeconds Auction bidding duration in seconds.
     * @param endTime Timestamp when bidding ends.
     * @param startingBid Minimum required starting bid.
     * @param admin Factory-level admin address.
     * @param confirmationWindow Post-auction buyer confirmation window.
     */
    event AuctionCreated(
        address indexed auctionAddress,
        address indexed seller,
        uint256 biddingTimeSeconds,
        uint256 endTime,
        uint256 startingBid,
        address admin,
        uint256 confirmationWindow
    );

    /* ---------------------------------------------------------------------- */
    /*                               Constructor                              */
    /* ---------------------------------------------------------------------- */

    /**
     * @notice Initializes the factory admin.
     * @dev The deployer becomes the admin used by each auction created
     *      through this factory.
     */
    constructor() {
        factoryAdmin = msg.sender;
    }

    /* ---------------------------------------------------------------------- */
    /*                            External Functions                          */
    /* ---------------------------------------------------------------------- */

    /**
     * @notice Creates and registers a new SimpleAuction contract.
     *
     * @param biddingTimeSeconds Auction bidding duration in seconds.
     * @param startingBid Minimum bid required to participate.
     * @param confirmationWindow Buyer confirmation window after auction end.
     * @param title Human-readable auction title.
     * @param description Human-readable auction description.
     *
     * @return Address of the newly deployed SimpleAuction contract.
     */
    function createAuction(
        uint256 biddingTimeSeconds,
        uint256 startingBid,
        uint256 confirmationWindow,
        string memory title,
        string memory description
    ) external returns (address) {
        string memory imagePlaceholder = "COMING_SOON";

        SimpleAuction auction = new SimpleAuction(
            biddingTimeSeconds,
            msg.sender,
            startingBid,
            factoryAdmin,
            confirmationWindow,
            address(this),
            title,
            description
        );

        address auctionAddress = address(auction);
        uint256 auctionEndTime = auction.endTime();

        _registerAuction(
            auctionAddress,
            msg.sender,
            biddingTimeSeconds,
            auctionEndTime,
            startingBid,
            factoryAdmin,
            confirmationWindow,
            title,
            description,
            imagePlaceholder
        );

        emit AuctionCreated(
            auctionAddress,
            msg.sender,
            biddingTimeSeconds,
            auctionEndTime,
            startingBid,
            factoryAdmin,
            confirmationWindow
        );

        return auctionAddress;
    }
}
