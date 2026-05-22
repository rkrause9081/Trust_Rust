/*
 * config.rs
 *
 * Purpose:
 *     Provides environment variable loading and runtime
 *     configuration utilities.
 *
 * Responsibilities:
 *     - Load environment variables from .env
 *     - Validate required configuration values
 *     - Parse numeric configuration types
 *     - Provide centralized application config access
 *
 * Non-Responsibilities:
 *     - HTTP request handling
 *     - RPC provider initialization
 *     - Transaction execution
 *     - Ethereum address parsing
 *
 * Architecture:
 *
 *     .env / environment variables
 *                 ↓
 *             config.rs
 *                 ↓
 *             main.rs
 *                 ↓
 *         application modules
 */

use alloy::primitives::U256;
use eyre::{Result, eyre};

/* -------------------------------------------------------------------------- */
/*                            Internal Config Helpers                         */
/* -------------------------------------------------------------------------- */

/**
 * Loads environment variables from a `.env` file if present.
 *
 * Safe to call multiple times — dotenvy silently ignores
 * repeated loads after initialization.
 */
fn load_dotenv() {
    dotenvy::dotenv().ok();
}

/**
 * Retrieves a required environment variable.
 *
 * # Arguments
 *
 * * `key` - Environment variable name.
 *
 * # Returns
 *
 * The environment variable value as a `String`.
 *
 * # Errors
 *
 * Returns an error if the variable is missing.
 */
fn required_var(key: &str) -> Result<String> {
    load_dotenv();

    std::env::var(key).map_err(|_| eyre!("Missing required environment variable: {}", key))
}

/**
 * Retrieves an optional `u64` environment variable.
 *
 * Falls back to the provided default value if the variable
 * does not exist.
 *
 * # Arguments
 *
 * * `key` - Environment variable name.
 * * `default_value` - Fallback value if the variable is unset.
 *
 * # Returns
 *
 * Parsed `u64` configuration value.
 *
 * # Errors
 *
 * Returns an error if the variable exists but cannot be parsed.
 */
fn optional_u64_var(key: &str, default_value: u64) -> Result<u64> {
    load_dotenv();

    match std::env::var(key) {
        Ok(raw) => raw
            .parse::<u64>()
            .map_err(|e| eyre!("Invalid {} value '{}': {}", key, raw, e)),

        Err(_) => Ok(default_value),
    }
}

/* -------------------------------------------------------------------------- */
/*                          Environment Configuration                         */
/* -------------------------------------------------------------------------- */

/**
 * Retrieves the Ethereum RPC URL.
 *
 * Expected environment variable:
 *     RPC_URL
 *
 * # Returns
 *
 * JSON-RPC endpoint URL.
 *
 * # Errors
 *
 * Returns an error if the variable is missing.
 */
pub fn get_rpc_url() -> Result<String> {
    required_var("RPC_URL")
}

/**
 * Retrieves the factory contract address.
 *
 * Expected environment variable:
 *     FACTORY_ADDRESS
 *
 * # Returns
 *
 * Factory contract address as a string.
 *
 * # Errors
 *
 * Returns an error if the variable is missing.
 */
pub fn get_factory_address() -> Result<String> {
    required_var("FACTORY_ADDRESS")
}

/**
 * Retrieves the seller wallet address.
 *
 * Expected environment variable:
 *     SELLER_ADDRESS
 *
 * # Returns
 *
 * Seller wallet address as a string.
 *
 * # Errors
 *
 * Returns an error if the variable is missing.
 */
pub fn get_seller_address() -> Result<String> {
    required_var("SELLER_ADDRESS")
}

/**
 * Retrieves the bidder wallet address.
 *
 * Expected environment variable:
 *     BIDDER_ADDRESS
 *
 * # Returns
 *
 * Bidder wallet address as a string.
 *
 * # Errors
 *
 * Returns an error if the variable is missing.
 */
pub fn get_bidder_address() -> Result<String> {
    required_var("BIDDER_ADDRESS")
}

/**
 * Retrieves the deployed auction contract address.
 *
 * Expected environment variable:
 *     AUCTION_ADDRESS
 *
 * # Returns
 *
 * Auction contract address as a string.
 *
 * # Errors
 *
 * Returns an error if the variable is missing.
 */
pub fn get_auction_address() -> Result<String> {
    required_var("AUCTION_ADDRESS")
}

/**
 * Retrieves the auction bidding duration in seconds.
 *
 * Expected environment variable:
 *     BIDDING_TIME_SECONDS
 *
 * Default:
 *     3600 seconds (1 hour)
 *
 * # Returns
 *
 * Auction bidding duration in seconds.
 *
 * # Errors
 *
 * Returns an error if parsing fails.
 */
pub fn get_bidding_time_seconds() -> Result<u64> {
    optional_u64_var("BIDDING_TIME_SECONDS", 3_600u64)
}

/**
 * Retrieves the auction starting bid amount in wei.
 *
 * Expected environment variable:
 *     STARTING_BID_WEI
 *
 * Default:
 *     1 ETH in wei
 *
 * # Returns
 *
 * Starting bid amount as `U256`.
 *
 * # Errors
 *
 * Returns an error if the value cannot be parsed.
 */
pub fn get_starting_bid_wei() -> Result<U256> {
    load_dotenv();

    let raw =
        std::env::var("STARTING_BID_WEI").unwrap_or_else(|_| "1000000000000000000".to_string());

    U256::from_str_radix(&raw, 10)
        .map_err(|e| eyre!("Invalid STARTING_BID_WEI value '{}': {}", raw, e))
}

/**
 * Retrieves the auction confirmation window duration.
 *
 * Expected environment variable:
 *     CONFIRMATION_WINDOW
 *
 * Default:
 *     259200 seconds (72 hours)
 *
 * # Returns
 *
 * Confirmation window duration in seconds.
 *
 * # Errors
 *
 * Returns an error if parsing fails.
 */
pub fn get_confirmation_window() -> Result<u64> {
    optional_u64_var("CONFIRMATION_WINDOW", 259_200u64)
}

/**
 * Retrieves the admin wallet address.
 *
 * Expected environment variable:
 *     ADMIN_ADDRESS
 *
 * Used for privileged admin-only functionality.
 *
 * # Returns
 *
 * Admin wallet address as a string.
 *
 * # Errors
 *
 * Returns an error if the variable is missing.
 */
pub fn get_admin_address() -> Result<String> {
    required_var("ADMIN_ADDRESS")
}

/**
 * Retrieves the registry pagination limit.
 *
 * Expected environment variable:
 *     REGISTRY_PAGE_LIMIT
 *
 * Default:
 *     20
 *
 * # Returns
 *
 * Maximum number of registry entries per page.
 *
 * # Errors
 *
 * Returns an error if parsing fails.
 */
pub fn get_registry_page_limit() -> Result<u64> {
    optional_u64_var("REGISTRY_PAGE_LIMIT", 20u64)
}
