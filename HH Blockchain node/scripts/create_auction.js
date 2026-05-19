import hre from "hardhat";

async function main() {

  const { ethers } = await hre.network.create();

  const factoryAddress = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

  const factory = await ethers.getContractAt(
    "AuctionFactory",
    factoryAddress
  );

const biddingTimeSeconds = 3600;
const startingBid = ethers.parseEther("0.1");
const confirmationWindow = 259200; // 3 days

const tx = await factory.createAuction(
  biddingTimeSeconds,
  startingBid,
  confirmationWindow
);
  const receipt = await tx.wait();

  const event = receipt.logs.find(
    (log) => log.fragment && log.fragment.name === "AuctionCreated"
  );

  console.log("New auction:", event.args.auctionAddress);
}

main().catch(console.error);
