/*
 * auth.rs
 *
 * Purpose:
 *     Handles Sign-In With Ethereum (SIWE) authentication for
 *     the Trust Rust web server.
 *
 * Responsibilities:
 *     - Generate one-time authentication nonces
 *     - Verify signed SIWE messages
 *     - Recover signer wallet addresses
 *     - Create in-memory user sessions
 *     - Set HTTP-only session cookies
 *
 * Non-Responsibilities:
 *     - Persistent user storage
 *     - Blockchain transactions
 *     - Authorization for protected routes
 *     - Long-term session cleanup
 *
 * Architecture:
 *
 *      Browser Wallet
 *            ↓
 *       SIWE Message
 *            ↓
 *         auth.rs
 *            ↓
 *      AppState Sessions
 */

use std::sync::Arc;

use alloy_primitives::{Address, Signature};

use axum::{extract::State, Json};

use serde::{Deserialize, Serialize};

use tower_cookies::{Cookie, Cookies};

use uuid::Uuid;

use crate::state::AppState;

/* -------------------------------------------------------------------------- */
/*                              Response Types                                */
/* -------------------------------------------------------------------------- */

/**
 * Response returned when a new SIWE nonce is generated.
 */
#[derive(Serialize)]
pub struct NonceResponse {
    pub nonce: String,
}

/**
 * Request payload used to verify a signed SIWE message.
 */
#[derive(Deserialize)]
pub struct VerifyRequest {
    pub message: String,
    pub signature: String,
}

/**
 * Response returned after successful SIWE verification.
 */
#[derive(Serialize)]
pub struct VerifyResponse {
    pub success: bool,
    pub wallet: String,
}

/**
 * Response returned after logging out.
 */
#[derive(Serialize)]
pub struct LogoutResponse {
    pub success: bool,
}

/* -------------------------------------------------------------------------- */
/*                              Auth Handlers                                 */
/* -------------------------------------------------------------------------- */

/**
 * Generates and stores a one-time SIWE nonce.
 *
 * The nonce is stored in shared application state and must
 * be consumed during signature verification.
 *
 * # Arguments
 *
 * * `state` - Shared Axum application state.
 *
 * # Returns
 *
 * JSON response containing the generated nonce.
 */
pub async fn get_nonce(State(state): State<Arc<AppState>>) -> Json<NonceResponse> {
    let nonce = Uuid::new_v4().to_string();

    state
        .nonces
        .lock()
        .unwrap()
        .insert(nonce.clone(), nonce.clone());

    Json(NonceResponse { nonce })
}

/**
 * Verifies a signed SIWE message and creates a user session.
 *
 * This handler:
 *     - Extracts the nonce from the signed message
 *     - Extracts the claimed wallet address
 *     - Consumes the nonce to prevent replay attacks
 *     - Recovers the signer from the submitted signature
 *     - Compares the recovered signer with the claimed wallet
 *     - Creates an in-memory session
 *     - Sets an HTTP-only session cookie
 *
 * # Arguments
 *
 * * `state` - Shared Axum application state.
 * * `cookies` - Cookie jar used to set the session cookie.
 * * `payload` - Signed SIWE message and signature.
 *
 * # Returns
 *
 * JSON response indicating success and the authenticated wallet.
 *
 * # Errors
 *
 * Returns a string error if:
 *     - The nonce is missing
 *     - The wallet address is missing or invalid
 *     - The nonce is invalid or expired
 *     - The signature format is invalid
 *     - The signer cannot be recovered
 *     - The recovered signer does not match the claimed wallet
 */
pub async fn verify_siwe(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(payload): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, String> {
    let nonce = payload
        .message
        .lines()
        .find(|line| line.starts_with("Nonce: "))
        .and_then(|line| line.strip_prefix("Nonce: "))
        .ok_or("Nonce missing")?
        .to_string();

    let claimed_wallet_line = payload
        .message
        .lines()
        .find(|line| line.starts_with("0x"))
        .ok_or("Wallet missing")?;

    let claimed_wallet: Address = claimed_wallet_line
        .parse()
        .map_err(|_| "Invalid wallet address")?;

    let valid_nonce = state.nonces.lock().unwrap().remove(&nonce);

    if valid_nonce.is_none() {
        return Err("Invalid or expired nonce".into());
    }

    let sig: Signature = payload
        .signature
        .parse()
        .map_err(|_| "Invalid signature format")?;

    let recovered = sig
        .recover_address_from_msg(payload.message.as_bytes())
        .map_err(|_| "Failed to recover signer")?;

    if recovered != claimed_wallet {
        return Err("Signature does not match claimed address".into());
    }

    let session_id = Uuid::new_v4().to_string();

    state
        .sessions
        .lock()
        .unwrap()
        .insert(session_id.clone(), recovered.to_string());

    cookies.add(
        Cookie::build(("trust_session", session_id))
            .path("/")
            .http_only(true)
            .secure(false)
            .into(),
    );

    Ok(Json(VerifyResponse {
        success: true,
        wallet: recovered.to_string(),
    }))
}
/**
 * Logs out the active user by removing the server-side session and clearing
 * the trust_session cookie.
 *
 * # Arguments
 *
 * * `state` - Shared Axum application state.
 * * `cookies` - Cookie jar containing the session cookie.
 *
 * # Returns
 *
 * JSON response indicating successful logout.
 */
pub async fn logout(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
) -> Json<LogoutResponse> {
    if let Some(session_cookie) = cookies.get("trust_session") {
        let session_id = session_cookie.value().to_string();

        state
            .sessions
            .lock()
            .unwrap()
            .remove(&session_id);
    }

    cookies.remove(
        Cookie::build(("trust_session", ""))
            .path("/")
            .http_only(true)
            .secure(false)
            .into(),
    );

    Json(LogoutResponse { success: true })
}
