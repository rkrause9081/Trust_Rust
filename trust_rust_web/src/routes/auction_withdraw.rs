/*
 * auction_withdraw.rs
 *
 * Purpose:
 *     HTTP route handler for withdrawal requests.
 *
 *     Allows authenticated users to withdraw refundable funds
 *     from an auction escrow contract and returns withdrawal
 *     transaction details to the client.
 *
 * Responsibilities:
 *     - Parse withdrawal requests
 *     - Validate auction addresses
 *     - Retrieve authenticated wallet address
 *     - Execute withdrawal transactions
 *     - Return withdrawal results
 *
 * Non-Responsibilities:
 *     - Withdrawal contract implementation
 *     - Session creation
 *     - Provider initialization
 *     - Escrow settlement logic
 *
 * Architecture:
 *
 *       Client Request
 *              ↓
 *      auction_withdraw.rs
 *              ↓
 *      Session Validation
 *              ↓
 *      trust_rust_client::withdraw
 *              ↓
 *          Escrow Contract
 *
 * Endpoint:
 *     POST /api/withdraw
 */

use std::sync::Arc;

use alloy::primitives::Address;
use axum::{extract::State, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use trust_rust_client::withdraw;

use crate::{routes::session::get_session_wallet, state::AppState};

/* -------------------------------------------------------------------------- */
/*                             Request Structures                             */
/* -------------------------------------------------------------------------- */

/**
 * Withdrawal request payload.
 *
 * Identifies the auction whose escrow
 * funds should be withdrawn.
 */
#[derive(Debug, Deserialize)]
pub struct WithdrawRequest {
    pub auction_address: String,
}

/* -------------------------------------------------------------------------- */
/*                              Route Handlers                                */
/* -------------------------------------------------------------------------- */

/**
 * Handles escrow withdrawal requests.
 *
 * Retrieves the authenticated wallet address, validates
 * the auction address, executes the withdrawal transaction,
 * and returns withdrawal details to the client.
 *
 * # Request Body
 *
 * {
 *     "auction_address": "..."
 * }
 *
 * # Returns
 *
 * Success response:
 *
 * {
 *     "success": true,
 *     "tx_hash": "...",
 *     "amount_withdrawn_wei": "...",
 *     "amount_withdrawn_eth": "..."
 * }
 *
 * # Errors
 *
 * Returns an error if:
 *     - Session authentication fails
 *     - Auction address is invalid
 *     - Withdrawal transaction fails
 */
pub async fn withdraw_handler(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(req): Json<WithdrawRequest>,
) -> Result<Json<serde_json::Value>, String> {
    // Retrieve the authenticated wallet address from the session.
    let caller = get_session_wallet(&state, &cookies)?;

    // Parse and validate the auction contract address.
    let auction_address: Address = req
        .auction_address
        .trim()
        .parse()
        .map_err(|_| "Invalid auction address".to_string())?;

    // Execute the withdrawal transaction.
    let result = withdraw::withdraw(
        state.rpc_provider.as_ref(),
        auction_address,
        caller,
    )
    .await
    .map_err(|e| format!("Withdrawal failed: {e}"))?;

    // Return withdrawal transaction details.
    Ok(Json(serde_json::json!({
        "success": true,
        "tx_hash": result.tx_hash,
        "amount_withdrawn_wei": result.amount_withdrawn_wei.to_string(),
        "amount_withdrawn_eth": result.amount_withdrawn_eth
    })))
}