/*
 * lib.rs
 *
 * Purpose:
 *     Crate root — declares and exports all public modules.
 *
 *     This file defines the public surface of the trust_rust_client
 *     library crate. Any module listed here with `pub mod` becomes
 *     accessible to main.rs, integration tests, Axum handlers, etc.
 */

/// Environment variable loading
pub mod config;

/// Alloy provider connection and address parsing
pub mod auction_loader;

/// On-chain bid placement and highest bid retrieval
pub mod bidding;

/// On-chain auction creation through AuctionFactory
pub mod create_auction;

/// On-chain registry read/query calls
pub mod registry;

/// Escrow operations (confirmReceipt, claimAfterTimeout, flagRefund, endAuction, etc.)
pub mod escrow;

/// Withdrawal of pending returns (used after flagRefund or outbids)
pub mod withdraw;

// === Public re-exports for easy use from trust_rust_web ===
pub use create_auction::{
    create_auction,
    create_auction_with_default_confirmation,
    CreateAuctionResult,
};