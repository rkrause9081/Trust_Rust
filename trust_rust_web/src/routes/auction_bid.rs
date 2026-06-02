/*
 * auction_bid.rs
 *
 * Purpose:
 *     HTTP route handler for auction bid placement.
 *
 *     Accepts bid requests from authenticated users, validates
 *     request parameters, submits the bid transaction to the
 *     blockchain, and returns transaction results to the client.
 *
 * Responsibilities:
 *     - Parse bid request payloads
 *     - Validate auction addresses
 *     - Validate bid amounts
 *     - Retrieve authenticated bidder wallet
 *     - Submit bid transactions
 *     - Return API responses
 *
 * Non-Responsibilities:
 *     - Bid contract implementation
 *     - Session creation
 *     - Provider initialization
 *     - Auction validation logic
 *
 * Architecture:
 *
 *      Client Request
 *             ↓
 *      auction_bid.rs
 *             ↓
 *      Session Validation
 *             ↓
 *      trust_rust_client::bidding
 *             ↓
 *        Auction Contract
 *
 * Endpoint:
 *     POST /api/bid
 */

use std::sync::Arc;

use alloy::primitives::{Address, U256};
use axum::{extract::State, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use trust_rust_client::bidding;

use crate::{routes::session::get_session_wallet, state::AppState};

/* -------------------------------------------------------------------------- */
/*                             Request Structures                             */
/* -------------------------------------------------------------------------- */

/**
 * Bid placement request payload.
 *
 * Fields are received as strings and converted
 * into strongly-typed blockchain values before
 * submission.
 */
#[derive(Debug, Deserialize)]
pub struct PlaceBidRequest {
    pub auction_address: String,
    pub bid_amount_wei: String,
}

/* -------------------------------------------------------------------------- */
/*                              Route Handlers                                */
/* -------------------------------------------------------------------------- */

/**
 * Handles auction bid placement requests.
 *
 * Validates request parameters, retrieves the authenticated
 * bidder wallet from the current session, submits the bid
 * transaction to the blockchain, and returns transaction
 * details to the client.
 *
 * # Request Body
 *
 * {
 *     "auction_address": "...",
 *     "bid_amount_wei": "..."
 * }
 *
 * # Returns
 *
 * Success response:
 *
 * {
 *     "success": true,
 *     "tx_hash": "...",
 *     "new_highest_bid_wei": "..."
 * }
 *
 * # Errors
 *
 * Returns an error if:
 *     - Session authentication fails
 *     - Auction address is invalid
 *     - Bid amount format is invalid
 *     - Bid transaction fails
 */
pub async fn place_bid_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<PlaceBidRequest>,
) -> Result<Json<serde_json::Value>, String> {
    // Retrieve the authenticated wallet address from the session.
    let bidder = get_session_wallet(&state, &cookies)?;

    // Parse and validate the auction contract address.
    let auction_address: Address = req
        .auction_address
        .trim()
        .parse()
        .map_err(|_| "Invalid auction address".to_string())?;

    // Parse and validate the bid amount in wei.
    let bid_amount = U256::from_str_radix(req.bid_amount_wei.trim(), 10)
        .map_err(|_| "Invalid bid amount format".to_string())?;

    // Submit the bid transaction to the auction contract.
    let result = bidding::place_bid(
        state.rpc_provider.as_ref(),
        auction_address,
        bidder,
        bid_amount,
    )
    .await
    .map_err(|e| format!("Bid failed: {e}"))?;

    // Return transaction results to the client.
    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": result.tx_hash,
        "new_highest_bid_wei": result.highest_bid_wei.to_string()
    })))
}