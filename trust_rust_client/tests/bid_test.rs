/*
 * bid_test.rs
 *
 * Purpose:
 *     Independent integration test for on-chain bid placement.
 *
 *     This test creates a fresh auction, places a bid, then verifies
 *     that the highest bid was updated correctly.
 *
 * Responsibilities:
 *     - Load the test environment
 *     - Connect to the blockchain provider
 *     - Create a test auction
 *     - Submit a bid transaction
 *     - Verify highest bid state changes
 *
 * Non-Responsibilities:
 *     - Auction contract deployment
 *     - Provider implementation
 *     - Environment variable definition
 *     - Bid function implementation
 *
 * Architecture:
 *
 *     bid_test.rs
 *          ↓
 *      common test utilities
 *          ↓
 *      trust_rust_client::bidding
 *          ↓
 *      Auction Contract
 */

mod common;

use alloy::primitives::U256;
use common::{
    DEFAULT_BID_AMOUNT_WEI, connect_test_provider, create_test_auction, get_bidder_address,
    get_factory_test_address, get_seller_test_address, load_test_env,
};
use trust_rust_client::bidding::{get_highest_bid, place_bid};

/* -------------------------------------------------------------------------- */
/*                              Bid Integration Test                          */
/* -------------------------------------------------------------------------- */

/**
 * Tests that placing a bid updates the auction highest bid.
 *
 * Creates a fresh test auction, reads the current highest bid,
 * submits a bid transaction, then verifies that the contract state
 * reflects the new highest bid.
 *
 * # Test Flow
 *
 * 1. Load test environment variables.
 * 2. Connect to the configured test provider.
 * 3. Retrieve factory, seller, and bidder test addresses.
 * 4. Create a fresh auction for the bid test.
 * 5. Read the highest bid before placing a bid.
 * 6. Submit a bid transaction.
 * 7. Read the highest bid after placing the bid.
 * 8. Assert that bid state and transaction metadata are valid.
 *
 * # Assertions
 *
 * Verifies that:
 *     - The highest bid returned from `place_bid()` matches
 *       a direct contract read
 *     - The highest bid is at least the submitted bid amount
 *     - The highest bid does not decrease
 *     - The transaction hash is not empty
 */
#[tokio::test]
async fn test_place_bid_updates_highest_bid() {
    // Load required environment variables for the test network.
    load_test_env();

    println!("\n================ BID TEST START ================");

    // Connect to the configured blockchain provider.
    let provider = connect_test_provider()
        .await
        .expect("failed to connect provider");

    // Load test addresses used for auction creation and bidding.
    let factory_address = get_factory_test_address();
    let seller_address = get_seller_test_address();
    let bidder_address = get_bidder_address();

    // Create a fresh auction so this test has isolated contract state.
    let created = create_test_auction(
        &provider,
        factory_address,
        seller_address,
        "Bid Test Auction",
    )
    .await;

    let auction_address = created.auction_address;
    let bid_amount = U256::from(DEFAULT_BID_AMOUNT_WEI);

    println!("Factory address: {:?}", factory_address);
    println!("Auction address: {:?}", auction_address);
    println!("Bidder address:  {:?}", bidder_address);

    // Read the highest bid before submitting the bid transaction.
    let highest_before = get_highest_bid(&provider, auction_address)
        .await
        .expect("highestBid before bid failed");

    println!("\nHighest bid before transaction:");
    println!("  highest_before_wei: {}", highest_before);

    // Submit the bid transaction to the auction contract.
    let result = place_bid(&provider, auction_address, bidder_address, bid_amount)
        .await
        .expect("place_bid failed");

    println!("\n================ BID TRANSACTION CONFIRMED ================");
    println!("{:#?}", result);

    // Read the highest bid again after the transaction is confirmed.
    let highest_after = get_highest_bid(&provider, auction_address)
        .await
        .expect("highestBid after bid failed");

    // Ensure the transaction result matches the contract's current state.
    assert_eq!(
        highest_after, result.highest_bid_wei,
        "highest bid returned by place_bid should match direct contract read"
    );

    // Ensure the highest bid is at least the amount submitted.
    assert!(
        highest_after >= bid_amount,
        "highest bid should be at least the submitted bid amount"
    );

    // Ensure bidding never causes the highest bid to decrease.
    assert!(
        highest_after >= highest_before,
        "highest bid should not decrease after placing a bid"
    );

    // Ensure a valid transaction hash was returned.
    assert!(
        !result.tx_hash.is_empty(),
        "transaction hash should not be empty"
    );

    println!("\n================ BID TEST PASSED ================\n");
}