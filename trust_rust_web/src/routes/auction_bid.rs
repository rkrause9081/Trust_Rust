/*
 * auction_bid.rs
 *
 * POST /api/bid
 */

use std::sync::Arc;

use alloy::primitives::{Address, U256};
use axum::{extract::State, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use trust_rust_client::bidding;

use crate::{routes::session::get_session_wallet, state::AppState};

#[derive(Debug, Deserialize)]
pub struct PlaceBidRequest {
    pub auction_address: String,
    pub bid_amount_wei: String,
}

pub async fn place_bid_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<PlaceBidRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let bidder = get_session_wallet(&state, &cookies)?;

    let auction_address: Address = req
        .auction_address
        .trim()
        .parse()
        .map_err(|_| "Invalid auction address".to_string())?;

    let bid_amount = U256::from_str_radix(req.bid_amount_wei.trim(), 10)
        .map_err(|_| "Invalid bid amount format".to_string())?;

    let result = bidding::place_bid(
        state.rpc_provider.as_ref(),
        auction_address,
        bidder,
        bid_amount,
    )
    .await
    .map_err(|e| format!("Bid failed: {e}"))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": result.tx_hash,
        "new_highest_bid_wei": result.highest_bid_wei.to_string()
    })))
}
