/*
 * withdraw.rs
 *
 * Purpose:
 *     Provides HTTP handlers for withdrawing pending auction funds.
 *
 * Responsibilities:
 *     - Validate authenticated user sessions
 *     - Parse withdrawal request payloads
 *     - Execute blockchain withdrawal transactions
 *     - Return withdrawal metadata as JSON
 *
 * Non-Responsibilities:
 *     - SIWE authentication verification
 *     - Smart contract interaction internals
 *     - Auction creation
 *     - Provider initialization
 *
 * Architecture:
 *
 *      Browser / Frontend
 *              ↓
 *         withdraw.rs
 *              ↓
 *           AppState
 *              ↓
 *      trust_rust_client
 *              ↓
 *       Auction Contract
 */

use std::sync::Arc;

use alloy::primitives::Address;

use axum::{extract::State, Json};

use serde::Deserialize;

use tower_cookies::Cookies;

use trust_rust_client::withdraw;

use crate::state::AppState;

/* -------------------------------------------------------------------------- */
/*                               Request Types                                */
/* -------------------------------------------------------------------------- */

/**
 * Request payload used for withdrawal actions.
 */
#[derive(Deserialize)]
pub struct WithdrawRequest {
    pub auction_address: String,
}

/* -------------------------------------------------------------------------- */
/*                              Route Handlers                                */
/* -------------------------------------------------------------------------- */

/**
 * Withdraws pending auction funds for the authenticated user.
 *
 * This handler:
 *     - Validates the active SIWE session
 *     - Retrieves the authenticated wallet address
 *     - Parses the auction contract address
 *     - Executes the blockchain withdrawal transaction
 *     - Returns withdrawal metadata as JSON
 *
 * # Arguments
 *
 * * `state` - Shared Axum application state.
 * * `cookies` - Cookie jar containing the session cookie.
 * * `req` - Withdrawal request payload.
 *
 * # Returns
 *
 * JSON response containing:
 *     - success status
 *     - transaction hash
 *     - withdrawn amount in wei
 *     - withdrawn amount in ETH
 *
 * # Errors
 *
 * Returns a string error if:
 *     - The user is not authenticated
 *     - The session is invalid or expired
 *     - The caller address is invalid
 *     - The auction address is invalid
 *     - Withdrawal execution fails
 */
pub async fn withdraw_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<WithdrawRequest>,
) -> Result<Json<serde_json::Value>, String> {
    let session_cookie = cookies
        .get("trust_session")
        .ok_or("Not authenticated — please sign in first")?;

    let session_id = session_cookie.value();

    let caller_str = {
        let sessions = state.sessions.lock().unwrap();

        sessions
            .get(session_id)
            .ok_or("Invalid or expired session")?
            .clone()
    };

    let caller: Address = caller_str
        .parse()
        .map_err(|_| "Invalid caller address in session")?;

    let auction_address: Address = req
        .auction_address
        .parse()
        .map_err(|_| "Invalid auction address")?;

    let result = withdraw::withdraw(&*state.rpc_provider, auction_address, caller)
        .await
        .map_err(|e| format!("Withdrawal failed: {}", e))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": result.tx_hash,
        "amount_withdrawn_wei":
            result.amount_withdrawn_wei.to_string(),
        "amount_withdrawn_eth":
            result.amount_withdrawn_eth
    })))
}
