// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/*
 * AuctionRegistry.sol
 *
 * Purpose:
 *     Stores and exposes registry metadata for auctions deployed
 *     through the AuctionFactory.
 *
 * Responsibilities:
 *     - Store deployed auction addresses
 *     - Store searchable auction metadata
 *     - Track auctions by seller
 *     - Track highest bid metadata for frontend queries
 *     - Provide registry read/query functions
 *
 * Non-Responsibilities:
 *     - Deploying auctions
 *     - Enforcing auction bidding rules
 *     - Holding escrow funds
 *     - Processing settlement or withdrawals
 *
 * Architecture:
 *
 *      AuctionFactory
 *            ↓
 *      AuctionRegistry
 *            ↓
 *      Registry Read APIs
 */

/* -------------------------------------------------------------------------- */
/*                              Auction Registry                              */
/* -------------------------------------------------------------------------- */

/**
 * @title AuctionRegistry
 * @notice Shared registry storage and read/query layer for deployed auctions.
 * @dev Intended to be inherited by AuctionFactory.
 */
abstract contract AuctionRegistry {
    /* ---------------------------------------------------------------------- */
    /*                              State Variables                           */
    /* ---------------------------------------------------------------------- */

    /// @notice List of all auction contracts deployed through the factory.
    address[] internal auctions;

    /**
     * @notice Registry metadata stored for each deployed auction.
     * @dev Mirrors the structure decoded by the Rust registry client.
     */
    struct AuctionRegistryItem {
        address auctionAddress;
        address seller;
        uint256 biddingTimeSeconds;
        uint256 endTime;
        uint256 startingBid;
        address highestBidder;
        uint256 highestBid;
        uint256 bidCount;
        address admin;
        uint256 confirmationWindow;
        uint256 createdAt;
        bool exists;
        string title;
        string description;
        string imagePlaceholder;
    }

    /// @notice Maps auction address to registry metadata.
    mapping(address => AuctionRegistryItem) internal auctionRegistry;

    /// @notice Maps seller address to auctions created by that seller.
    mapping(address => address[]) internal auctionsBySeller;

    /* ---------------------------------------------------------------------- */
    /*                             Internal Functions                         */
    /* ---------------------------------------------------------------------- */

    /**
     * @notice Registers a newly deployed auction.
     * @dev Called by AuctionFactory immediately after deploying SimpleAuction.
     *
     * @param auctionAddress Address of the deployed auction contract.
     * @param seller Address that created the auction.
     * @param biddingTimeSeconds Auction bidding duration in seconds.
     * @param endTime Timestamp when bidding ends.
     * @param startingBid Minimum bid required to participate.
     * @param admin Factory-level admin address.
     * @param confirmationWindow Buyer confirmation timeout window.
     * @param title Human-readable auction title.
     * @param description Human-readable auction description.
     * @param imagePlaceholder Placeholder image metadata.
     */
    function _registerAuction(
        address auctionAddress,
        address seller,
        uint256 biddingTimeSeconds,
        uint256 endTime,
        uint256 startingBid,
        address admin,
        uint256 confirmationWindow,
        string memory title,
        string memory description,
        string memory imagePlaceholder
    ) internal {
        auctions.push(auctionAddress);
        auctionsBySeller[seller].push(auctionAddress);

        auctionRegistry[auctionAddress] = AuctionRegistryItem({
            auctionAddress: auctionAddress,
            seller: seller,
            biddingTimeSeconds: biddingTimeSeconds,
            endTime: endTime,
            startingBid: startingBid,
            highestBidder: address(0),
            highestBid: 0,
            bidCount: 0,
            admin: admin,
            confirmationWindow: confirmationWindow,
            createdAt: block.timestamp,
            exists: true,
            title: title,
            description: description,
            imagePlaceholder: imagePlaceholder
        });
    }

    /* ---------------------------------------------------------------------- */
    /*                            External Mutations                          */
    /* ---------------------------------------------------------------------- */

    /**
     * @notice Updates registry metadata after a successful bid.
     * @dev Only the auction contract itself may update its registry entry.
     *
     * @param auctionAddress Auction contract address being updated.
     * @param bidder Address of the new highest bidder.
     * @param newBidAmount New highest bid amount.
     */
    function updateHighestBid(
        address auctionAddress,
        address bidder,
        uint256 newBidAmount
    ) external {
        require(auctionRegistry[auctionAddress].exists, "Auction not found");
        require(msg.sender == auctionAddress, "Only auction can update");

        AuctionRegistryItem storage item = auctionRegistry[auctionAddress];

        require(newBidAmount > item.highestBid, "Bid not higher");

        item.highestBidder = bidder;
        item.highestBid = newBidAmount;
        item.bidCount += 1;
    }

    /* ---------------------------------------------------------------------- */
    /*                              View Functions                            */
    /* ---------------------------------------------------------------------- */

    /**
     * @notice Returns all registered auction addresses.
     *
     * @return Array of auction contract addresses.
     */
    function getAuctions() external view returns (address[] memory) {
        return auctions;
    }

    /**
     * @notice Returns the total number of registered auctions.
     *
     * @return Number of auctions registered through the factory.
     */
    function auctionCount() external view returns (uint256) {
        return auctions.length;
    }

    /**
     * @notice Returns registry metadata for a specific auction.
     *
     * @param auctionAddress Auction contract address.
     * @return Registry item for the requested auction.
     */
    function getAuctionRegistryItem(
        address auctionAddress
    ) external view returns (AuctionRegistryItem memory) {
        require(auctionRegistry[auctionAddress].exists, "Auction not found");

        return auctionRegistry[auctionAddress];
    }

    /**
     * @notice Returns registry metadata by array index.
     *
     * @param index Index inside the auctions array.
     * @return Registry item at the requested index.
     */
    function getAuctionByIndex(
        uint256 index
    ) external view returns (AuctionRegistryItem memory) {
        require(index < auctions.length, "Index out of bounds");

        return auctionRegistry[auctions[index]];
    }

    /**
     * @notice Returns a paginated slice of auction registry entries.
     *
     * @param offset Starting index.
     * @param limit Maximum number of entries to return.
     * @return Page of registry items.
     */
    function getAuctionsPaginated(
        uint256 offset,
        uint256 limit
    ) external view returns (AuctionRegistryItem[] memory) {
        if (offset >= auctions.length) {
            return new AuctionRegistryItem[](0);
        }

        uint256 size = limit < auctions.length - offset
            ? limit
            : auctions.length - offset;

        AuctionRegistryItem[] memory page = new AuctionRegistryItem[](size);

        for (uint256 i = 0; i < size; i++) {
            page[i] = auctionRegistry[auctions[offset + i]];
        }

        return page;
    }

    /**
     * @notice Returns auctions created by a seller.
     *
     * @param seller Seller wallet address.
     * @return Array of auction addresses created by the seller.
     */
    function getAuctionsBySeller(
        address seller
    ) external view returns (address[] memory) {
        return auctionsBySeller[seller];
    }

    /**
     * @notice Checks whether an auction address is registered.
     *
     * @param auctionAddress Auction contract address.
     * @return True if the auction exists in the registry.
     */
    function isRegisteredAuction(
        address auctionAddress
    ) external view returns (bool) {
        return auctionRegistry[auctionAddress].exists;
    }
}
