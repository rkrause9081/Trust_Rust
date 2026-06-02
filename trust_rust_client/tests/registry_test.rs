/*
 * registry_test.rs
 *
 * Purpose:
 *     Independent integration test for AuctionFactory registry
 *     read/query calls.
 *
 *     This test creates a new auction and verifies that all
 *     registry read operations correctly return information
 *     about the newly registered auction.
 *
 * Responsibilities:
 *     - Create a test auction
 *     - Verify registry count updates
 *     - Verify auction registration status
 *     - Validate registry item retrieval methods
 *     - Verify seller registry queries
 *     - Verify paginated registry queries
 *
 * Non-Responsibilities:
 *     - Auction creation implementation
 *     - Registry contract implementation
 *     - Provider configuration
 *     - Environment variable management
 *
 * Architecture:
 *
 *     registry_test.rs
 *             ↓
 *      common test utilities
 *             ↓
 *      trust_rust_client::registry
 *             ↓
 *        AuctionFactory
 *             ↓
 *      Auction Registry Storage
 */

mod common;

use alloy::primitives::U256;
use common::{
    connect_test_provider, create_test_auction, get_factory_test_address, get_seller_test_address,
    load_test_env,
};
use trust_rust_client::registry::{
    get_auction_by_index, get_auction_count, get_auction_registry_item, get_auctions_by_seller,
    get_auctions_paginated, is_registered_auction,
};

/* -------------------------------------------------------------------------- */
/*                         Registry Query Integration Test                    */
/* -------------------------------------------------------------------------- */

/**
 * Tests registry read operations against a newly created auction.
 *
 * Creates a fresh auction and verifies that every supported
 * registry query correctly returns the expected auction data.
 *
 * # Test Flow
 *
 * 1. Load test environment variables.
 * 2. Connect to the configured blockchain provider.
 * 3. Read the current auction registry count.
 * 4. Create a new auction.
 * 5. Verify registry count increased.
 * 6. Verify auction registration status.
 * 7. Verify registry lookup by address.
 * 8. Verify registry lookup by index.
 * 9. Verify paginated registry queries.
 * 10. Verify seller-specific registry queries.
 *
 * # Assertions
 *
 * Verifies that:
 *     - Registry count increases by one
 *     - Auction is marked as registered
 *     - Registry item data matches created auction data
 *     - Index lookup returns the correct auction
 *     - Pagination returns expected results
 *     - Seller lookup contains the created auction
 */
#[tokio::test]
async fn test_registry_reads_created_auction() {
    // Load required environment variables for testing.
    load_test_env();

    println!("\n================ REGISTRY TEST START ================");

    // Connect to the configured blockchain provider.
    let provider = connect_test_provider()
        .await
        .expect("failed to connect provider");

    // Load test addresses used for registry operations.
    let factory_address = get_factory_test_address();
    let seller_address = get_seller_test_address();

    // Read the current registry count before creating an auction.
    let count_before = get_auction_count(&provider, factory_address)
        .await
        .expect("auctionCount call failed");

    // Create a fresh auction that should appear in the registry.
    let created = create_test_auction(
        &provider,
        factory_address,
        seller_address,
        "Registry Test Auction",
    )
    .await;

    // Read the registry count after auction creation.
    let count_after = get_auction_count(&provider, factory_address)
        .await
        .expect("auctionCount after create failed");

    // Verify registry count increased by exactly one.
    assert_eq!(
        count_after,
        count_before + U256::from(1u64),
        "registry count should increase by one"
    );

    // Verify the created auction is registered.
    let exists = is_registered_auction(&provider, factory_address, created.auction_address)
        .await
        .expect("isRegisteredAuction call failed");

    assert!(exists, "created auction should be registered");

    // Retrieve registry information using the auction address.
    let item_by_address =
        get_auction_registry_item(&provider, factory_address, created.auction_address)
            .await
            .expect("getAuctionRegistryItem call failed");

    // Verify registry address matches the created auction.
    assert_eq!(item_by_address.auction_address, created.auction_address);

    // Verify seller information is correct.
    assert_eq!(item_by_address.seller, seller_address);

    // Verify bidding duration matches auction creation data.
    assert_eq!(
        item_by_address.bidding_time_seconds,
        created.bidding_time_seconds
    );

    // Verify starting bid matches auction creation data.
    assert_eq!(item_by_address.starting_bid_wei, created.starting_bid_wei);

    // Verify confirmation window matches auction creation data.
    assert_eq!(
        item_by_address.confirmation_window,
        created.confirmation_window
    );

    // Verify registry item existence flag is set.
    assert!(item_by_address.exists, "registry item should exist");

    // Retrieve registry information using the auction index.
    let item_by_index = get_auction_by_index(&provider, factory_address, count_before)
        .await
        .expect("getAuctionByIndex call failed");

    // Verify index lookup returns the created auction.
    assert_eq!(item_by_index.auction_address, created.auction_address);

    // Retrieve a paginated registry result set.
    let page = get_auctions_paginated(
        &provider,
        factory_address,
        count_before,
        U256::from(1u64),
    )
    .await
    .expect("getAuctionsPaginated call failed");

    // Verify pagination returned one result.
    assert_eq!(page.len(), 1);

    // Verify pagination returned the correct auction.
    assert_eq!(page[0].auction_address, created.auction_address);

    // Retrieve all auctions associated with the seller.
    let seller_auctions = get_auctions_by_seller(&provider, factory_address, seller_address)
        .await
        .expect("getAuctionsBySeller call failed");

    // Verify seller registry contains the newly created auction.
    assert!(
        seller_auctions.contains(&created.auction_address),
        "seller auction list should contain created auction"
    );

    println!("\n================ REGISTRY TEST PASSED ================\n");
}