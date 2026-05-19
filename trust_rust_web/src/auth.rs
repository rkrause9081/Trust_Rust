/*
 * auth.rs
 *
 * Purpose:
 *     Handles Sign-In With Ethereum (SIWE) authentication.
 *
 *     This module is responsible for:
 *         - Generating one-time nonces
 *         - Verifying signed SIWE messages
 *         - Creating and managing user sessions via cookies
 *
 *     It does NOT:
 *         - Store persistent user data (in-memory only)
 *         - Perform blockchain transactions
 */

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use alloy_primitives::{Address, Signature};
use tower_cookies::{Cookies, Cookie};
use std::sync::Arc;

use crate::state::AppState;

/* ====================== RESPONSE TYPES ====================== */

#[derive(Serialize)]
pub struct NonceResponse {
    pub nonce: String,
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub message: String,
    pub signature: String,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub success: bool,
    pub wallet: String,
}

/* ====================== HANDLERS ====================== */

pub async fn get_nonce(State(state): State<Arc<AppState>>) -> Json<NonceResponse> {
    let nonce = Uuid::new_v4().to_string();

    state.nonces.lock().unwrap().insert(nonce.clone(), nonce.clone());

    Json(NonceResponse { nonce })
}

pub async fn verify_siwe(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(payload): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, String> {

    // Extract nonce from message (for replay protection)
    let nonce = payload.message
        .lines()
        .find(|l| l.starts_with("Nonce: "))
        .and_then(|l| l.strip_prefix("Nonce: "))
        .ok_or("Nonce missing")?
        .to_string();

    // Extract claimed wallet address
    let claimed_wallet_line = payload.message
        .lines()
        .find(|l| l.starts_with("0x"))
        .ok_or("Wallet missing")?;

    let claimed_wallet: Address = claimed_wallet_line.parse()
        .map_err(|_| "Invalid wallet address")?;

    // Consume nonce (single-use)
    let valid_nonce = state.nonces.lock().unwrap().remove(&nonce);
    if valid_nonce.is_none() {
        return Err("Invalid or expired nonce".into());
    }

    // Verify signature
    let sig: Signature = payload.signature.parse()
        .map_err(|_| "Invalid signature format")?;

    let recovered = sig
        .recover_address_from_msg(payload.message.as_bytes())
        .map_err(|_| "Failed to recover signer")?;

    if recovered != claimed_wallet {
        return Err("Signature does not match claimed address".into());
    }

    // Create session
    let session_id = Uuid::new_v4().to_string();
    state.sessions.lock().unwrap()
        .insert(session_id.clone(), recovered.to_string());

    // Set HTTP-only session cookie
    cookies.add(
        Cookie::build(("trust_session", session_id))
            .path("/")
            .http_only(true)
            .secure(false) // set to true in production with HTTPS
            .into()
    );

    Ok(Json(VerifyResponse {
        success: true,
        wallet: recovered.to_string(),
    }))
}