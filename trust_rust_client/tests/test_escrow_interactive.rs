/*
 * test_escrow_interactive.rs
 *
 * Purpose:
 *     Interactive integration test for all post-auction escrow and withdrawal paths.
 *
 *     This test allows the developer to choose at runtime which escrow resolution
 *     scenario to execute and verifies the full lifecycle including:
 *         - Time manipulation on Hardhat
 *         - Ending the auction
 *         - confirmReceipt()   → immediate payment to seller
 *         - claimAfterTimeout() → seller claims after confirmation window
 *         - flagRefund() + withdraw() → admin refunds buyer
 *
 *     It does NOT:
 *         - Test bidding behavior
 *         - Test auction creation
 *         - Test frontend/Axum routes
 *         - Deploy contracts
 *
 * Requirements:
 *     A local Hardhat node must be running.
 *     A SimpleAuction contract must already be deployed with a winning bidder.
 *
 * Expected .env values:
 *     RPC_URL
 *     AUCTION_ADDRESS
 *     BUYER_ADDRESS
 *     SELLER_ADDRESS
 *     ADMIN_ADDRESS
 *
 * System Position:
 *
 *     cargo test
 *         ↓
 *     test_escrow_interactive.rs  ← THIS FILE (interactive test)
 *         ↓
 *     escrow.rs + withdraw.rs
 *         ↓
 *     SimpleAuction.sol (AuctionEscrow + AuctionSettlement)
 *         ↓
 *     Hardhat / Ethereum node
 */

use alloy::providers::Provider;
use serde_json::Value;
use std::io::{self, Write};
use trust_rust_client::{
    auction_loader::{connect_provider, parse_address},
    config::{get_admin_address, get_rpc_url},
    escrow::{claim_after_timeout, confirm_receipt, end_auction, flag_refund},
    withdraw::withdraw,
};

async fn print_balance<P: Provider>(provider: &P, label: &str, address: alloy::primitives::Address) {
    if let Ok(balance) = provider.get_balance(address).await {
        let eth = balance.to::<u128>() as f64 / 1e18;
        println!("  {}: {:.6} ETH", label, eth);
    } else {
        println!("  {}: (balance fetch failed)", label);
    }
}

#[tokio::test]
async fn test_escrow_interactive() {
    dotenvy::dotenv().ok();

    println!("\n================ ESCROW INTERACTIVE TEST ================");

    let rpc_url = get_rpc_url().expect("RPC_URL missing");
    let auction_address = parse_address(
        &std::env::var("AUCTION_ADDRESS").expect("AUCTION_ADDRESS missing")
    )
    .expect("invalid auction address");

    let buyer_address = parse_address(
        &std::env::var("BUYER_ADDRESS").expect("BUYER_ADDRESS missing")
    )
    .expect("invalid buyer address");

    let seller_address = parse_address(
        &std::env::var("SELLER_ADDRESS").expect("SELLER_ADDRESS missing")
    )
    .expect("invalid seller address");

    let admin_address = parse_address(
        &get_admin_address().expect("ADMIN_ADDRESS missing")
    )
    .expect("invalid admin address");

    let provider = connect_provider(&rpc_url)
        .await
        .expect("failed to connect provider");

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

    // Common setup: end the auction
    println!("\nFast-forwarding time + ending auction...");
    let _: Value = provider
        .raw_request("evm_increaseTime".into(), ["3600".to_string()])
        .await
        .expect("evm_increaseTime failed");

    let _: Value = provider
        .raw_request("evm_mine".into(), Vec::<()>::new())
        .await
        .expect("evm_mine failed");

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
            println!("\n--- Fast-forwarding past confirmation window (3 days) ---");
            let _: Value = provider
                .raw_request("evm_increaseTime".into(), ["259200".to_string()])
                .await
                .expect("timeout increase failed");

            let _: Value = provider
                .raw_request("evm_mine".into(), Vec::<()>::new())
                .await
                .expect("evm_mine failed");

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
            println!("Invalid choice! Running default (confirmReceipt).");
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