use axum::{extract::State, Json};
use serde::Deserialize;
use std::sync::Arc;
use tower_cookies::Cookies;
use alloy::primitives::Address;
use trust_rust_client::withdraw;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct WithdrawRequest {
    pub auction_address: String,
}

pub async fn withdraw_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<WithdrawRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let session_cookie = cookies.get("trust_session")
        .ok_or("Not authenticated – please sign in first")?;

    let session_id = session_cookie.value();
    let caller_str = {
        let sessions = state.sessions.lock().unwrap();
        sessions.get(session_id)
            .ok_or("Invalid or expired session")?
            .clone()
    };

    let caller: Address = caller_str.parse()
        .map_err(|_| "Invalid caller address in session")?;

    let auction_address: Address = req.auction_address.parse()
        .map_err(|_| "Invalid auction address")?;

    let result = withdraw::withdraw(
        &*state.rpc_provider,           // This was causing the Sized error
        auction_address,
        caller,
    ).await.map_err(|e| format!("Withdrawal failed: {}", e))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": result.tx_hash,
        "amount_withdrawn_wei": result.amount_withdrawn_wei.to_string(),
        "amount_withdrawn_eth": result.amount_withdrawn_eth
    })))
}