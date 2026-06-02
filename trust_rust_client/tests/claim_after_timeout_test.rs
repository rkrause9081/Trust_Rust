/*
 * claim_after_timeout_test.rs
 *
 * Purpose:
 *     Independent integration test for seller timeout settlement.
 *
 *     This test creates an auction with an active bid, advances
 *     blockchain time beyond the auction and confirmation periods,
 *     then verifies that the seller can successfully claim escrowed
 *     funds after buyer inactivity.
 *
 * Responsibilities:
 *     - Create a test auction with a valid bid
 *     - Advance blockchain time through auction phases
 *     - End the auction
 *     - Execute seller timeout settlement
 *     - Verify escrow settlement results
 *
 * Non-Responsibilities:
 *     - Auction deployment
 *     - Escrow contract implementation
 *     - Provider configuration
 *     - Environment variable management
 *
 * Architecture:
 *
 *     claim_after_timeout_test.rs
 *                ↓
 *         common test utilities
 *                ↓
 *       trust_rust_client::escrow
 *                ↓
 *         Auction Contract
 *                ↓
 *          Escrow Contract
 */

mod common;

use common::{
    DEFAULT_BIDDING_TIME_SECONDS, connect_test_provider, create_test_auction_with_bid,
    get_bidder_address, get_factory_test_address, get_seller_test_address, hardhat_advance_time,
    load_test_env, print_balance,
};
use trust_rust_client::escrow::{claim_after_timeout, end_auction};

/* -------------------------------------------------------------------------- */
/*                     Seller Timeout Settlement Integration Test             */
/* -------------------------------------------------------------------------- */

/**
 * Tests seller escrow settlement after buyer timeout.
 *
 * Creates a new auction with a valid bid, advances blockchain
 * time beyond both the bidding period and confirmation window,
 * then verifies that the seller can successfully claim the
 * escrowed funds.
 *
 * # Test Flow
 *
 * 1. Load test environment variables.
 * 2. Connect to the configured blockchain provider.
 * 3. Create a test auction with an active bid.
 * 4. Record account balances before settlement.
 * 5. Advance time until the auction can be ended.
 * 6. End the auction.
 * 7. Advance time past the buyer confirmation window.
 * 8. Execute seller timeout claim.
 * 9. Verify settlement state and transaction metadata.
 * 10. Display balances after settlement.
 *
 * # Assertions
 *
 * Verifies that:
 *     - The timeout claim transaction succeeds
 *     - A valid transaction hash is returned
 *     - Escrow is marked as settled
 */
#[tokio::test]
async fn test_claim_after_timeout() {
    // Load required environment variables for testing.
    load_test_env();

    println!("\n================ CLAIM AFTER TIMEOUT TEST START ================");

    // Connect to the configured blockchain provider.
    let provider = connect_test_provider()
        .await
        .expect("failed to connect provider");

    // Load test addresses used throughout the settlement flow.
    let factory_address = get_factory_test_address();
    let seller_address = get_seller_test_address();
    let buyer_address = get_bidder_address();

    // Create a fresh auction with an existing bid so escrow
    // settlement can be tested.
    let created = create_test_auction_with_bid(
        &provider,
        factory_address,
        seller_address,
        buyer_address,
        "Claim Timeout Test Auction",
    )
    .await;

    let auction_address = created.auction_address;

    println!("Auction address: {:?}", auction_address);
    println!("Buyer:           {:?}", buyer_address);
    println!("Seller:          {:?}", seller_address);

    // Display account balances before settlement occurs.
    println!("\n=== BALANCES BEFORE ===");
    print_balance(&provider, "Buyer", buyer_address).await;
    print_balance(&provider, "Seller", seller_address).await;

    println!("\nFast-forwarding time + ending auction...");

    // Advance blockchain time until the bidding period expires.
    hardhat_advance_time(&provider, DEFAULT_BIDDING_TIME_SECONDS)
        .await
        .expect("time advance failed");

    // End the auction after bidding has concluded.
    let _ = end_auction(&provider, auction_address, buyer_address)
        .await
        .expect("end_auction failed");

    println!("\nFast-forwarding past confirmation window...");

    // Advance blockchain time beyond the confirmation window
    // to simulate buyer inactivity.
    hardhat_advance_time(&provider, 259_200u64)
        .await
        .expect("timeout advance failed");

    // Allow the seller to claim escrowed funds after timeout.
    let result = claim_after_timeout(&provider, auction_address, seller_address)
        .await
        .expect("claim_after_timeout failed");

    // Verify a transaction hash was returned.
    assert!(
        !result.tx_hash.is_empty(),
        "transaction hash should not be empty"
    );

    // Verify escrow was successfully settled.
    assert!(result.escrow_settled, "escrow should be settled");

    println!("Claim result: {:#?}", result);

    // Display balances after escrow settlement.
    println!("\n=== BALANCES AFTER CLAIM ===");
    print_balance(&provider, "Buyer", buyer_address).await;
    print_balance(&provider, "Seller", seller_address).await;

    println!("\n================ CLAIM AFTER TIMEOUT TEST PASSED ================\n");
}