/*
 * main.rs
 *
 * Purpose:
 *     Application entry point and top-level orchestration.
 *
 *     This module demonstrates the full auction lifecycle:
 *       - Create auction
 *       - Place bids (optional)
 *       - Registry queries
 *       - Escrow operations (end auction, confirmReceipt, flagRefund, withdraw)
 */
use alloy::primitives::U256;
use alloy::providers::Provider;
use eyre::Result;
use trust_rust_client::{
    auction_loader::{connect_provider, parse_address},
    config::{
        get_bidding_time_seconds, get_confirmation_window, get_factory_address,
        get_registry_page_limit, get_rpc_url, get_seller_address, get_starting_bid_wei,
    },
    create_auction::create_auction,
    escrow::{confirm_receipt, end_auction, flag_refund},
    registry::{
        get_auction_count, get_auction_registry_item, get_auctions_by_seller,
        get_auctions_paginated, is_registered_auction,
    },
    withdraw::withdraw,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n================ TRUST RUST CLIENT START ================");

    let rpc_url = get_rpc_url()?;
    let factory_address = parse_address(&get_factory_address()?)?;
    let seller_address = parse_address(&get_seller_address()?)?;

    println!("RPC URL:          {}", rpc_url);
    println!("Factory address:  {:?}", factory_address);
    println!("Seller address:   {:?}", seller_address);

    let bidding_time_seconds = U256::from(get_bidding_time_seconds()?);
    let starting_bid_wei = get_starting_bid_wei()?;
    let confirmation_window = U256::from(get_confirmation_window()?);

    println!("\nAuction creation config:");
    println!("  bidding_time_seconds: {}", bidding_time_seconds);
    println!("  starting_bid_wei:     {}", starting_bid_wei);
    println!("  confirmation_window:  {}", confirmation_window);

    let provider = connect_provider(&rpc_url).await?;
    println!("\nProvider connected successfully.");

    // === Create Auction ===
    println!("\nSending createAuction transaction...");
    let created = create_auction(
        &provider,
        factory_address,
        seller_address,
        bidding_time_seconds,
        starting_bid_wei,
        confirmation_window,
    )
    .await?;

    println!("\n================ AUCTION CREATED ================");
    println!("{:#?}", created);

    // === Registry Checks ===
    let registered = is_registered_auction(&provider, factory_address, created.auction_address).await?;
    println!("\nIs registered auction: {}", registered);

    let registry_item = get_auction_registry_item(&provider, factory_address, created.auction_address).await?;
    println!("\n================ REGISTRY ITEM ================");
    println!("{:#?}", registry_item);

    // === Escrow & Settlement Demo ===
    println!("\n================ ESCROW DEMO START ================");

    // Fast-forward time so auction can end
    println!("Fast-forwarding time to end auction...");
    let _: serde_json::Value = provider
        .raw_request_dyn("evm_increaseTime".into(), ["3600".to_string()])
        .await?;
    let _: serde_json::Value = provider.raw_request_dyn("evm_mine".into(), Vec::<()>::new()).await?;

    let end_tx = end_auction(&provider, created.auction_address, seller_address).await?;
    println!("Auction ended. Tx: {}", end_tx);

    // Buyer confirms receipt → seller gets paid
    println!("\nBuyer confirming receipt...");
    let confirm_result = confirm_receipt(&provider, created.auction_address, seller_address).await?; // Using seller as buyer for demo
    println!("Confirm receipt result: {:#?}", confirm_result);

    // Optional: Admin flag refund + withdraw (uncomment if you want to test)
    /*
    println!("\nAdmin flagging refund...");
    let flag_result = flag_refund(&provider, created.auction_address, seller_address).await?;
    println!("Flag result: {:#?}", flag_result);

    println!("\nBuyer withdrawing refund...");
    let withdraw_result = withdraw(&provider, created.auction_address, seller_address).await?;
    println!("Withdraw result: {:#?}", withdraw_result);
    */

    println!("\n================ TRUST RUST CLIENT DONE ================\n");

    Ok(())
}