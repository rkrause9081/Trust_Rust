/*
 * bidding.rs
 *
 * Purpose:
 *     Provides HTTP handlers for auction bid placement.
 *
 * Responsibilities:
 *     - Validate authenticated bidder sessions
 *     - Parse bid request payloads
 *     - Convert request data into blockchain types
 *     - Submit bid transactions
 *     - Return JSON API responses
 *
 * Non-Responsibilities:
 *     - SIWE authentication verification
 *     - Smart contract interaction internals
 *     - Auction registry queries
 *     - Provider initialization
 *
 * Architecture:
 *
 *      Browser / Frontend
 *              ↓
 *          bidding.rs
 *              ↓
 *           AppState
 *              ↓
 *      trust_rust_client
 *              ↓
 *       Auction Contract
 */

use std::sync::Arc;

use alloy::primitives::{Address, U256};

use axum::{extract::State, Json};

use serde::Deserialize;

use tower_cookies::Cookies;

use trust_rust_client::bidding;

use crate::state::AppState;

/* -------------------------------------------------------------------------- */
/*                               Request Types                                */
/* -------------------------------------------------------------------------- */

/**
 * Request payload used to place an auction bid.
 */
#[derive(Deserialize)]
pub struct PlaceBidRequest {
    pub auction_address: String,
    pub bid_amount_wei: String,
}

/* -------------------------------------------------------------------------- */
/*                              Route Handlers                                */
/* -------------------------------------------------------------------------- */

/**
 * Places a bid on an auction contract using the authenticated user.
 *
 * This handler:
 *     - Validates the active SIWE session
 *     - Retrieves the authenticated bidder wallet
 *     - Parses the auction address
 *     - Parses the bid amount
 *     - Executes the blockchain bid transaction
 *     - Returns bid transaction metadata as JSON
 *
 * # Arguments
 *
 * * `state` - Shared Axum application state.
 * * `cookies` - Cookie jar containing the session cookie.
 * * `req` - Bid request payload.
 *
 * # Returns
 *
 * JSON response containing:
 *     - success status
 *     - transaction hash
 *     - updated highest bid
 *
 * # Errors
 *
 * Returns a string error if:
 *     - The user is not authenticated
 *     - The session is invalid or expired
 *     - The bidder address is invalid
 *     - The auction address is invalid
 *     - The bid amount cannot be parsed
 *     - Bid submission fails
 */
pub async fn place_bid_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<PlaceBidRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let session_cookie = cookies
        .get("trust_session")
        .ok_or("Not authenticated – please sign in first".to_string())?;

    let session_id = session_cookie.value();

    let bidder_str = {
        let sessions = state.sessions.lock().unwrap();

        sessions
            .get(session_id)
            .ok_or("Invalid or expired session".to_string())?
            .clone()
    };

    let bidder: Address = bidder_str
        .parse()
        .map_err(|_| "Invalid bidder address in session".to_string())?;

    let auction_address: Address = req
        .auction_address
        .parse()
        .map_err(|_| "Invalid auction address".to_string())?;

    let bid_amount = U256::from_str_radix(&req.bid_amount_wei, 10)
        .map_err(|_| "Invalid bid amount format".to_string())?;

    let result = bidding::place_bid(&*state.rpc_provider, auction_address, bidder, bid_amount)
        .await
        .map_err(|e| format!("Bid failed: {}", e))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": result.tx_hash,
        "new_highest_bid_wei": result.highest_bid_wei.to_string()
    })))
}
