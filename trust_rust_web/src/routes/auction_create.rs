/*
 * auction_create.rs
 *
 * POST /api/create-auction
 */

use std::sync::Arc;

use alloy::primitives::U256;
use axum::{extract::State, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use trust_rust_client::create_auction_with_default_confirmation;

use crate::{routes::session::get_session_wallet, state::AppState};

#[derive(Debug, Deserialize)]
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
    validate_create_request(&req)?;

    let seller = get_session_wallet(&state, &cookies)?;

    let starting_bid = U256::from_str_radix(req.starting_bid_wei.trim(), 10)
        .map_err(|_| "Invalid starting bid amount".to_string())?;

    let result = create_auction_with_default_confirmation(
        state.rpc_provider.as_ref(),
        state.factory_address,
        seller,
        U256::from(req.bidding_time_seconds),
        starting_bid,
        req.title,
        req.description,
    )
    .await
    .map_err(|e| format!("Failed to create auction: {e}"))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "auction_address": result.auction_address.to_string(),
        "tx_hash": result.tx_hash,
        "seller": result.seller.to_string()
    })))
}

fn validate_create_request(req: &CreateAuctionRequest) -> Result<(), String> {
    if req.bidding_time_seconds == 0 {
        return Err("bidding_time_seconds must be greater than 0".to_string());
    }

    if req.starting_bid_wei.trim().is_empty() {
        return Err("starting_bid_wei is required".to_string());
    }

    if req.title.trim().is_empty() {
        return Err("title is required".to_string());
    }

    if req.description.trim().is_empty() {
        return Err("description is required".to_string());
    }

    Ok(())
}
