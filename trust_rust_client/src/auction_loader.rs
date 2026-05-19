/*
 * auction_loader.rs
 *
 * Purpose:
 *     Low-level Alloy connection and address parsing utilities.
 *
 *     This module is responsible for establishing a connection to the
 *     Ethereum node and parsing contract addresses into typed values
 *     for use throughout the application.
 *
 *     It does NOT:
 *         - Handle HTTP requests
 *         - Handle user input
 *         - Manage application flow
 *         - Send transactions
 *
 *     It is called by:
 *         main.rs      (loads provider + parses addresses for auction interaction)
 *         bidding.rs   (provider passed in for all on-chain calls)
 *
 * System Position:
 *
 *     main.rs  (entry point / orchestration layer)
 *         ↓
 *     bidding.rs
 *         ↓
 *     auction_loader.rs  ← THIS FILE (Alloy connection + address parser)
 *         ↓
 *     Hardhat / Ethereum node
 */

use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
};
use eyre::Result;

/// Establish an async connection to an Alloy provider (Ethereum node).
///
/// Parameters:
///     provider_url: JSON-RPC endpoint (e.g. "http://127.0.0.1:8545" for local Hardhat).
///                   Passed in from main.rs after loading from the RPC_URL env var.
///
/// Returns:
///     impl Provider: Active Alloy provider instance ready for RPC calls.
///
/// Errors:
///     Returns Err if the connection to the node fails.
///     The ? operator propagates this up to main.rs for handling.
pub async fn connect_provider(provider_url: &str) -> Result<impl Provider> {
    // Connect to the Ethereum node at the given JSON-RPC URL.
    // ProviderBuilder is the entry point for all Alloy provider configuration.
    // .connect() is async — the thread is free while the connection is established.
    let provider = ProviderBuilder::new().connect(provider_url).await?;
    Ok(provider)
}

/// Parse a hex string into a strongly-typed Ethereum Address.
///
/// Validates the address format at the boundary — if the string is not a valid
/// Ethereum address, this fails immediately rather than deep inside a contract call.
///
/// Parameters:
///     address: Ethereum address as a hex string (e.g. "0xAbCd...1234").
///
/// Returns:
///     Address: Alloy typed address, checksum-validated at parse time.
///
/// Errors:
///     Returns Err if the string is not a valid Ethereum address format.
pub fn parse_address(address: &str) -> Result<Address> {
    // .parse() on a &str attempts to convert it into the target type (Address).
    // Returns Err if the hex string is malformed or the wrong length.
    // The ? propagates that error up to the caller.
    Ok(address.parse()?)
}