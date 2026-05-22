/*
 * main.rs
 *
 * Purpose:
 *     Application entry point and top-level orchestration.
 */

use alloy::{primitives::U256, providers::Provider};
use eyre::Result;
use serde_json::value::RawValue;

use trust_rust_client::{
    auction_loader::{connect_provider, parse_address},
    config::{
        get_bidding_time_seconds, get_confirmation_window, get_factory_address, get_rpc_url,
        get_seller_address, get_starting_bid_wei,
    },
    create_auction::create_auction,
    escrow::{confirm_receipt, end_auction},
    registry::{get_auction_registry_item, is_registered_auction},
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n================ TRUST RUST CLIENT START ================");

    let rpc_url = get_rpc_url()?;

    let factory_address = parse_address(&get_factory_address()?)?;
    let seller_address = parse_address(&get_seller_address()?)?;

    let bidding_time_seconds = U256::from(get_bidding_time_seconds()?);
    let starting_bid_wei = get_starting_bid_wei()?;
    let confirmation_window = U256::from(get_confirmation_window()?);

    println!("RPC URL:          {}", rpc_url);
    println!("Factory address:  {:?}", factory_address);
    println!("Seller address:   {:?}", seller_address);

    println!("\nAuction creation config:");
    println!("  bidding_time_seconds: {}", bidding_time_seconds);
    println!("  starting_bid_wei:     {}", starting_bid_wei);
    println!("  confirmation_window:  {}", confirmation_window);

    let provider = connect_provider(&rpc_url).await?;

    println!("\nProvider connected successfully.");

    println!("\nSending createAuction transaction...");

    let created = create_auction(
        &provider,
        factory_address,
        seller_address,
        bidding_time_seconds,
        starting_bid_wei,
        confirmation_window,
        "Demo Auction".to_string(),
        "Auction created from the Trust Rust client demo.".to_string(),
    )
    .await?;

    println!("\n================ AUCTION CREATED ================");
    println!("{:#?}", created);

    let registered =
        is_registered_auction(&provider, factory_address, created.auction_address).await?;

    println!("\nIs registered auction: {}", registered);

    let registry_item =
        get_auction_registry_item(&provider, factory_address, created.auction_address).await?;

    println!("\n================ REGISTRY ITEM ================");
    println!("{:#?}", registry_item);

    println!("\n================ ESCROW DEMO START ================");

    println!("Fast-forwarding time to end auction...");

    let increase_time_params = RawValue::from_string("[3600]".to_string())?;

    let _ = provider
        .raw_request_dyn("evm_increaseTime".into(), &increase_time_params)
        .await?;

    let mine_params = RawValue::from_string("[]".to_string())?;

    let _ = provider
        .raw_request_dyn("evm_mine".into(), &mine_params)
        .await?;

    let end_tx = end_auction(&provider, created.auction_address, seller_address).await?;

    println!("Auction ended. Tx: {}", end_tx);

    println!("\nBuyer confirming receipt...");

    let confirm_result =
        confirm_receipt(&provider, created.auction_address, seller_address).await?;

    println!("Confirm receipt result: {:#?}", confirm_result);

    println!("\n================ TRUST RUST CLIENT DONE ================\n");

    Ok(())
}
