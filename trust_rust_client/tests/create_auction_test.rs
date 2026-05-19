/*
 * create_auction_test.rs
 *
 * Purpose:
 *     Integration test for on-chain auction creation through AuctionFactory.
 *
 *     This test verifies that create_auction.rs can:
 *         - Connect to the configured Ethereum node
 *         - Call AuctionFactory.createAuction(...)
 *         - Wait for the transaction receipt
 *         - Decode the AuctionCreated event
 *         - Return the deployed auction address and event metadata
 *
 *     It does NOT:
 *         - Test bidding behavior
 *         - Test frontend/Axum routes
 *         - Deploy the AuctionFactory itself
 *
 * Requirements:
 *     A local Hardhat node or compatible Ethereum RPC must be running.
 *     AuctionFactory must already be deployed.
 *
 * Expected .env values:
 *     RPC_URL
 *     FACTORY_ADDRESS
 *     SELLER_ADDRESS
 *
 * Optional .env value:
 *     CONFIRMATION_WINDOW
 */

use alloy::primitives::U256;
use trust_rust_client::{
    auction_loader::{connect_provider, parse_address},
    config::{
        get_confirmation_window,
        get_factory_address,
        get_rpc_url,
        get_seller_address,
    },
    create_auction::create_auction,
};

#[tokio::test]
async fn test_create_auction_emits_created_event() {
    dotenvy::dotenv().ok();

    println!("\n================ CREATE AUCTION TEST START ================");

    let rpc_url = get_rpc_url()
        .expect("RPC_URL missing");

    let factory_address = parse_address(
        &get_factory_address().expect("FACTORY_ADDRESS missing")
    )
    .expect("invalid factory address");

    let seller_address = parse_address(
        &get_seller_address().expect("SELLER_ADDRESS missing")
    )
    .expect("invalid seller address");

    println!("RPC URL:         {}", rpc_url);
    println!("Factory address: {:?}", factory_address);
    println!("Seller address:  {:?}", seller_address);

    let provider = connect_provider(&rpc_url)
        .await
        .expect("failed to connect provider");

    println!("\nProvider connected successfully.");

    let bidding_time_seconds = U256::from(3_600u64);
    let starting_bid_wei = U256::from(1_000_000_000_000_000_000u128);
    let confirmation_window = U256::from(
        get_confirmation_window().expect("invalid CONFIRMATION_WINDOW")
    );

    println!("\nAuction creation parameters:");
    println!("  bidding_time_seconds: {}", bidding_time_seconds);
    println!("  starting_bid_wei:     {}", starting_bid_wei);
    println!("  confirmation_window:  {}", confirmation_window);

    println!("\nSending createAuction transaction...");

    let result = create_auction(
        &provider,
        factory_address,
        seller_address,
        bidding_time_seconds,
        starting_bid_wei,
        confirmation_window,
    )
    .await
    .expect("create_auction failed");

    println!("\n================ AUCTION CREATED ================");
    println!("{:#?}", result);

    println!("\nValidating returned auction data...");

    assert_ne!(
        result.auction_address,
        alloy::primitives::Address::ZERO,
        "auction address should not be zero"
    );

    assert_eq!(
        result.seller,
        seller_address,
        "seller address should match transaction sender"
    );

    assert_eq!(
        result.bidding_time_seconds,
        bidding_time_seconds,
        "bidding duration should match input"
    );

    assert_eq!(
        result.starting_bid_wei,
        starting_bid_wei,
        "starting bid should match input"
    );

    assert_eq!(
        result.confirmation_window,
        confirmation_window,
        "confirmation window should match input"
    );

    assert!(
        result.end_time > U256::ZERO,
        "auction end time should be greater than zero"
    );

    assert_ne!(
        result.admin,
        alloy::primitives::Address::ZERO,
        "admin address should not be zero"
    );

    assert!(
        !result.tx_hash.is_empty(),
        "transaction hash should not be empty"
    );

    println!("\nValidation passed.");
    println!("Auction address to use in AUCTION_ADDRESS:");
    println!("{:?}", result.auction_address);

    println!("\n================ CREATE AUCTION TEST PASSED ================\n");
}