/*
 * config.rs
 *
 * Purpose:
 *     Environment variable loading and configuration utilities.
 *
 *     This module is responsible for loading runtime configuration
 *     values from the environment or a .env file.
 *
 *     It does NOT:
 *         - Handle HTTP requests
 *         - Manage provider connections
 *         - Send transactions
 *         - Parse Ethereum addresses into Alloy Address types
 *
 *     It is called by:
 *         main.rs
 *         integration tests
 *         future Axum handlers / app state builders
 */

use alloy::primitives::U256;
use eyre::{eyre, Result};

fn load_dotenv() {
    dotenvy::dotenv().ok();
}

fn required_var(key: &str) -> Result<String> {
    load_dotenv();
    std::env::var(key).map_err(|_| {
        eyre!("Missing required environment variable: {}", key)
    })
}

fn optional_u64_var(key: &str, default_value: u64) -> Result<u64> {
    load_dotenv();
    match std::env::var(key) {
        Ok(raw) => raw.parse::<u64>().map_err(|e| {
            eyre!("Invalid {} value '{}': {}", key, raw, e)
        }),
        Err(_) => Ok(default_value),
    }
}

pub fn get_rpc_url() -> Result<String> {
    required_var("RPC_URL")
}

pub fn get_factory_address() -> Result<String> {
    required_var("FACTORY_ADDRESS")
}

pub fn get_seller_address() -> Result<String> {
    required_var("SELLER_ADDRESS")
}

pub fn get_bidder_address() -> Result<String> {
    required_var("BIDDER_ADDRESS")
}

pub fn get_auction_address() -> Result<String> {
    required_var("AUCTION_ADDRESS")
}

pub fn get_bidding_time_seconds() -> Result<u64> {
    optional_u64_var("BIDDING_TIME_SECONDS", 3_600u64)
}

pub fn get_starting_bid_wei() -> Result<U256> {
    load_dotenv();
    let raw = std::env::var("STARTING_BID_WEI")
        .unwrap_or_else(|_| "1000000000000000000".to_string());
    U256::from_str_radix(&raw, 10).map_err(|e| {
        eyre!("Invalid STARTING_BID_WEI value '{}': {}", raw, e)
    })
}

pub fn get_confirmation_window() -> Result<u64> {
    optional_u64_var("CONFIRMATION_WINDOW", 259_200u64)
}

/// Load the admin wallet address from the environment.
///
/// Expected env var:
///     ADMIN_ADDRESS
///
/// Used for admin-only functions like flagRefund().
pub fn get_admin_address() -> Result<String> {
    required_var("ADMIN_ADDRESS")
}

pub fn get_registry_page_limit() -> Result<u64> {
    optional_u64_var("REGISTRY_PAGE_LIMIT", 20u64)
}