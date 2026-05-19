// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * AuctionRegistry.sol
 *
 * Purpose:
 *     Handles all auction registry storage and read/query logic.
 *
 *     This contract is responsible for:
 *         - Storing deployed auction metadata
 *         - Tracking seller-created auctions
 *         - Providing frontend hydration/query functions
 *         - Pagination support for UI consumption
 *
 *     This contract does NOT:
 *         - Deploy auctions
 *         - Handle auction bidding
 *         - Handle settlement logic
 *
 *     It is inherited by:
 *         AuctionFactory.sol
 *
 * System Position:
 *
 *     AuctionFactory.sol  (deployment/orchestration)
 *         ↓
 *     AuctionRegistry.sol  ← THIS FILE (registry/query layer)
 *         ↓
 *     SimpleAuction.sol  (auction instances)
 *
 * Equivalent to:
 *     registry.rs on the Rust backend side
 */

abstract contract AuctionRegistry {

    /**
     * Stores all deployed auction contract addresses.
     *
     * Used for:
     *     - Global auction indexing
     *     - Pagination
     *     - Frontend hydration
     */
    address[] internal auctions;

    /**
     * Core metadata registry entry.
     *
     * This struct exists specifically for frontend hydration
     * and backend indexing.
     */
    struct AuctionRegistryItem {
        address auctionAddress;
        address seller;
        uint256 biddingTimeSeconds;
        uint256 endTime;
        uint256 startingBid;
        address admin;
        uint256 confirmationWindow;
        uint256 createdAt;
        bool exists;
    }

    /**
     * Maps auction contract address → registry metadata.
     */
    mapping(address => AuctionRegistryItem) internal auctionRegistry;

    /**
     * Maps seller → list of auctions they created.
     */
    mapping(address => address[]) internal auctionsBySeller;

    /**
     * Internal registry writer.
     *
     * Called ONLY by:
     *     AuctionFactory.sol
     *
     * This function registers newly deployed auctions
     * into the global registry system.
     */
    function _registerAuction(
        address auctionAddress,
        address seller,
        uint256 biddingTimeSeconds,
        uint256 endTime,
        uint256 startingBid,
        address admin,
        uint256 confirmationWindow
    ) internal {
        auctions.push(auctionAddress);

        auctionsBySeller[seller].push(auctionAddress);

        auctionRegistry[auctionAddress] = AuctionRegistryItem({
            auctionAddress: auctionAddress,
            seller: seller,
            biddingTimeSeconds: biddingTimeSeconds,
            endTime: endTime,
            startingBid: startingBid,
            admin: admin,
            confirmationWindow: confirmationWindow,
            createdAt: block.timestamp,
            exists: true
        });
    }

    /**
     * Returns all registered auction addresses.
     *
     * WARNING:
     *     This becomes expensive at large scale.
     *
     * Frontends should prefer:
     *     getAuctionsPaginated()
     */
    function getAuctions()
        external
        view
        returns (address[] memory)
    {
        return auctions;
    }

    /**
     * Returns total number of registered auctions.
     */
    function auctionCount()
        external
        view
        returns (uint256)
    {
        return auctions.length;
    }

    /**
     * Returns registry metadata for a specific auction.
     */
    function getAuctionRegistryItem(
        address auctionAddress
    )
        external
        view
        returns (AuctionRegistryItem memory)
    {
        require(
            auctionRegistry[auctionAddress].exists,
            "Auction not found"
        );

        return auctionRegistry[auctionAddress];
    }

    /**
     * Returns auction registry entry by global index.
     *
     * Useful for:
     *     - Pagination systems
     *     - Infinite scrolling UIs
     *     - Alloy/Rust hydration
     */
    function getAuctionByIndex(
        uint256 index
    )
        external
        view
        returns (AuctionRegistryItem memory)
    {
        require(index < auctions.length, "Index out of bounds");

        return auctionRegistry[auctions[index]];
    }

    /**
     * Returns paginated auction registry entries.
     *
     * Example:
     *     offset = 0
     *     limit  = 20
     *
     * Returns:
     *     auctions[0 → 19]
     *
     * Intended for:
     *     - Frontend page hydration
     *     - REST APIs
     *     - Axum JSON responses
     */
    function getAuctionsPaginated(
        uint256 offset,
        uint256 limit
    )
        external
        view
        returns (AuctionRegistryItem[] memory)
    {
        require(offset < auctions.length, "Offset out of bounds");

        uint256 remaining = auctions.length - offset;

        uint256 size =
            limit < remaining
                ? limit
                : remaining;

        AuctionRegistryItem[] memory page =
            new AuctionRegistryItem[](size);

        for (uint256 i = 0; i < size; i++) {
            page[i] =
                auctionRegistry[
                    auctions[offset + i]
                ];
        }

        return page;
    }

    /**
     * Returns all auction addresses created by a seller.
     */
    function getAuctionsBySeller(
        address seller
    )
        external
        view
        returns (address[] memory)
    {
        return auctionsBySeller[seller];
    }

    /**
     * Verifies whether an auction exists in the registry.
     */
    function isRegisteredAuction(
        address auctionAddress
    )
        external
        view
        returns (bool)
    {
        return auctionRegistry[auctionAddress].exists;
    }
}