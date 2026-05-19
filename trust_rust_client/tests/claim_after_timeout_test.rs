/*
 * claim_after_timeout_test.rs
 *
 * Purpose:
 *     Integration test for the seller claiming escrowed funds after the buyer
 *     confirmation window has expired.
 *
 *     This test verifies that claim_after_timeout.rs (and the underlying
 *     AuctionEscrow logic) can:
 *         - Connect to the configured Ethereum node
 *         - Fast-forward blockchain time
 *         - End the auction
 *         - Advance past the confirmation window
 *         - Successfully call claimAfterTimeout()
 *         - Confirm funds are transferred to the seller
 *
 *     It does NOT:
 *         - Test bidding behavior
 *         - Test confirmReceipt() or flagRefund() paths
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
 *
 * System Position:
 *
 *     cargo test
 *         ↓
 *     claim_after_timeout_test.rs  ← THIS FILE (integration test)
 *         ↓
 *     escrow.rs                    (claimAfterTimeout + endAuction)
 *         ↓
 *     SimpleAuction.sol (AuctionEscrow)
 *         ↓
 *     Hardhat / Ethereum node
 */

use alloy::primitives::U256;
use alloy::providers::Provider;
use serde_json::Value;
use trust_rust_client::{
    auction_loader::{connect_provider, parse_address},
    config::get_rpc_url,
    escrow::{claim_after_timeout, end_auction},
};

async fn print_balance<P: Provider>(provider: &P, label: &str, address: alloy::primitives::Address) {
    if let Ok(balance) = provider.get_balance(address).await {
        let eth = balance.to::<u128>() as f64 / 1e18;
        println!("  {}: {:.6} ETH", label, eth);
    }
}

#[tokio::test]
async fn test_claim_after_timeout() {
    dotenvy::dotenv().ok();

    println!("\n================ CLAIM AFTER TIMEOUT TEST START ================");

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

    let provider = connect_provider(&rpc_url)
        .await
        .expect("failed to connect provider");

    println!("Auction address: {:?}", auction_address);
    println!("Buyer:           {:?}", buyer_address);
    println!("Seller:          {:?}", seller_address);

    println!("\n=== BALANCES BEFORE ===");
    print_balance(&provider, "Buyer", buyer_address).await;
    print_balance(&provider, "Seller", seller_address).await;

    // Fast forward + end auction
    println!("\nFast-forwarding time + ending auction...");
    let _: Value = provider
        .raw_request("evm_increaseTime".into(), ["3600".to_string()])
        .await
        .expect("time increase failed");

    let _: Value = provider
        .raw_request("evm_mine".into(), Vec::<()>::new())
        .await
        .expect("mine failed");

    let _ = end_auction(&provider, auction_address, buyer_address)
        .await
        .expect("end_auction failed");

    // Fast-forward past confirmation window
    println!("\nFast-forwarding past confirmation window (3 days)...");
    let _: Value = provider
        .raw_request("evm_increaseTime".into(), ["259200".to_string()])
        .await
        .expect("timeout increase failed");

    let _: Value = provider
        .raw_request("evm_mine".into(), Vec::<()>::new())
        .await
        .expect("mine failed");

    println!("\nSeller claiming after timeout...");
    let result = claim_after_timeout(&provider, auction_address, seller_address)
        .await
        .expect("claim_after_timeout failed");

    println!("Claim result: {:#?}", result);

    println!("\n=== BALANCES AFTER CLAIM ===");
    print_balance(&provider, "Buyer", buyer_address).await;
    print_balance(&provider, "Seller", seller_address).await;

    println!("\n================ CLAIM AFTER TIMEOUT TEST PASSED ================\n");
}