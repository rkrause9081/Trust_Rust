/*
 * routes/mod.rs
 *
 * Purpose:
 *     Central route module registry for the application.
 *
 *     Exposes all route handler modules through a single
 *     interface, allowing the web server to import and
 *     register API endpoints from one location.
 *
 * Responsibilities:
 *     - Organize route modules
 *     - Export route handlers
 *     - Provide a central routing namespace
 *     - Maintain route structure consistency
 *
 * Non-Responsibilities:
 *     - Route implementation
 *     - Business logic
 *     - Request validation
 *     - Blockchain interactions
 *
 * Architecture:
 *
 *          routes/mod.rs
 *                 ↓
 *     ┌───────────┼───────────┐
 *     ↓           ↓           ↓
 *  Auction     Escrow     Session
 *   Routes      Routes      Routes
 *
 * Naming Rules:
 *     - auction_query.rs
 *         Read-only auction registry/list calls
 *
 *     - auction_create.rs
 *         Auction creation transactions
 *
 *     - auction_bid.rs
 *         Bid placement transactions
 *
 *     - auction_withdraw.rs
 *         Withdrawal transactions
 *
 *     - escrow_routes.rs
 *         Escrow lifecycle operations
 *
 *     - session.rs
 *         Session and authentication helpers
 */

/* -------------------------------------------------------------------------- */
/*                              Auction Routes                                */
/* -------------------------------------------------------------------------- */

// Read-only auction registry and listing endpoints.
pub mod auction_query;

// Auction creation endpoints.
pub mod auction_create;

// Auction bid submission endpoints.
pub mod auction_bid;

// Withdrawal endpoints for refundable balances.
pub mod auction_withdraw;

/* -------------------------------------------------------------------------- */
/*                               Escrow Routes                                */
/* -------------------------------------------------------------------------- */

// Escrow lifecycle management endpoints.
pub mod escrow_routes;

/* -------------------------------------------------------------------------- */
/*                              Session Support                               */
/* -------------------------------------------------------------------------- */

// Session management and wallet authentication helpers.
pub mod session;