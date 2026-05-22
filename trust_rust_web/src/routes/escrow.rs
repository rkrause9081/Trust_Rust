/*
 * escrow.rs
 *
 * Purpose:
 *     Provides HTTP handlers for escrow lifecycle and
 *     post-auction settlement operations.
 *
 * Responsibilities:
 *     - Fetch escrow status information
 *     - Validate authenticated user sessions
 *     - Execute escrow settlement actions
 *     - Execute timeout claims
 *     - Execute refund actions
 *     - Return JSON API responses
 *
 * Non-Responsibilities:
 *     - SIWE authentication verification
 *     - Smart contract interaction internals
 *     - Provider initialization
 *     - Persistent storage management
 *
 * Architecture:
 *
 *      Browser / Frontend
 *              ↓
 *          escrow.rs
 *              ↓
 *           AppState
 *              ↓
 *      trust_rust_client
 *              ↓
 *      Auction Escrow Logic
 */

use std::sync::Arc;

use alloy::primitives::Address;

use axum::{
    extract::{Path, State},
    Json,
};

use serde::{Deserialize, Serialize};

use tower_cookies::Cookies;

use trust_rust_client::{
    can_claim_timeout, can_confirm_receipt, can_flag_refund, claim_after_timeout, confirm_receipt,
    end_auction, flag_refund, get_escrow_status, get_time_remaining_for_confirmation, EscrowStatus,
};

use crate::state::AppState;

/* -------------------------------------------------------------------------- */
/*                               Request Types                                */
/* -------------------------------------------------------------------------- */

/**
 * Request payload used for escrow action endpoints.
 */
#[derive(Deserialize)]
pub struct EscrowActionRequest {
    pub auction_address: String,
}

/* -------------------------------------------------------------------------- */
/*                               Response Types                               */
/* -------------------------------------------------------------------------- */

/**
 * Response returned when fetching escrow status information.
 */
#[derive(Serialize)]
pub struct EscrowStatusResponse {
    pub success: bool,

    pub status: String,

    pub time_remaining_seconds: u64,

    pub can_confirm_receipt: bool,
    pub can_claim_timeout: bool,
    pub can_flag_refund: bool,
}

/* -------------------------------------------------------------------------- */
/*                             Internal Utilities                             */
/* -------------------------------------------------------------------------- */

/**
 * Converts an `EscrowStatus` enum into a frontend-friendly string.
 *
 * # Arguments
 *
 * * `status` - Escrow lifecycle status.
 *
 * # Returns
 *
 * Human-readable escrow status string.
 */
fn status_to_string(status: EscrowStatus) -> String {
    match status {
        EscrowStatus::ActiveAuction => "ActiveAuction".to_string(),

        EscrowStatus::AwaitingFinalization => "AwaitingFinalization".to_string(),

        EscrowStatus::AwaitingBuyerConfirmation => "AwaitingBuyerConfirmation".to_string(),

        EscrowStatus::Complete => "Complete".to_string(),

        EscrowStatus::Refunded => "Refunded".to_string(),
    }
}

/**
 * Retrieves the authenticated wallet address from the active session.
 *
 * # Arguments
 *
 * * `state` - Shared Axum application state.
 * * `cookies` - Cookie jar containing the session cookie.
 *
 * # Returns
 *
 * Authenticated wallet address.
 *
 * # Errors
 *
 * Returns a string error if:
 *     - The user is not authenticated
 *     - The session is invalid or expired
 *     - The wallet address cannot be parsed
 */
fn get_session_wallet(state: &Arc<AppState>, cookies: &Cookies) -> Result<Address, String> {
    let session_cookie = cookies
        .get("trust_session")
        .ok_or("Not authenticated — please sign in first")?;

    let session_id = session_cookie.value();

    let wallet_str = {
        let sessions = state.sessions.lock().unwrap();

        sessions
            .get(session_id)
            .ok_or("Invalid or expired session")?
            .clone()
    };

    wallet_str
        .parse()
        .map_err(|_| "Invalid wallet address in session".to_string())
}

/* -------------------------------------------------------------------------- */
/*                              Status Handlers                               */
/* -------------------------------------------------------------------------- */

/**
 * Retrieves escrow status information for an auction.
 *
 * This handler:
 *     - Validates the authenticated session
 *     - Retrieves escrow lifecycle status
 *     - Retrieves remaining confirmation time
 *     - Checks available escrow actions for the caller
 *     - Returns escrow metadata as JSON
 *
 * # Arguments
 *
 * * `state` - Shared Axum application state.
 * * `cookies` - Cookie jar containing the session cookie.
 * * `auction_address_str` - Auction contract address from the route path.
 *
 * # Returns
 *
 * JSON response containing:
 *     - escrow status
 *     - remaining confirmation time
 *     - available caller actions
 *
 * # Errors
 *
 * Returns a string error if:
 *     - Authentication fails
 *     - The auction address is invalid
 *     - Escrow status retrieval fails
 */
pub async fn escrow_status_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Path(auction_address_str): Path<String>,
) -> Result<Json<EscrowStatusResponse>, String> {
    let caller = get_session_wallet(&state, &cookies)?;

    let auction_address: Address = auction_address_str
        .parse()
        .map_err(|_| "Invalid auction address".to_string())?;

    let status = get_escrow_status(state.rpc_provider.as_ref(), auction_address)
        .await
        .map_err(|e| format!("Failed to fetch escrow status: {}", e))?;

    let time_remaining =
        get_time_remaining_for_confirmation(state.rpc_provider.as_ref(), auction_address)
            .await
            .unwrap_or(0);

    let can_confirm = can_confirm_receipt(state.rpc_provider.as_ref(), auction_address, caller)
        .await
        .unwrap_or(false);

    let can_claim = can_claim_timeout(state.rpc_provider.as_ref(), auction_address, caller)
        .await
        .unwrap_or(false);

    let can_refund = can_flag_refund(state.rpc_provider.as_ref(), auction_address, caller)
        .await
        .unwrap_or(false);

    Ok(Json(EscrowStatusResponse {
        success: true,

        status: status_to_string(status),

        time_remaining_seconds: time_remaining,

        can_confirm_receipt: can_confirm,

        can_claim_timeout: can_claim,

        can_flag_refund: can_refund,
    }))
}

/* -------------------------------------------------------------------------- */
/*                           Escrow Action Handlers                           */
/* -------------------------------------------------------------------------- */

/**
 * Ends an auction after the bidding period has expired.
 *
 * # Arguments
 *
 * * `state` - Shared Axum application state.
 * * `cookies` - Cookie jar containing the session cookie.
 * * `req` - Escrow action request payload.
 *
 * # Returns
 *
 * JSON response containing the transaction hash.
 *
 * # Errors
 *
 * Returns a string error if:
 *     - Authentication fails
 *     - The auction address is invalid
 *     - Auction finalization fails
 */
pub async fn end_auction_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<EscrowActionRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let caller = get_session_wallet(&state, &cookies)?;

    let auction_address: Address = req
        .auction_address
        .parse()
        .map_err(|_| "Invalid auction address".to_string())?;

    let tx_hash = end_auction(state.rpc_provider.as_ref(), auction_address, caller)
        .await
        .map_err(|e| format!("Failed to end auction: {}", e))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": tx_hash
    })))
}

/**
 * Confirms receipt of the auctioned item.
 *
 * # Arguments
 *
 * * `state` - Shared Axum application state.
 * * `cookies` - Cookie jar containing the session cookie.
 * * `req` - Escrow action request payload.
 *
 * # Returns
 *
 * JSON response containing the transaction hash.
 *
 * # Errors
 *
 * Returns a string error if:
 *     - Authentication fails
 *     - The auction address is invalid
 *     - Receipt confirmation fails
 */
pub async fn confirm_receipt_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<EscrowActionRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let caller = get_session_wallet(&state, &cookies)?;

    let auction_address: Address = req
        .auction_address
        .parse()
        .map_err(|_| "Invalid auction address".to_string())?;

    let result = confirm_receipt(state.rpc_provider.as_ref(), auction_address, caller)
        .await
        .map_err(|e| format!("Failed to confirm receipt: {}", e))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": result.tx_hash
    })))
}

/**
 * Claims escrow settlement after confirmation timeout expiration.
 *
 * # Arguments
 *
 * * `state` - Shared Axum application state.
 * * `cookies` - Cookie jar containing the session cookie.
 * * `req` - Escrow action request payload.
 *
 * # Returns
 *
 * JSON response containing the transaction hash.
 *
 * # Errors
 *
 * Returns a string error if:
 *     - Authentication fails
 *     - The auction address is invalid
 *     - Timeout claim fails
 */
pub async fn claim_timeout_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<EscrowActionRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let caller = get_session_wallet(&state, &cookies)?;

    let auction_address: Address = req
        .auction_address
        .parse()
        .map_err(|_| "Invalid auction address".to_string())?;

    let result = claim_after_timeout(state.rpc_provider.as_ref(), auction_address, caller)
        .await
        .map_err(|e| format!("Failed to claim timeout: {}", e))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": result.tx_hash
    })))
}

/**
 * Flags an escrow refund action.
 *
 * # Arguments
 *
 * * `state` - Shared Axum application state.
 * * `cookies` - Cookie jar containing the session cookie.
 * * `req` - Escrow action request payload.
 *
 * # Returns
 *
 * JSON response containing the transaction hash.
 *
 * # Errors
 *
 * Returns a string error if:
 *     - Authentication fails
 *     - The auction address is invalid
 *     - Refund flagging fails
 */
pub async fn refund_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<EscrowActionRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let caller = get_session_wallet(&state, &cookies)?;

    let auction_address: Address = req
        .auction_address
        .parse()
        .map_err(|_| "Invalid auction address".to_string())?;

    let result = flag_refund(state.rpc_provider.as_ref(), auction_address, caller)
        .await
        .map_err(|e| format!("Failed to flag refund: {}", e))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": result.tx_hash
    })))
}
