// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./AuctionState.sol";
import "./AuctionSettlement.sol";
import "./AuctionEscrow.sol";

interface IAuctionRegistry {
    function updateHighestBid(
        address auctionAddress,
        address bidder,
        uint256 newBidAmount
    ) external;
}

contract SimpleAuction is AuctionState, AuctionSettlement, AuctionEscrow {
    address public immutable registry;

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