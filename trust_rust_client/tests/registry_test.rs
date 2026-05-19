/*
 * registry_test.rs
 *
 * Purpose:
 *     Integration test for AuctionFactory registry read/query calls.
 *
 *     This test verifies that registry.rs can:
 *         - Connect to the configured Ethereum node
 *         - Read the total auction count from AuctionFactory
 *         - Create a new auction through AuctionFactory
 *         - Confirm the registry count increases
 *         - Confirm the created auction is registered
 *         - Read registry metadata by address, index, seller, and pagination
 *
 *     It does NOT:
 *         - Test bidding behavior
 *         - Deploy the AuctionFactory itself
 *         - Test frontend/Axum routes
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
 *
 * System Position:
 *
 *     cargo test
 *         ↓
 *     registry_test.rs  ← THIS FILE (integration test)
 *         ↓
 *     ├── create_auction.rs  (factory write transaction)
 *     └── registry.rs        (factory registry view calls)
 *             ↓
 *         AuctionFactory.sol / AuctionRegistry.sol
 *             ↓
 *         Hardhat / Ethereum node
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
    registry::{
        get_auction_by_index,
        get_auction_count,
        get_auction_registry_item,
        get_auctions_by_seller,
        get_auctions_paginated,
        is_registered_auction,
    },
};

#[tokio::test]
async fn test_registry_reads_created_auction() {
    // Load .env values for the test environment.
    dotenvy::dotenv().ok();

    // Load and validate the RPC URL.
    let rpc_url = get_rpc_url()
        .expect("RPC_URL missing");

    // Parse the deployed AuctionFactory address.
    let factory_address = parse_address(
        &get_factory_address().expect("FACTORY_ADDRESS missing")
    )
    .expect("invalid factory address");

    // Parse the seller address.
    let seller_address = parse_address(
        &get_seller_address().expect("SELLER_ADDRESS missing")
    )
    .expect("invalid seller address");

    // Connect to the configured Ethereum node.
    let provider = connect_provider(&rpc_url)
        .await
        .expect("failed to connect provider");

    // Read the registry count before creating a new auction.
    // Read the registry count before creating a new auction.
    //
    // This gives us a stable index for the auction created inside this test.
    let count_before = get_auction_count(
        &provider,
        factory_address,
    )
    .await
    .expect("auctionCount call failed");

    println!("\n================ REGISTRY TEST START ================");
    println!("Factory address: {:?}", factory_address);
    println!("Seller address:  {:?}", seller_address);
    println!("Auction count before create: {}", count_before);

    // Define test auction parameters.
    let bidding_time_seconds = U256::from(3_600u64);
    let starting_bid_wei = U256::from(1_000_000_000_000_000_000u128);
    let confirmation_window = U256::from(
        get_confirmation_window().expect("invalid CONFIRMATION_WINDOW")
    );

    println!("\nCreating auction with:");
    println!("  bidding_time_seconds: {}", bidding_time_seconds);
    println!("  starting_bid_wei:     {}", starting_bid_wei);
    println!("  confirmation_window:  {}", confirmation_window);

    // Create a new auction through the factory.
    //
    // This should write a new registry entry inside AuctionRegistry.sol.
    let created = create_auction(
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
    println!("Transaction hash: {}", created.tx_hash);
    println!("Auction address:  {:?}", created.auction_address);
    println!("Seller:           {:?}", created.seller);
    println!("Auction end time: {}", created.end_time);

    // Read the registry count after creation.
    let count_after = get_auction_count(
        &provider,
        factory_address,
    )
    .await
    .expect("auctionCount after create failed");

    println!("\nAuction count after create: {}", count_after);

    // Confirm the registry grew by exactly one auction.
    assert_eq!(
        count_after,
        count_before + U256::from(1u64)
    );

    // Confirm the created auction is marked as registered.
    let exists = is_registered_auction(
        &provider,
        factory_address,
        created.auction_address,
    )
    .await
    .expect("isRegisteredAuction call failed");

    println!("\nIs registered auction: {}", exists);

    assert!(exists, "created auction should be registered");

    // Read the registry item directly by auction address.
    let item_by_address = get_auction_registry_item(
        &provider,
        factory_address,
        created.auction_address,
    )
    .await
    .expect("getAuctionRegistryItem call failed");

    println!("\n================ REGISTRY ITEM BY ADDRESS ================");
    println!("{:#?}", item_by_address);

    assert_eq!(item_by_address.auction_address, created.auction_address);
    assert_eq!(item_by_address.seller, seller_address);
    assert_eq!(item_by_address.bidding_time_seconds, bidding_time_seconds);
    assert_eq!(item_by_address.starting_bid_wei, starting_bid_wei);
    assert_eq!(item_by_address.confirmation_window, confirmation_window);
    assert!(item_by_address.exists);

    // Read the same item by index.
    //
    // Since count_before was captured before creation, the new auction
    // should be stored at index count_before.
    let item_by_index = get_auction_by_index(
        &provider,
        factory_address,
        count_before,
    )
    .await
    .expect("getAuctionByIndex call failed");

    println!("\n================ REGISTRY ITEM BY INDEX ================");
    println!("{:#?}", item_by_index);

    assert_eq!(item_by_index.auction_address, created.auction_address);
    assert_eq!(item_by_index.seller, seller_address);

    // Read a one-item page starting at the new auction's index.
    let page = get_auctions_paginated(
        &provider,
        factory_address,
        count_before,
        U256::from(1u64),
    )
    .await
    .expect("getAuctionsPaginated call failed");

    println!("\n================ PAGINATED RESULTS ================");
    println!("{:#?}", page);

    assert_eq!(page.len(), 1);
    assert_eq!(page[0].auction_address, created.auction_address);

    // Read all auctions created by the seller and confirm the new auction appears.
    let seller_auctions = get_auctions_by_seller(
        &provider,
        factory_address,
        seller_address,
    )
    .await
    .expect("getAuctionsBySeller call failed");

    println!("\n================ SELLER AUCTIONS ================");
    println!("{:#?}", seller_auctions);

    assert!(
        seller_auctions.contains(&created.auction_address),
        "seller auction list should contain created auction"
    );

    println!("\n================ REGISTRY TEST PASSED ================\n");
}