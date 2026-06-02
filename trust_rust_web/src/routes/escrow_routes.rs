/*
 * escrow_routes.rs
 *
 * Escrow lifecycle routes.
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

use crate::{routes::session::get_session_wallet, state::AppState};

#[derive(Debug, Deserialize)]
pub struct EscrowActionRequest {
    pub auction_address: String,
}

#[derive(Debug, Serialize)]
pub struct EscrowStatusResponse {
    pub success: bool,
    pub status: String,
    pub time_remaining_seconds: u64,
    pub can_confirm_receipt: bool,
    pub can_claim_timeout: bool,
    pub can_flag_refund: bool,
}

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
        .map_err(|e| format!("Failed to fetch escrow status: {e}"))?;

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

pub async fn end_auction_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<EscrowActionRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let caller = get_session_wallet(&state, &cookies)?;
    let auction_address = parse_auction_address(&req.auction_address)?;

    let tx_hash = end_auction(state.rpc_provider.as_ref(), auction_address, caller)
        .await
        .map_err(|e| format!("Failed to end auction: {e}"))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": tx_hash
    })))
}

pub async fn confirm_receipt_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<EscrowActionRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let caller = get_session_wallet(&state, &cookies)?;
    let auction_address = parse_auction_address(&req.auction_address)?;

    let result = confirm_receipt(state.rpc_provider.as_ref(), auction_address, caller)
        .await
        .map_err(|e| format!("Failed to confirm receipt: {e}"))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": result.tx_hash
    })))
}

pub async fn claim_timeout_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<EscrowActionRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let caller = get_session_wallet(&state, &cookies)?;
    let auction_address = parse_auction_address(&req.auction_address)?;

    let result = claim_after_timeout(state.rpc_provider.as_ref(), auction_address, caller)
        .await
        .map_err(|e| format!("Failed to claim timeout: {e}"))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": result.tx_hash
    })))
}

pub async fn refund_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<EscrowActionRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let caller = get_session_wallet(&state, &cookies)?;
    let auction_address = parse_auction_address(&req.auction_address)?;

    let result = flag_refund(state.rpc_provider.as_ref(), auction_address, caller)
        .await
        .map_err(|e| format!("Failed to flag refund: {e}"))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": result.tx_hash
    })))
}

fn status_to_string(status: EscrowStatus) -> String {
    match status {
        EscrowStatus::ActiveAuction => "ActiveAuction".to_string(),
        EscrowStatus::AwaitingFinalization => "AwaitingFinalization".to_string(),
        EscrowStatus::AwaitingBuyerConfirmation => "AwaitingBuyerConfirmation".to_string(),
        EscrowStatus::Complete => "Complete".to_string(),
        EscrowStatus::Refunded => "Refunded".to_string(),
    }
}

fn parse_auction_address(raw: &str) -> Result<Address, String> {
    raw.trim()
        .parse()
        .map_err(|_| "Invalid auction address".to_string())
}
