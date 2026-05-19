import hre from "hardhat";

async function main() {
  const { ethers } = await hre.network.create();

  const factory = await ethers.deployContract("AuctionFactory");
  await factory.waitForDeployment();

  console.log("Factory deployed:", await factory.getAddress());
}

main().catch(console.error);
