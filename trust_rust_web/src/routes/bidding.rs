use axum::{extract::State, Json};
use serde::Deserialize;
use std::sync::Arc;
use tower_cookies::Cookies;
use alloy::primitives::{Address, U256};
use trust_rust_client::bidding;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct PlaceBidRequest {
    pub auction_address: String,
    pub bid_amount_wei: String,
}

pub async fn place_bid_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<PlaceBidRequest>,
) -> Result<Json<serde_json::Value>, String> {
    // Get bidder from SIWE session
    let session_cookie = cookies.get("trust_session")
        .ok_or("Not authenticated – please sign in first".to_string())?;

    let session_id = session_cookie.value();
    let bidder_str = {
        let sessions = state.sessions.lock().unwrap();
        sessions.get(session_id)
            .ok_or("Invalid or expired session".to_string())?
            .clone()
    };

    let bidder: Address = bidder_str.parse()
        .map_err(|_| "Invalid bidder address in session".to_string())?;

    let auction_address: Address = req.auction_address.parse()
        .map_err(|_| "Invalid auction address".to_string())?;

    let bid_amount = U256::from_str_radix(&req.bid_amount_wei, 10)
        .map_err(|_| "Invalid bid amount format".to_string())?;

    let result = bidding::place_bid(
        &*state.rpc_provider,
        auction_address,
        bidder,
        bid_amount,
    ).await.map_err(|e| format!("Bid failed: {}", e))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": result.tx_hash,
        "new_highest_bid_wei": result.highest_bid_wei.to_string()
    })))
}