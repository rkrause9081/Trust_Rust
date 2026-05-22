/*
 * mod.rs
 *
 * Purpose:
 *     Route module registry for the Trust Rust web server.
 *
 * Responsibilities:
 *     - Declare route submodules
 *     - Organize HTTP handler namespaces
 *     - Provide centralized route module access
 *
 * Non-Responsibilities:
 *     - HTTP routing configuration
 *     - Request handling logic
 *     - Authentication
 *     - Blockchain interaction
 *
 * Architecture:
 *
 *            main.rs
 *                ↓
 *            routes/
 *                ↓
 *      Route Handler Modules
 */

/* -------------------------------------------------------------------------- */
/*                              Route Modules                                 */
/* -------------------------------------------------------------------------- */

/// Auction creation route handlers.
pub mod auction;

/// Auction registry and listing route handlers.
pub mod auction_list;

/// Bid placement route handlers.
pub mod bidding;

/// Withdrawal route handlers.
pub mod withdraw;

/// Escrow lifecycle and settlement route handlers.
pub mod escrow;
