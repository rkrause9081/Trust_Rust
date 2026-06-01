/*
 * routes/mod.rs
 *
 * Route module registry.
 *
 * Naming rule:
 * - auction_query.rs    -> read-only auction registry/list calls
 * - auction_create.rs   -> create auction transaction
 * - auction_bid.rs      -> place bid transaction
 * - auction_withdraw.rs -> withdraw pending return transaction
 * - escrow_routes.rs    -> escrow lifecycle routes
 */

pub mod auction_query;
pub mod auction_create;
pub mod auction_bid;
pub mod auction_withdraw;
pub mod escrow_routes;
pub mod session;