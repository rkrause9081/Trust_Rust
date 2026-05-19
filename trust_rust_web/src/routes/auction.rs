use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_cookies::Cookies;
use trust_rust_client::create_auction_with_default_confirmation;
use alloy::primitives::{Address, U256};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateAuctionRequest {
    pub bidding_time_seconds: u64,
    pub starting_bid_wei: String,
}

#[derive(Serialize)]
pub struct CreateAuctionResponse {
    pub success: bool,
    pub tx_hash: String,
    pub auction_address: String,
    pub seller: String,
    pub message: String,
}

pub async fn create_auction_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<CreateAuctionRequest>,
) -> Result<Json<CreateAuctionResponse>, String> {
    // Get seller from SIWE session cookie
    let session_cookie = cookies.get("trust_session")
        .ok_or("Not authenticated – please sign in first")?;

    let session_id = session_cookie.value();

    let seller_str = {
        let sessions = state.sessions.lock().unwrap();
        sessions.get(session_id)
            .ok_or("Invalid or expired session")?
            .clone()
    };

    let seller: Address = seller_str.parse()
        .map_err(|_| "Invalid seller address in session")?;

    let starting_bid = U256::from_str_radix(&req.starting_bid_wei, 10)
        .map_err(|_| "Invalid starting bid amount")?;

    // Call the client library (important: deref the Arc)
    match create_auction_with_default_confirmation(
        &*state.rpc_provider,
        state.factory_address,
        seller,
        U256::from(req.bidding_time_seconds),
        starting_bid,
    ).await {
        Ok(result) => Ok(Json(CreateAuctionResponse {
            success: true,
            tx_hash: result.tx_hash,
            auction_address: result.auction_address.to_string(),
            seller: result.seller.to_string(),
            message: "Auction created successfully!".to_string(),
        })),
        Err(e) => Err(format!("Failed to create auction: {}", e)),
    }
}