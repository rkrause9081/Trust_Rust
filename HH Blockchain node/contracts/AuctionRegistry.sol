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
        uint256 bidCount;
        address admin;
        uint256 confirmationWindow;
        uint256 createdAt;
        bool exists;
        string title;
        string description;
        string imagePlaceholder;
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

    function getAuctions() external view returns (address[] memory) {
        return auctions;
    }

    function auctionCount() external view returns (uint256) {
        return auctions.length;
    }

    function getAuctionRegistryItem(address auctionAddress)
        external
        view
        returns (AuctionRegistryItem memory)
    {
        require(auctionRegistry[auctionAddress].exists, "Auction not found");
        return auctionRegistry[auctionAddress];
    }

    function getAuctionByIndex(uint256 index)
        external
        view
        returns (AuctionRegistryItem memory)
    {
        require(index < auctions.length, "Index out of bounds");
        return auctionRegistry[auctions[index]];
    }

    function getAuctionsPaginated(uint256 offset, uint256 limit)
        external
        view
        returns (AuctionRegistryItem[] memory)
    {
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

    function getAuctionsBySeller(address seller)
        external
        view
        returns (address[] memory)
    {
        return auctionsBySeller[seller];
    }

    function isRegisteredAuction(address auctionAddress)
        external
        view
        returns (bool)
    {
        return auctionRegistry[auctionAddress].exists;
    }
}