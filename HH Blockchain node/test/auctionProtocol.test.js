/*
 * auctionProtocol.test.js
 *
 * Purpose:
 *     Hardhat/Mocha tests for the T.R.U.S.T Auction Protocol smart contracts.
 *
 * Responsibilities:
 *     - Test AuctionFactory deployment and auction creation
 *     - Test registry read behavior
 *     - Test bidding behavior
 *     - Test refund withdrawal behavior
 *     - Test escrow settlement flows
 *
 * Notes:
 *     These tests use Hardhat 3's network connection API.
 */

import { expect } from "chai";
import { network } from "hardhat";

/* -------------------------------------------------------------------------- */
/*                              Test Constants                                */
/* -------------------------------------------------------------------------- */

const BIDDING_TIME_SECONDS = 3600;
const CONFIRMATION_WINDOW_SECONDS = 259200;

const STARTING_BID = 1n * 10n ** 18n;
const FIRST_BID = 2n * 10n ** 18n;
const SECOND_BID = 3n * 10n ** 18n;

/* -------------------------------------------------------------------------- */
/*                              Helper Functions                              */
/* -------------------------------------------------------------------------- */

/**
 * Expects an async contract call to revert.
 *
 * This avoids relying on a specific revert matcher plugin.
 *
 * @param {Promise<unknown>} promise
 */
async function expectRevert(promise) {
    let reverted = false;

    try {
        await promise;
    } catch {
        reverted = true;
    }

    expect(reverted).to.equal(true);
}

/**
 * Advances local blockchain time and mines one block.
 *
 * @param {object} ethers
 * @param {number} seconds
 */
async function advanceTime(ethers, seconds) {
    await ethers.provider.send("evm_increaseTime", [seconds]);
    await ethers.provider.send("evm_mine", []);
}

/**
 * Deploys a fresh AuctionFactory.
 *
 * @param {object} ethers
 * @returns {Promise<object>}
 */
async function deployFactory(ethers) {
    const AuctionFactory =
        await ethers.getContractFactory("AuctionFactory");

    const factory =
        await AuctionFactory.deploy();

    await factory.waitForDeployment();

    return factory;
}

/**
 * Creates a fresh auction through the factory and returns
 * both the registry item and SimpleAuction contract instance.
 *
 * @param {object} ethers
 * @param {object} factory
 * @param {object} seller
 * @param {string} title
 * @returns {Promise<{ item: object, auction: object }>}
 */
async function createAuction(
    ethers,
    factory,
    seller,
    title = "Test Auction"
) {
    const index =
        await factory.auctionCount();

    const tx =
        await factory
            .connect(seller)
            .createAuction(
                BIDDING_TIME_SECONDS,
                STARTING_BID,
                CONFIRMATION_WINDOW_SECONDS,
                title,
                "Created from Hardhat smart contract tests"
            );

    await tx.wait();

    const item =
        await factory.getAuctionByIndex(index);

    const auction =
        await ethers.getContractAt(
            "SimpleAuction",
            item.auctionAddress
        );

    return {
        item,
        auction,
    };
}

/**
 * Creates a fresh auction and places one winning bid.
 *
 * @param {object} ethers
 * @param {object} factory
 * @param {object} seller
 * @param {object} bidder
 * @returns {Promise<object>}
 */
async function createAuctionWithBid(
    ethers,
    factory,
    seller,
    bidder
) {
    const { auction } =
        await createAuction(
            ethers,
            factory,
            seller,
            "Auction With Bid"
        );

    await auction
        .connect(bidder)
        .bid({
            value: FIRST_BID,
        });

    return auction;
}

/* -------------------------------------------------------------------------- */
/*                              AuctionFactory                                */
/* -------------------------------------------------------------------------- */

describe("AuctionFactory", function () {
    let ethers;

    let owner;
    let seller;
    let bidder;

    let factory;

    beforeEach(async function () {
        const connection =
            await network.create();

        ethers =
            connection.ethers;

        [owner, seller, bidder] =
            await ethers.getSigners();

        factory =
            await deployFactory(ethers);
    });

    it("deploys successfully", async function () {
        const address =
            await factory.getAddress();

        expect(address).to.not.equal(
            ethers.ZeroAddress
        );
    });

    it("creates and registers an auction", async function () {
        const countBefore =
            await factory.auctionCount();

        const { item } =
            await createAuction(
                ethers,
                factory,
                seller,
                "Factory Registry Test"
            );

        const countAfter =
            await factory.auctionCount();

        expect(countAfter).to.equal(
            countBefore + 1n
        );

        expect(item.auctionAddress).to.not.equal(
            ethers.ZeroAddress
        );

        expect(item.seller).to.equal(
            seller.address
        );

        expect(item.startingBid).to.equal(
            STARTING_BID
        );

        expect(item.confirmationWindow).to.equal(
            BigInt(CONFIRMATION_WINDOW_SECONDS)
        );

        expect(item.title).to.equal(
            "Factory Registry Test"
        );

        expect(item.exists).to.equal(true);

        const isRegistered =
            await factory.isRegisteredAuction(
                item.auctionAddress
            );

        expect(isRegistered).to.equal(true);
    });

    it("returns auctions by seller", async function () {
        const { item } =
            await createAuction(
                ethers,
                factory,
                seller,
                "Seller Query Test"
            );

        const sellerAuctions =
            await factory.getAuctionsBySeller(
                seller.address
            );

        expect(sellerAuctions).to.include(
            item.auctionAddress
        );
    });

    it("returns paginated auction registry entries", async function () {
        const { item } =
            await createAuction(
                ethers,
                factory,
                seller,
                "Pagination Test"
            );

        const page =
            await factory.getAuctionsPaginated(
                0,
                10
            );

        expect(page.length).to.be.greaterThan(0);

        const addresses =
            page.map((entry) => entry.auctionAddress);

        expect(addresses).to.include(
            item.auctionAddress
        );
    });
});

/* -------------------------------------------------------------------------- */
/*                              Bidding Logic                                 */
/* -------------------------------------------------------------------------- */

describe("SimpleAuction bidding", function () {
    let ethers;

    let seller;
    let bidder;
    let secondBidder;

    let factory;

    beforeEach(async function () {
        const connection =
            await network.create();

        ethers =
            connection.ethers;

        [, seller, bidder, secondBidder] =
            await ethers.getSigners();

        factory =
            await deployFactory(ethers);
    });

    it("accepts a valid bid and updates the highest bid", async function () {
        const { auction } =
            await createAuction(
                ethers,
                factory,
                seller,
                "Bid Test"
            );

        await auction
            .connect(bidder)
            .bid({
                value: FIRST_BID,
            });

        expect(await auction.highestBid()).to.equal(
            FIRST_BID
        );

        expect(await auction.highestBidder()).to.equal(
            bidder.address
        );
    });

    it("rejects a bid lower than the current highest bid", async function () {
        const { auction } =
            await createAuction(
                ethers,
                factory,
                seller,
                "Low Bid Test"
            );

        await auction
            .connect(bidder)
            .bid({
                value: FIRST_BID,
            });

        await expectRevert(
            auction
                .connect(secondBidder)
                .bid({
                    value: STARTING_BID,
                })
        );
    });

    it("moves the previous highest bid into pending returns", async function () {
        const { auction } =
            await createAuction(
                ethers,
                factory,
                seller,
                "Outbid Test"
            );

        await auction
            .connect(bidder)
            .bid({
                value: FIRST_BID,
            });

        await auction
            .connect(secondBidder)
            .bid({
                value: SECOND_BID,
            });

        const pending =
            await auction.pendingReturns(
                bidder.address
            );

        expect(pending).to.equal(FIRST_BID);

        expect(await auction.highestBid()).to.equal(
            SECOND_BID
        );

        expect(await auction.highestBidder()).to.equal(
            secondBidder.address
        );
    });

    it("allows an outbid bidder to withdraw pending returns", async function () {
        const { auction } =
            await createAuction(
                ethers,
                factory,
                seller,
                "Withdraw Test"
            );

        await auction
            .connect(bidder)
            .bid({
                value: FIRST_BID,
            });

        await auction
            .connect(secondBidder)
            .bid({
                value: SECOND_BID,
            });

        await auction
            .connect(bidder)
            .withdraw();

        const pendingAfter =
            await auction.pendingReturns(
                bidder.address
            );

        expect(pendingAfter).to.equal(0n);
    });

    it("rejects bids after the auction has ended", async function () {
        const { auction } =
            await createAuction(
                ethers,
                factory,
                seller,
                "Late Bid Test"
            );

        await advanceTime(
            ethers,
            BIDDING_TIME_SECONDS + 1
        );

        await expectRevert(
            auction
                .connect(bidder)
                .bid({
                    value: FIRST_BID,
                })
        );
    });
});

/* -------------------------------------------------------------------------- */
/*                              Escrow Logic                                  */
/* -------------------------------------------------------------------------- */

describe("Auction escrow settlement", function () {
    let ethers;

    let owner;
    let seller;
    let bidder;

    let factory;

    beforeEach(async function () {
        const connection =
            await network.create();

        ethers =
            connection.ethers;

        [owner, seller, bidder] =
            await ethers.getSigners();

        factory =
            await deployFactory(ethers);
    });

    it("moves into settlement after ending an auction with a winning bid", async function () {
        const auction =
            await createAuctionWithBid(
                ethers,
                factory,
                seller,
                bidder
            );

        await advanceTime(
            ethers,
            BIDDING_TIME_SECONDS + 1
        );

        await auction.endAuction();

        const status =
            await auction.getEscrowStatus();

        expect(status).to.not.equal(0n);
    });

    it("allows the highest bidder to confirm receipt", async function () {
        const auction =
            await createAuctionWithBid(
                ethers,
                factory,
                seller,
                bidder
            );

        await advanceTime(
            ethers,
            BIDDING_TIME_SECONDS + 1
        );

        await auction.endAuction();

        expect(
            await auction.canConfirmReceipt(
                bidder.address
            )
        ).to.equal(true);

        await auction
            .connect(bidder)
            .confirmReceipt();

        expect(await auction.getEscrowStatus()).to.equal(
            3n
        );
    });

    it("allows the seller to claim after the confirmation timeout", async function () {
        const auction =
            await createAuctionWithBid(
                ethers,
                factory,
                seller,
                bidder
            );

        await advanceTime(
            ethers,
            BIDDING_TIME_SECONDS + 1
        );

        await auction.endAuction();

        await advanceTime(
            ethers,
            CONFIRMATION_WINDOW_SECONDS + 1
        );

        expect(
            await auction.canClaimTimeout(
                seller.address
            )
        ).to.equal(true);

        await auction
            .connect(seller)
            .claimAfterTimeout();

        expect(await auction.getEscrowStatus()).to.equal(
            3n
        );
    });

    it("allows admin refund flagging and buyer withdrawal", async function () {
        const auction =
            await createAuctionWithBid(
                ethers,
                factory,
                seller,
                bidder
            );

        await advanceTime(
            ethers,
            BIDDING_TIME_SECONDS + 1
        );

        await auction.endAuction();

        expect(
            await auction.canFlagRefund(
                owner.address
            )
        ).to.equal(true);

        await auction
            .connect(owner)
            .flagRefund();

        expect(await auction.getEscrowStatus()).to.equal(
            4n
        );

        const pendingBefore =
            await auction.pendingReturns(
                bidder.address
            );

        expect(pendingBefore).to.equal(
            FIRST_BID
        );

        await auction
            .connect(bidder)
            .withdraw();

        const pendingAfter =
            await auction.pendingReturns(
                bidder.address
            );

        expect(pendingAfter).to.equal(0n);
    });
});
