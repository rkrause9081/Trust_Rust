/*
 * test_escrow_interactive.rs
 *
 * Purpose:
 *     Interactive integration test for post-auction escrow and
 *     withdrawal paths.
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

#[tokio::test]
async fn test_escrow_interactive() {
    load_test_env();

    println!("\n================ ESCROW INTERACTIVE TEST ================");

    let provider = connect_test_provider()
        .await
        .expect("failed to connect provider");

    let factory_address = get_factory_test_address();
    let seller_address = get_seller_test_address();
    let buyer_address = get_bidder_address();
    let admin_address = get_admin_test_address();

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

    println!("\nWhich escrow scenario would you like to test?");
    println!("1) confirmReceipt()          → Buyer confirms, seller gets paid immediately");
    println!("2) claimAfterTimeout()       → Seller claims after confirmation window expires");
    println!("3) flagRefund() + withdraw() → Admin refunds buyer");
    print!("Enter choice (1/2/3): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();

    io::stdin().read_line(&mut choice).unwrap();

    let choice = choice.trim();

    println!("\n=== INITIAL BALANCES ===");
    print_balance(&provider, "Buyer", buyer_address).await;
    print_balance(&provider, "Seller", seller_address).await;

    println!("\nFast-forwarding time + ending auction...");

    hardhat_advance_time(&provider, DEFAULT_BIDDING_TIME_SECONDS)
        .await
        .expect("time advance failed");

    let _ = end_auction(&provider, auction_address, buyer_address)
        .await
        .expect("end_auction failed");

    match choice {
        "1" => {
            println!("\n--- Running: confirmReceipt() ---");

            let res = confirm_receipt(&provider, auction_address, buyer_address)
                .await
                .expect("confirm_receipt failed");

            println!("Result: {:#?}", res);
        }

        "2" => {
            println!("\n--- Fast-forwarding past confirmation window ---");

            hardhat_advance_time(&provider, 259_200u64)
                .await
                .expect("timeout advance failed");

            println!("\n--- Running: claimAfterTimeout() ---");

            let res = claim_after_timeout(&provider, auction_address, seller_address)
                .await
                .expect("claim_after_timeout failed");

            println!("Result: {:#?}", res);
        }

        "3" => {
            println!("\n--- Running: flagRefund() + withdraw() ---");

            let flag_res = flag_refund(&provider, auction_address, admin_address)
                .await
                .expect("flag_refund failed");

            println!("Flag result: {:#?}", flag_res);

            let withdraw_res = withdraw(&provider, auction_address, buyer_address)
                .await
                .expect("withdraw failed");

            println!("Withdraw result: {:#?}", withdraw_res);
        }

        _ => {
            println!("Invalid choice! Running default confirmReceipt().");

            let res = confirm_receipt(&provider, auction_address, buyer_address)
                .await
                .expect("confirm_receipt failed");

            println!("Result: {:#?}", res);
        }
    }

    println!("\n=== FINAL BALANCES ===");
    print_balance(&provider, "Buyer", buyer_address).await;
    print_balance(&provider, "Seller", seller_address).await;

    println!("\n================ ESCROW INTERACTIVE TEST COMPLETED ================\n");
}
