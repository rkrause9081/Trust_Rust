// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

abstract contract AuctionRegistry {
    address[] internal auctions;

    struct AuctionRegistryItem {
        address auctionAddress;
        address seller;
        uint256 biddingTimeSeconds;
        uint256 endTime;
        uint256 startingBid;
        address highestBidder;
        uint256 highestBid;
        address admin;
        uint256 confirmationWindow;
        uint256 createdAt;
        bool exists;
    }

    mapping(address => AuctionRegistryItem) internal auctionRegistry;
    mapping(address => address[]) internal auctionsBySeller;

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
            highestBidder: address(0),
            highestBid: startingBid,
            admin: admin,
            confirmationWindow: confirmationWindow,
            createdAt: block.timestamp,
            exists: true
        });
    }

    function _updateHighestBid(
        address auctionAddress,
        address bidder,
        uint256 newBidAmount
    ) internal {
        if (!auctionRegistry[auctionAddress].exists) return;

        AuctionRegistryItem storage item = auctionRegistry[auctionAddress];
        if (newBidAmount > item.highestBid) {
            item.highestBidder = bidder;
            item.highestBid = newBidAmount;
        }
    }

    function getAuctionsPaginated(uint256 offset, uint256 limit)
        external view returns (AuctionRegistryItem[] memory)
    {
        if (offset >= auctions.length) return new AuctionRegistryItem[](0);

        uint256 size = limit < (auctions.length - offset) ? limit : (auctions.length - offset);
        AuctionRegistryItem[] memory page = new AuctionRegistryItem[](size);

        for (uint256 i = 0; i < size; i++) {
            page[i] = auctionRegistry[auctions[offset + i]];
        }
        return page;
    }

    function getAuctionRegistryItem(address auctionAddress)
        external view returns (AuctionRegistryItem memory)
    {
        require(auctionRegistry[auctionAddress].exists, "Auction not found");
        return auctionRegistry[auctionAddress];
    }

    function auctionCount() external view returns (uint256) {
        return auctions.length;
    }
}