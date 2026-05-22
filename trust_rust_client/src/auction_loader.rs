/*
 * auction_loader.rs
 *
 * Purpose:
 *     Provides Alloy provider initialization and Ethereum
 *     address parsing utilities.
 *
 * Responsibilities:
 *     - Establish RPC provider connections
 *     - Parse and validate Ethereum addresses
 *
 * Non-Responsibilities:
 *     - HTTP request handling
 *     - Transaction execution
 *     - Application orchestration
 *
 * Architecture:
 *
 *     main.rs
 *         ↓
 *     bidding.rs
 *         ↓
 *     auction_loader.rs
 *         ↓
 *     Ethereum RPC node
 */

use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
};
use eyre::Result;

/* -------------------------------------------------------------------------- */
/*                              Provider Helpers                              */
/* -------------------------------------------------------------------------- */

/**
 * Establishes an async connection to an Alloy provider.
 *
 * # Arguments
 *
 * * `provider_url` - JSON-RPC endpoint URL
 *   (example: "http://127.0.0.1:8545").
 *
 * # Returns
 *
 * Active Alloy provider instance ready for RPC calls.
 *
 * # Errors
 *
 * Returns an error if the RPC connection fails.
 */
    pub async fn connect_provider(
    provider_url: &str,
    ) -> Result<impl Provider + use<>> {
    let provider = ProviderBuilder::new()
        .connect(provider_url)
        .await?;

    Ok(provider)
    }

/* -------------------------------------------------------------------------- */
/*                              Address Helpers                               */
/* -------------------------------------------------------------------------- */

/**
 * Parses a hex string into a strongly-typed Ethereum address.
 *
 * This validates the address format at the application boundary,
 * preventing malformed addresses from propagating deeper into
 * contract interaction logic.
 *
 * # Arguments
 *
 * * `address` - Ethereum address as a hex string
 *   (example: "0xAbCd...1234").
 *
 * # Returns
 *
 * Parsed Alloy `Address` type.
 *
 * # Errors
 *
 * Returns an error if the address string is invalid or malformed.
 */
pub fn parse_address(address: &str) -> Result<Address> {
    Ok(address.parse()?)
}
