import { expect } from "chai";
import { network } from "hardhat";

describe("AuctionFactory", function () {
    let ethers;
    let factory;

    let owner;
    let seller;
    let bidder;

    beforeEach(async function () {
        const connection = await network.connect();
        ethers = connection.ethers;

        [owner, seller, bidder] =
            await ethers.getSigners();

        const AuctionFactory =
            await ethers.getContractFactory(
                "AuctionFactory"
            );

        factory =
            await AuctionFactory.deploy();

        await factory.waitForDeployment();
    });

    it("deploys successfully", async function () {
        const address =
            await factory.getAddress();

        expect(address).to.not.equal(
            ethers.ZeroAddress
        );
    });

    it("creates an auction", async function () {
        const tx =
            await factory
                .connect(seller)
                .createAuction(
                    3600,
                    ethers.parseEther("1"),
                    259200,
                    "Test Auction",
                    "Created from Hardhat test"
                );

        await tx.wait();

        const count =
            await factory.auctionCount();

        expect(count).to.equal(1n);
    });
});