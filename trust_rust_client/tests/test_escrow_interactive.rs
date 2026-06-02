/*
 * test_escrow_interactive.rs
 *
 * Purpose:
 *     Interactive integration test for post-auction escrow and
 *     withdrawal paths.
 *
 *     This test creates a fresh auction with a valid bid, then
 *     allows the user to interactively select which escrow
 *     settlement path to execute.
 *
 * Responsibilities:
 *     - Create a test auction with an active bid
 *     - Present escrow workflow options
 *     - Execute selected escrow scenario
 *     - Display balances before and after settlement
 *     - Demonstrate escrow contract behavior
 *
 * Non-Responsibilities:
 *     - Escrow contract implementation
 *     - Withdrawal implementation
 *     - Provider initialization
 *     - Environment variable management
 *
 * Architecture:
 *
 *     test_escrow_interactive.rs
 *                  ↓
 *          common test utilities
 *                  ↓
 *       trust_rust_client::escrow
 *                  ↓
 *      trust_rust_client::withdraw
 *                  ↓
 *            Auction Contract
 *                  ↓
 *             Escrow Contract
 *
 * Note:
 *     This test creates its own auction and setup bid before
 *     running the selected scenario.
 */

mod common;

use std::io::{self, Write};

use common::{
    DEFAULT_BIDDING_TIME_SECONDS, connect_test_provider, create_test_auction_with_bid,
    get_admin_test_address, get_bidder_address, get_factory_test_address, get_seller_test_address,
    hardhat_advance_time, load_test_env, print_balance,
};
use trust_rust_client::{
    escrow::{claim_after_timeout, confirm_receipt, end_auction, flag_refund},
    withdraw::withdraw,
};

/* -------------------------------------------------------------------------- */
/*                     Interactive Escrow Workflow Test                       */
/* -------------------------------------------------------------------------- */

/**
 * Interactive escrow settlement integration test.
 *
 * Creates a fresh auction with an active bid, advances the auction
 * to completion, then allows the user to choose one of several
 * escrow settlement paths for manual testing.
 *
 * # Available Scenarios
 *
 * 1. confirmReceipt()
 *      Buyer confirms receipt and seller is paid immediately.
 *
 * 2. claimAfterTimeout()
 *      Seller claims escrow after confirmation window expires.
 *
 * 3. flagRefund() + withdraw()
 *      Administrator flags refund and buyer withdraws funds.
 *
 * # Test Flow
 *
 * 1. Load test environment variables.
 * 2. Connect to the configured blockchain provider.
 * 3. Create a test auction with an active bid.
 * 4. Display available escrow scenarios.
 * 5. Read user scenario selection.
 * 6. Display initial balances.
 * 7. Advance time and end auction.
 * 8. Execute selected escrow workflow.
 * 9. Display final balances.
 */
#[tokio::test]
async fn test_escrow_interactive() {
    // Load required environment variables for testing.
    load_test_env();

    println!("\n================ ESCROW INTERACTIVE TEST ================");

    // Connect to the configured blockchain provider.
    let provider = connect_test_provider()
        .await
        .expect("failed to connect provider");

    // Load test addresses used throughout escrow workflows.
    let factory_address = get_factory_test_address();
    let seller_address = get_seller_test_address();
    let buyer_address = get_bidder_address();
    let admin_address = get_admin_test_address();

    // Create a fresh auction with a valid bid so escrow
    // settlement scenarios can be tested.
    let created = create_test_auction_with_bid(
        &provider,
        factory_address,
        seller_address,
        buyer_address,
        "Interactive Escrow Test Auction",
    )
    .await;

    let auction_address = created.auction_address;

    println!("Auction: {:?}", auction_address);
    println!("Buyer:   {:?}", buyer_address);
    println!("Seller:  {:?}", seller_address);
    println!("Admin:   {:?}", admin_address);

    // Present available escrow settlement workflows.
    println!("\nWhich escrow scenario would you like to test?");
    println!("1) confirmReceipt()          → Buyer confirms, seller gets paid immediately");
    println!("2) claimAfterTimeout()       → Seller claims after confirmation window expires");
    println!("3) flagRefund() + withdraw() → Admin refunds buyer");
    print!("Enter choice (1/2/3): ");
    io::stdout().flush().unwrap();

    // Read the selected workflow from standard input.
    let mut choice = String::new();

    io::stdin().read_line(&mut choice).unwrap();

    let choice = choice.trim();

    // Display account balances before escrow settlement.
    println!("\n=== INITIAL BALANCES ===");
    print_balance(&provider, "Buyer", buyer_address).await;
    print_balance(&provider, "Seller", seller_address).await;

    println!("\nFast-forwarding time + ending auction...");

    // Advance blockchain time until the auction expires.
    hardhat_advance_time(&provider, DEFAULT_BIDDING_TIME_SECONDS)
        .await
        .expect("time advance failed");

    // End the auction before executing escrow actions.
    let _ = end_auction(&provider, auction_address, buyer_address)
        .await
        .expect("end_auction failed");

    // Execute the selected escrow settlement workflow.
    match choice {
        "1" => {
            println!("\n--- Running: confirmReceipt() ---");

            // Buyer confirms successful receipt of goods.
            let res = confirm_receipt(&provider, auction_address, buyer_address)
                .await
                .expect("confirm_receipt failed");

            println!("Result: {:#?}", res);
        }

        "2" => {
            println!("\n--- Fast-forwarding past confirmation window ---");

            // Advance time beyond the confirmation period.
            hardhat_advance_time(&provider, 259_200u64)
                .await
                .expect("timeout advance failed");

            println!("\n--- Running: claimAfterTimeout() ---");

            // Seller claims escrow after buyer inactivity.
            let res = claim_after_timeout(&provider, auction_address, seller_address)
                .await
                .expect("claim_after_timeout failed");

            println!("Result: {:#?}", res);
        }

        "3" => {
            println!("\n--- Running: flagRefund() + withdraw() ---");

            // Administrator flags the escrow for refund.
            let flag_res = flag_refund(&provider, auction_address, admin_address)
                .await
                .expect("flag_refund failed");

            println!("Flag result: {:#?}", flag_res);

            // Buyer withdraws refunded escrow funds.
            let withdraw_res = withdraw(&provider, auction_address, buyer_address)
                .await
                .expect("withdraw failed");

            println!("Withdraw result: {:#?}", withdraw_res);
        }

        _ => {
            println!("Invalid choice! Running default confirmReceipt().");

            // Default to buyer confirmation for invalid input.
            let res = confirm_receipt(&provider, auction_address, buyer_address)
                .await
                .expect("confirm_receipt failed");

            println!("Result: {:#?}", res);
        }
    }

    // Display balances after escrow settlement completes.
    println!("\n=== FINAL BALANCES ===");
    print_balance(&provider, "Buyer", buyer_address).await;
    print_balance(&provider, "Seller", seller_address).await;

    println!("\n================ ESCROW INTERACTIVE TEST COMPLETED ================\n");
}