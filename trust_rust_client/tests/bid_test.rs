/*
 * bid_test.rs
 *
 * Purpose:
 *     Integration test for on-chain bid placement.
 *
 *     This test verifies that bidding.rs can:
 *         - Connect to the configured Ethereum node
 *         - Call bid() on an existing SimpleAuction contract
 *         - Wait for the transaction receipt
 *         - Read highestBid() after the transaction
 *         - Confirm the returned bid result matches contract state
 *
 *     It does NOT:
 *         - Create a new auction
 *         - Deploy contracts
 *         - Test AuctionFactory behavior
 *         - Test frontend/Axum routes
 *
 * Requirements:
 *     A local Hardhat node or compatible Ethereum RPC must be running.
 *     A SimpleAuction contract must already be deployed.
 *
 * Expected .env values:
 *     RPC_URL
 *     AUCTION_ADDRESS
 *     BIDDER_ADDRESS
 */

use alloy::primitives::U256;
use trust_rust_client::{
    auction_loader::{connect_provider, parse_address},
    bidding::{get_highest_bid, place_bid},
};

#[tokio::test]
async fn test_place_bid_updates_highest_bid() {
    dotenvy::dotenv().ok();

    println!("\n================ BID TEST START ================");

    let rpc_url = std::env::var("RPC_URL")
        .expect("RPC_URL missing");

    let auction_address = parse_address(
        &std::env::var("AUCTION_ADDRESS").expect("AUCTION_ADDRESS missing")
    )
    .expect("invalid auction address");

    let bidder_address = parse_address(
        &std::env::var("BIDDER_ADDRESS").expect("BIDDER_ADDRESS missing")
    )
    .expect("invalid bidder address");

    println!("RPC URL:         {}", rpc_url);
    println!("Auction address: {:?}", auction_address);
    println!("Bidder address:  {:?}", bidder_address);

    let provider = connect_provider(&rpc_url)
        .await
        .expect("failed to connect provider");

    println!("\nProvider connected successfully.");

    let bid_amount = U256::from(2_000_000_000_000_000_000u128);

    println!("\nBid parameters:");
    println!("  bid_amount_wei: {}", bid_amount);

    let highest_before = get_highest_bid(&provider, auction_address)
        .await
        .expect("highestBid before bid failed");

    println!("\nHighest bid before transaction:");
    println!("  highest_before_wei: {}", highest_before);

    println!("\nSending bid transaction...");

    let result = place_bid(&provider, auction_address, bidder_address, bid_amount)
        .await
        .expect("place_bid failed");

    println!("\n================ BID TRANSACTION CONFIRMED ================");
    println!("{:#?}", result);

    let highest_after = get_highest_bid(&provider, auction_address)
        .await
        .expect("highestBid after bid failed");

    println!("\nHighest bid after transaction:");
    println!("  highest_after_wei: {}", highest_after);

    println!("\nValidating bid result...");

    assert_eq!(
        highest_after,
        result.highest_bid_wei,
        "highest bid returned by place_bid should match direct contract read"
    );

    assert!(
        highest_after >= bid_amount,
        "highest bid should be at least the submitted bid amount"
    );

    assert!(
        highest_after >= highest_before,
        "highest bid should not decrease after placing a bid"
    );

    assert!(
        !result.tx_hash.is_empty(),
        "transaction hash should not be empty"
    );

    println!("\nValidation passed.");
    println!("Bid transaction hash: {}", result.tx_hash);

    println!("\n================ BID TEST PASSED ================\n");
}