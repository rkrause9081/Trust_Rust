/*
 * common/mod.rs
 *
 * Purpose:
 *     Shared integration test helpers for Trust Rust client tests.
 *
 * Responsibilities:
 *     - Load test environment configuration
 *     - Create fresh auctions for independent tests
 *     - Place setup bids when escrow tests need a winning bidder
 *     - Manipulate Hardhat blockchain time
 *
 * Non-Responsibilities:
 *     - Production application logic
 *     - Frontend or Axum route testing
 */

use alloy::{
    primitives::{Address, U256},
    providers::Provider,
};
use eyre::Result;
use serde_json::value::RawValue;

use trust_rust_client::{
    auction_loader::{connect_provider, parse_address},
    bidding::place_bid,
    config::{
        get_admin_address, get_confirmation_window, get_factory_address, get_rpc_url,
        get_seller_address,
    },
    create_auction::{CreateAuctionResult, create_auction},
};

/* -------------------------------------------------------------------------- */
/*                              Test Constants                                */
/* -------------------------------------------------------------------------- */

pub const DEFAULT_BIDDING_TIME_SECONDS: u64 = 3_600;
pub const DEFAULT_STARTING_BID_WEI: u128 = 1_000_000_000_000_000_000;
pub const DEFAULT_BID_AMOUNT_WEI: u128 = 2_000_000_000_000_000_000;

/* -------------------------------------------------------------------------- */
/*                              Environment Helpers                           */
/* -------------------------------------------------------------------------- */

/**
 * Loads environment variables from `.env`.
 */
pub fn load_test_env() {
    dotenvy::dotenv().ok();
}

/**
 * Loads the configured bidder address.
 *
 * # Returns
 *
 * Parsed bidder wallet address.
 */
pub fn get_bidder_address() -> Address {
    parse_address(&std::env::var("BIDDER_ADDRESS").expect("BIDDER_ADDRESS missing"))
        .expect("invalid bidder address")
}

/**
 * Loads the configured buyer address.
 *
 * # Returns
 *
 * Parsed buyer wallet address.
 */
pub fn get_buyer_address() -> Address {
    parse_address(&std::env::var("BUYER_ADDRESS").expect("BUYER_ADDRESS missing"))
        .expect("invalid buyer address")
}

/**
 * Loads the configured seller address.
 *
 * # Returns
 *
 * Parsed seller wallet address.
 */
pub fn get_seller_test_address() -> Address {
    parse_address(&get_seller_address().expect("SELLER_ADDRESS missing"))
        .expect("invalid seller address")
}

/**
 * Loads the configured admin address.
 *
 * # Returns
 *
 * Parsed admin wallet address.
 */
pub fn get_admin_test_address() -> Address {
    parse_address(&get_admin_address().expect("ADMIN_ADDRESS missing"))
        .expect("invalid admin address")
}

/**
 * Loads the configured factory address.
 *
 * # Returns
 *
 * Parsed auction factory contract address.
 */
pub fn get_factory_test_address() -> Address {
    parse_address(&get_factory_address().expect("FACTORY_ADDRESS missing"))
        .expect("invalid factory address")
}

/* -------------------------------------------------------------------------- */
/*                              Provider Helpers                              */
/* -------------------------------------------------------------------------- */

/**
 * Connects to the configured RPC provider.
 *
 * # Returns
 *
 * Active Alloy provider instance.
 */
pub async fn connect_test_provider() -> eyre::Result<impl Provider> {
    let rpc_url = get_rpc_url()?;

    connect_provider(&rpc_url).await
}

/* -------------------------------------------------------------------------- */
/*                              Auction Helpers                               */
/* -------------------------------------------------------------------------- */

/**
 * Creates a fresh auction for an integration test.
 *
 * Each test should call this instead of relying on `AUCTION_ADDRESS`.
 * This keeps tests independent and prevents Cargo test ordering issues.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `factory_address` - Auction factory contract address.
 * * `seller_address` - Seller wallet address.
 * * `title` - Test auction title.
 *
 * # Returns
 *
 * Full auction creation result.
 */
pub async fn create_test_auction<P>(
    provider: &P,
    factory_address: Address,
    seller_address: Address,
    title: &str,
) -> CreateAuctionResult
where
    P: Provider + ?Sized,
{
    let confirmation_window =
        U256::from(get_confirmation_window().expect("invalid CONFIRMATION_WINDOW"));

    create_auction(
        provider,
        factory_address,
        seller_address,
        U256::from(DEFAULT_BIDDING_TIME_SECONDS),
        U256::from(DEFAULT_STARTING_BID_WEI),
        confirmation_window,
        title.to_string(),
        "Auction created automatically during integration testing.".to_string(),
    )
    .await
    .expect("create_test_auction failed")
}

/**
 * Creates an auction and places a winning setup bid.
 *
 * This is useful for escrow tests that require an auction with
 * an existing highest bidder.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `factory_address` - Auction factory contract address.
 * * `seller_address` - Seller wallet address.
 * * `bidder_address` - Bidder wallet address.
 * * `title` - Test auction title.
 *
 * # Returns
 *
 * Full auction creation result.
 */
pub async fn create_test_auction_with_bid<P>(
    provider: &P,
    factory_address: Address,
    seller_address: Address,
    bidder_address: Address,
    title: &str,
) -> CreateAuctionResult
where
    P: Provider + ?Sized,
{
    let created = create_test_auction(provider, factory_address, seller_address, title).await;

    place_bid(
        provider,
        created.auction_address,
        bidder_address,
        U256::from(DEFAULT_BID_AMOUNT_WEI),
    )
    .await
    .expect("setup bid failed");

    created
}

/* -------------------------------------------------------------------------- */
/*                              Hardhat Helpers                               */
/* -------------------------------------------------------------------------- */

/**
 * Sends a raw Hardhat JSON-RPC request.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `method` - JSON-RPC method name.
 * * `params_json` - Raw JSON params array.
 */
pub async fn hardhat_raw_request<P>(provider: &P, method: &str, params_json: &str) -> Result<()>
where
    P: Provider + ?Sized,
{
    let params = RawValue::from_string(params_json.to_string())?;

    let method = method.to_string();

    let _ = provider
    .raw_request_dyn(method.into(), &params)
    .await?;

    Ok(())
}

/**
 * Advances Hardhat blockchain time.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `seconds` - Number of seconds to advance.
 */
pub async fn hardhat_increase_time<P>(provider: &P, seconds: u64) -> Result<()>
where
    P: Provider + ?Sized,
{
    hardhat_raw_request(provider, "evm_increaseTime", &format!("[{}]", seconds)).await
}

/**
 * Mines one Hardhat block.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 */
pub async fn hardhat_mine<P>(provider: &P) -> Result<()>
where
    P: Provider + ?Sized,
{
    hardhat_raw_request(provider, "evm_mine", "[]").await
}

/**
 * Advances time and mines a block.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `seconds` - Number of seconds to advance.
 */
pub async fn hardhat_advance_time<P>(provider: &P, seconds: u64) -> Result<()>
where
    P: Provider + ?Sized,
{
    hardhat_increase_time(provider, seconds).await?;

    hardhat_mine(provider).await?;

    Ok(())
}

/* -------------------------------------------------------------------------- */
/*                              Debug Helpers                                 */
/* -------------------------------------------------------------------------- */

/**
 * Prints an account balance in ETH.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `label` - Display label.
 * * `address` - Wallet address to query.
 */
pub async fn print_balance<P>(provider: &P, label: &str, address: Address)
where
    P: Provider + ?Sized,
{
    if let Ok(balance) = provider.get_balance(address).await {
        let eth = balance.to::<u128>() as f64 / 1e18;

        println!("  {}: {:.6} ETH", label, eth);
    } else {
        println!("  {}: (balance fetch failed)", label);
    }
}
