// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./Auction/SimpleAuction.sol";
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