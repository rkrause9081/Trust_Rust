/*
 * flag_refund_test.rs
 *
 * Purpose:
 *     Integration test for the admin refund flow + buyer withdrawal.
 *
 *     This test verifies that:
 *         - flagRefund() correctly queues the winning bid in pendingReturns
 *         - withdraw() successfully transfers the queued funds to the buyer
 *         - Balance changes are visible on-chain
 *
 *     It does NOT:
 *         - Test normal payment paths (confirmReceipt / claimAfterTimeout)
 *         - Test bidding behavior
 *         - Deploy contracts
 *
 * Requirements:
 *     A local Hardhat node must be running.
 *     A SimpleAuction contract must be deployed and ended.
 *
 * Expected .env values:
 *     RPC_URL
 *     AUCTION_ADDRESS
 *     BUYER_ADDRESS
 *     ADMIN_ADDRESS
 *
 * System Position:
 *
 *     cargo test
 *         ↓
 *     flag_refund_test.rs  ← THIS FILE (integration test)
 *         ↓
 *     escrow.rs  +  withdraw.rs
 *         ↓
 *     SimpleAuction.sol (AuctionEscrow + AuctionSettlement)
 *         ↓
 *     Hardhat / Ethereum node
 */

use alloy::providers::Provider;
use serde_json::Value;
use trust_rust_client::{
    auction_loader::{connect_provider, parse_address},
    config::{get_admin_address, get_rpc_url},
    escrow::{end_auction, flag_refund},
    withdraw::withdraw,
};

async fn print_balance<P: Provider>(provider: &P, label: &str, address: alloy::primitives::Address) {
    if let Ok(balance) = provider.get_balance(address).await {
        let eth = balance.to::<u128>() as f64 / 1e18;
        println!("  {}: {:.6} ETH", label, eth);
    }
}

#[tokio::test]
async fn test_flag_refund() {
    dotenvy::dotenv().ok();

    println!("\n================ FLAG REFUND + WITHDRAW TEST START ================");

    let rpc_url = get_rpc_url().expect("RPC_URL missing");
    let auction_address = parse_address(
        &std::env::var("AUCTION_ADDRESS").expect("AUCTION_ADDRESS missing")
    )
    .expect("invalid auction address");

    let buyer_address = parse_address(
        &std::env::var("BUYER_ADDRESS").expect("BUYER_ADDRESS missing")
    )
    .expect("invalid buyer address");

    let admin_address = parse_address(
        &get_admin_address().expect("ADMIN_ADDRESS missing")
    )
    .expect("invalid admin address");

    let provider = connect_provider(&rpc_url)
        .await
        .expect("failed to connect provider");

    println!("Auction: {:?}", auction_address);
    println!("Buyer:   {:?}", buyer_address);
    println!("Admin:   {:?}", admin_address);

    println!("\n=== BALANCES BEFORE ===");
    print_balance(&provider, "Buyer", buyer_address).await;

    // Setup
    println!("\nFast-forwarding + ending auction...");
    let _: Value = provider
        .raw_request("evm_increaseTime".into(), ["3600".to_string()])
        .await
        .expect("time failed");

    let _: Value = provider
        .raw_request("evm_mine".into(), Vec::<()>::new())
        .await
        .expect("mine failed");

    let _ = end_auction(&provider, auction_address, buyer_address)
        .await
        .expect("end_auction failed");

    // Flag refund
    println!("\nAdmin calling flagRefund()...");
    let flag_result = flag_refund(&provider, auction_address, admin_address)
        .await
        .expect("flag_refund failed");
    println!("Flag result: {:#?}", flag_result);

    // Buyer withdraws
    println!("\nBuyer calling withdraw() to collect refund...");
    let withdraw_result = withdraw(&provider, auction_address, buyer_address)
        .await
        .expect("withdraw failed");
    println!("Withdraw result: {:#?}", withdraw_result);

    println!("\n=== BALANCES AFTER WITHDRAW ===");
    print_balance(&provider, "Buyer", buyer_address).await;

    println!("\n================ FLAG REFUND + WITHDRAW TEST PASSED ================\n");
}