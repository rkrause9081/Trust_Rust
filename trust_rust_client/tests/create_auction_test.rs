/*
 * create_auction_test.rs
 *
 * Purpose:
 *     Independent integration test for on-chain auction creation
 *     through AuctionFactory.
 *
 *     This test creates a new auction through the factory contract
 *     and verifies that the emitted AuctionCreated event contains
 *     the expected values.
 *
 * Responsibilities:
 *     - Load test environment configuration
 *     - Connect to the blockchain provider
 *     - Create a test auction
 *     - Verify decoded event data
 *     - Validate returned transaction metadata
 *
 * Non-Responsibilities:
 *     - Contract deployment
 *     - Factory implementation
 *     - Provider initialization logic
 *     - Environment variable definition
 *
 * Architecture:
 *
 *     create_auction_test.rs
 *                ↓
 *         common test utilities
 *                ↓
 *         create_test_auction()
 *                ↓
 *         AuctionFactory Contract
 *                ↓
 *         AuctionCreated Event
 */

mod common;

use alloy::primitives::{Address, U256};
use common::{
    DEFAULT_BIDDING_TIME_SECONDS, DEFAULT_STARTING_BID_WEI, connect_test_provider,
    create_test_auction, get_factory_test_address, get_seller_test_address, load_test_env,
};

/* -------------------------------------------------------------------------- */
/*                     Auction Creation Integration Test                       */
/* -------------------------------------------------------------------------- */

/**
 * Tests that auction creation emits a valid AuctionCreated event.
 *
 * Creates a new auction through the factory contract and verifies
 * that the returned event data matches the values supplied during
 * auction creation.
 *
 * # Test Flow
 *
 * 1. Load test environment variables.
 * 2. Connect to the configured blockchain provider.
 * 3. Retrieve factory and seller test addresses.
 * 4. Create a new auction.
 * 5. Validate all decoded event fields.
 * 6. Verify transaction metadata.
 *
 * # Assertions
 *
 * Verifies that:
 *     - Auction address is not zero
 *     - Seller matches the transaction sender
 *     - Bidding duration matches the configured input
 *     - Starting bid matches the configured input
 *     - Confirmation window is valid
 *     - Auction end time is valid
 *     - Admin address is valid
 *     - Transaction hash is populated
 */
#[tokio::test]
async fn test_create_auction_emits_created_event() {
    // Load required environment variables for testing.
    load_test_env();

    println!("\n================ CREATE AUCTION TEST START ================");

    // Connect to the configured blockchain provider.
    let provider = connect_test_provider()
        .await
        .expect("failed to connect provider");

    // Load test addresses used for auction creation.
    let factory_address = get_factory_test_address();
    let seller_address = get_seller_test_address();

    // Create a new test auction through the factory contract.
    let result = create_test_auction(
        &provider,
        factory_address,
        seller_address,
        "Create Auction Test",
    )
    .await;

    println!("\n================ AUCTION CREATED ================");
    println!("{:#?}", result);

    // Verify the created auction address is valid.
    assert_ne!(
        result.auction_address,
        Address::ZERO,
        "auction address should not be zero"
    );

    // Verify the seller matches the transaction sender.
    assert_eq!(
        result.seller, seller_address,
        "seller address should match transaction sender"
    );

    // Verify the bidding duration matches the supplied input.
    assert_eq!(
        result.bidding_time_seconds,
        U256::from(DEFAULT_BIDDING_TIME_SECONDS),
        "bidding duration should match input"
    );

    // Verify the starting bid matches the supplied input.
    assert_eq!(
        result.starting_bid_wei,
        U256::from(DEFAULT_STARTING_BID_WEI),
        "starting bid should match input"
    );

    // Verify a valid confirmation window was assigned.
    assert!(
        result.confirmation_window > U256::ZERO,
        "confirmation window should be greater than zero"
    );

    // Verify a valid auction end time was generated.
    assert!(
        result.end_time > U256::ZERO,
        "auction end time should be greater than zero"
    );

    // Verify the admin address is valid.
    assert_ne!(
        result.admin,
        Address::ZERO,
        "admin address should not be zero"
    );

    // Verify a transaction hash was returned.
    assert!(
        !result.tx_hash.is_empty(),
        "transaction hash should not be empty"
    );

    println!("\n================ CREATE AUCTION TEST PASSED ================\n");
}