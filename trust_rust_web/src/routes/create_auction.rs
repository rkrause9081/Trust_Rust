/*
 * routes/auction.rs
 *
 * Axum route handler that connects the frontend create_auction.js form
 * to the trust_rust_client::create_auction.rs blockchain client helper.
 */

use std::sync::Arc;

use alloy::primitives::{Address, U256};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use crate::state::AppState;

// Adjust this import path if your blockchain client module is named differently.
use trust_rust_client::create_auction::create_auction_with_default_confirmation;

/* -------------------------------------------------------------------------- */
/*                              Request / Response                            */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Deserialize)]
pub struct CreateAuctionRequest {
    pub bidding_time_seconds: u64,
    pub starting_bid_wei: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct CreateAuctionResponse {
    pub tx_hash: String,
    pub auction_address: String,
    pub seller: String,
    pub bidding_time_seconds: String,
    pub end_time: String,
    pub starting_bid_wei: String,
    pub admin: String,
    pub confirmation_window: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

/* -------------------------------------------------------------------------- */
/*                                Route Handler                               */
/* -------------------------------------------------------------------------- */

pub async fn create_auction_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(payload): Json<CreateAuctionRequest>,
) -> Response {
    match create_auction_from_request(state, cookies, payload).await {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err((status, message)) => (
            status,
            Json(ErrorResponse {
                error: message,
            }),
        )
            .into_response(),
    }
}

async fn create_auction_from_request(
    state: Arc<AppState>,
    cookies: Cookies,
    payload: CreateAuctionRequest,
) -> Result<CreateAuctionResponse, (StatusCode, String)> {
    validate_payload(&payload)?;

    let seller = get_seller_from_cookies(&cookies)?;

    let bidding_time_seconds = U256::from(payload.bidding_time_seconds);

    let starting_bid_wei = U256::from_str_radix(payload.starting_bid_wei.trim(), 10)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid starting_bid_wei".to_string()))?;

    /*
     * These field names assume your AppState stores:
     *   pub provider: ...
     *   pub factory_address: Address
     *
     * If your state uses different names, only update the two lines below.
     */
    let result = create_auction_with_default_confirmation(
        &state.provider,
        state.factory_address,
        seller,
        bidding_time_seconds,
        starting_bid_wei,
        payload.title,
        payload.description,
    )
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create auction: {err}"),
        )
    })?;

    Ok(CreateAuctionResponse {
        tx_hash: result.tx_hash,
        auction_address: result.auction_address.to_string(),
        seller: result.seller.to_string(),
        bidding_time_seconds: result.bidding_time_seconds.to_string(),
        end_time: result.end_time.to_string(),
        starting_bid_wei: result.starting_bid_wei.to_string(),
        admin: result.admin.to_string(),
        confirmation_window: result.confirmation_window.to_string(),
    })
}

/* -------------------------------------------------------------------------- */
/*                                  Helpers                                   */
/* -------------------------------------------------------------------------- */

fn validate_payload(payload: &CreateAuctionRequest) -> Result<(), (StatusCode, String)> {
    if payload.bidding_time_seconds == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "bidding_time_seconds must be greater than 0".to_string(),
        ));
    }

    if payload.title.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "title is required".to_string()));
    }

    if payload.description.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "description is required".to_string(),
        ));
    }

    if payload.starting_bid_wei.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "starting_bid_wei is required".to_string(),
        ));
    }

    Ok(())
}

fn get_seller_from_cookies(cookies: &Cookies) -> Result<Address, (StatusCode, String)> {
    // Match these names to whatever verify_siwe writes during login.
    let cookie_names = ["wallet_address", "address", "siwe_address", "user_address"];

    let seller_raw = cookie_names
        .iter()
        .find_map(|name| cookies.get(name).map(|cookie| cookie.value().to_string()))
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "No signed-in wallet found. Please sign in first.".to_string(),
            )
        })?;

    seller_raw.parse::<Address>().map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            "Signed-in wallet address is invalid".to_string(),
        )
    })
}
