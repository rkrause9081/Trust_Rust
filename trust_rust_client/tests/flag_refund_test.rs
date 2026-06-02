/*
 * flag_refund_test.rs
 *
 * Purpose:
 *     Independent integration test for admin refund flow and
 *     buyer withdrawal.
 *
 *     This test creates an auction with a valid bid, ends the
 *     auction, allows the admin to flag the escrow for refund,
 *     then verifies that the buyer can successfully withdraw
 *     the refunded funds.
 *
 * Responsibilities:
 *     - Create a test auction with an active bid
 *     - Advance blockchain time beyond the bidding period
 *     - End the auction
 *     - Execute admin refund flagging
 *     - Execute buyer withdrawal
 *     - Verify refund and withdrawal state changes
 *
 * Non-Responsibilities:
 *     - Escrow contract implementation
 *     - Withdrawal logic implementation
 *     - Provider initialization
 *     - Environment variable management
 *
 * Architecture:
 *
 *     flag_refund_test.rs
 *               ↓
 *        common test utilities
 *               ↓
 *      trust_rust_client::escrow
 *               ↓
 *      trust_rust_client::withdraw
 *               ↓
 *         Auction Contract
 *               ↓
 *          Escrow Contract
 */

mod common;

use common::{
    DEFAULT_BIDDING_TIME_SECONDS, connect_test_provider, create_test_auction_with_bid,
    get_admin_test_address, get_bidder_address, get_factory_test_address, get_seller_test_address,
    hardhat_advance_time, load_test_env, print_balance,
};
use trust_rust_client::{
    escrow::{end_auction, flag_refund},
    withdraw::withdraw,
};

/* -------------------------------------------------------------------------- */
/*                    Refund Flagging + Withdrawal Test                       */
/* -------------------------------------------------------------------------- */

/**
 * Tests the complete refund and withdrawal workflow.
 *
 * Creates a new auction with an active bid, advances blockchain
 * time until the auction can be ended, flags the escrow for refund
 * as the administrator, then verifies that the buyer can withdraw
 * the refunded funds.
 *
 * # Test Flow
 *
 * 1. Load test environment variables.
 * 2. Connect to the configured blockchain provider.
 * 3. Create a test auction with an active bid.
 * 4. Display buyer balance before refund.
 * 5. Advance time until auction expiration.
 * 6. End the auction.
 * 7. Flag the escrow for refund.
 * 8. Execute buyer withdrawal.
 * 9. Verify refund and withdrawal results.
 * 10. Display buyer balance after withdrawal.
 *
 * # Assertions
 *
 * Verifies that:
 *     - Refund flag transaction succeeds
 *     - Refund is marked as flagged
 *     - Withdrawal transaction succeeds
 *     - Withdrawn amount is greater than zero
 *     - Transaction hashes are returned
 */
#[tokio::test]
async fn test_flag_refund() {
    // Load required environment variables for testing.
    load_test_env();

    println!("\n================ FLAG REFUND + WITHDRAW TEST START ================");

    // Connect to the configured blockchain provider.
    let provider = connect_test_provider()
        .await
        .expect("failed to connect provider");

    // Load test addresses used throughout the refund workflow.
    let factory_address = get_factory_test_address();
    let seller_address = get_seller_test_address();
    let buyer_address = get_bidder_address();
    let admin_address = get_admin_test_address();

    // Create a fresh auction containing a valid bid so refund
    // behavior can be tested.
    let created = create_test_auction_with_bid(
        &provider,
        factory_address,
        seller_address,
        buyer_address,
        "Flag Refund Test Auction",
    )
    .await;

    let auction_address = created.auction_address;

    println!("Auction: {:?}", auction_address);
    println!("Buyer:   {:?}", buyer_address);
    println!("Admin:   {:?}", admin_address);

    // Display buyer balance before any refund activity.
    println!("\n=== BALANCES BEFORE ===");
    print_balance(&provider, "Buyer", buyer_address).await;

    println!("\nFast-forwarding + ending auction...");

    // Advance blockchain time until the auction expires.
    hardhat_advance_time(&provider, DEFAULT_BIDDING_TIME_SECONDS)
        .await
        .expect("time advance failed");

    // End the completed auction.
    let _ = end_auction(&provider, auction_address, buyer_address)
        .await
        .expect("end_auction failed");

    // Flag the escrow for refund using the administrator account.
    let flag_result = flag_refund(&provider, auction_address, admin_address)
        .await
        .expect("flag_refund failed");

    // Verify the refund flag transaction returned a valid hash.
    assert!(
        !flag_result.tx_hash.is_empty(),
        "flag refund transaction hash should not be empty"
    );

    // Verify the refund flag was successfully applied.
    assert!(flag_result.refund_flagged, "refund should be flagged");

    // Withdraw the refunded funds as the buyer.
    let withdraw_result = withdraw(&provider, auction_address, buyer_address)
        .await
        .expect("withdraw failed");

    // Verify the withdrawal transaction returned a valid hash.
    assert!(
        !withdraw_result.tx_hash.is_empty(),
        "withdraw transaction hash should not be empty"
    );

    // Verify funds were actually withdrawn.
    assert!(
        withdraw_result.amount_withdrawn_wei > alloy::primitives::U256::ZERO,
        "withdrawn amount should be greater than zero"
    );

    println!("Flag result: {:#?}", flag_result);
    println!("Withdraw result: {:#?}", withdraw_result);

    // Display buyer balance after withdrawal.
    println!("\n=== BALANCES AFTER WITHDRAW ===");
    print_balance(&provider, "Buyer", buyer_address).await;

    println!("\n================ FLAG REFUND + WITHDRAW TEST PASSED ================\n");
}