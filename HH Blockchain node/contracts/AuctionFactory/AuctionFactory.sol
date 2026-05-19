// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
/**
 * AuctionFactory.sol
 *
 * Purpose:
 *     Responsible for deploying new auction contracts.
 *
 *     This contract:
 *         - Deploys SimpleAuction instances
 *         - Registers auctions into AuctionRegistry
 *         - Emits deployment events
 *
 *     This contract does NOT:
 *         - Handle bidding logic
 *         - Store auction settlement logic
 *         - Store registry query logic
 *
 * System Position:
 *
 *     AuctionFactory.sol  ← THIS FILE (deployment layer)
 *         ↓
 *     AuctionRegistry.sol  (registry/query layer)
 *         ↓
 *     SimpleAuction.sol  (auction instances)
 *
 * Equivalent to:
 *     factory.rs on the Rust backend side
 */

import "../Auction/SimpleAuction.sol";
import "./AuctionRegistry.sol";

contract AuctionFactory is AuctionRegistry {
    address public immutable factoryAdmin;

    event AuctionCreated(
        address indexed auctionAddress,
        address indexed seller,
        uint256 biddingTimeSeconds,
        uint256 endTime,
        uint256 startingBid,
        address admin,
        uint256 confirmationWindow
    );

    constructor() {
        factoryAdmin = msg.sender;
    }

    function createAuction(
        uint256 biddingTimeSeconds,
        uint256 startingBid,
        uint256 confirmationWindow
    ) external returns (address) {
        SimpleAuction auction = new SimpleAuction(
            biddingTimeSeconds,
            msg.sender,
            startingBid,
            factoryAdmin,
            confirmationWindow
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
            confirmationWindow
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