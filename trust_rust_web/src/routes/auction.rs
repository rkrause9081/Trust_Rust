use axum::{extract::State, Json};
use serde::Deserialize;
use std::sync::Arc;
use tower_cookies::Cookies;

use alloy::primitives::{Address, U256};

use trust_rust_client::create_auction_with_default_confirmation;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateAuctionRequest {
    pub bidding_time_seconds: u64,
    pub starting_bid_wei: String,

    pub title: String,
    pub description: String,
}

pub async fn create_auction_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<CreateAuctionRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let session_cookie = cookies
        .get("trust_session")
        .ok_or("Not authenticated – please sign in first".to_string())?;

    let session_id = session_cookie.value();

    let seller_str = {
        let sessions = state.sessions.lock().unwrap();

        sessions
            .get(session_id)
            .ok_or("Invalid or expired session".to_string())?
            .clone()
    };

    let seller: Address = seller_str
        .parse()
        .map_err(|_| "Invalid seller address".to_string())?;

    let starting_bid = U256::from_str_radix(
        &req.starting_bid_wei,
        10,
    )
    .map_err(|_| "Invalid starting bid amount".to_string())?;

    let result = create_auction_with_default_confirmation(
        &*state.rpc_provider,
        state.factory_address,
        seller,
        U256::from(req.bidding_time_seconds),
        starting_bid,
        req.title,
        req.description,
    )
    .await
    .map_err(|e| format!("Failed to create auction: {}", e))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "auction_address": result.auction_address.to_string(),
        "tx_hash": result.tx_hash,
        "seller": result.seller.to_string()
    })))
}