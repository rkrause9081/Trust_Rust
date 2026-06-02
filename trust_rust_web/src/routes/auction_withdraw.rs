/*
 * auction_withdraw.rs
 *
 * POST /api/withdraw
 */

use std::sync::Arc;

use alloy::primitives::Address;
use axum::{extract::State, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use trust_rust_client::withdraw;

use crate::{routes::session::get_session_wallet, state::AppState};

#[derive(Debug, Deserialize)]
pub struct WithdrawRequest {
    pub auction_address: String,
}

pub async fn withdraw_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<WithdrawRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let caller = get_session_wallet(&state, &cookies)?;

    let auction_address: Address = req
        .auction_address
        .trim()
        .parse()
        .map_err(|_| "Invalid auction address".to_string())?;

    let result = withdraw::withdraw(state.rpc_provider.as_ref(), auction_address, caller)
        .await
        .map_err(|e| format!("Withdrawal failed: {e}"))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": result.tx_hash,
        "amount_withdrawn_wei": result.amount_withdrawn_wei.to_string(),
        "amount_withdrawn_eth": result.amount_withdrawn_eth
    })))
}
