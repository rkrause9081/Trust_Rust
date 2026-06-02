/*
 * session.rs
 *
 * Purpose:
 *     Shared session helper for authenticated route handlers.
 *
 *     Retrieves the authenticated wallet address associated
 *     with the current session cookie and validates that the
 *     session is active.
 *
 * Responsibilities:
 *     - Read session cookies
 *     - Validate session existence
 *     - Retrieve stored wallet addresses
 *     - Convert wallet strings into Address values
 *     - Provide authenticated user identity
 *
 * Non-Responsibilities:
 *     - Session creation
 *     - Session expiration management
 *     - Authentication verification
 *     - Cookie generation
 *
 * Architecture:
 *
 *         Route Handler
 *                ↓
 *           session.rs
 *                ↓
 *         Session Cookie
 *                ↓
 *       AppState Session Map
 *                ↓
 *         Wallet Address
 */

use std::sync::Arc;

use alloy::primitives::Address;
use tower_cookies::Cookies;

use crate::state::AppState;

/* -------------------------------------------------------------------------- */
/*                            Session Helper Logic                            */
/* -------------------------------------------------------------------------- */

/**
 * Retrieves the authenticated wallet address from the session.
 *
 * Reads the trust_session cookie, validates that the session
 * exists in application state, and converts the stored wallet
 * address into a strongly-typed Address value.
 *
 * # Arguments
 *
 * * `state` - Shared application state.
 * * `cookies` - Incoming request cookies.
 *
 * # Returns
 *
 * Authenticated wallet address associated with
 * the active session.
 *
 * # Errors
 *
 * Returns an error if:
 *     - Session cookie is missing
 *     - Session does not exist
 *     - Session has expired
 *     - Stored wallet address is invalid
 */
pub fn get_session_wallet(
    state: &Arc<AppState>,
    cookies: &Cookies,
) -> Result<Address, String> {
    // Retrieve the session cookie from the request.
    let session_cookie = cookies
        .get("trust_session")
        .ok_or("Not authenticated — please sign in first")?;

    // Extract the session identifier.
    let session_id = session_cookie.value();

    // Look up the wallet address associated with the session.
    let wallet_str = {
        let sessions = state.sessions.lock().unwrap();

        sessions
            .get(session_id)
            .ok_or("Invalid or expired session")?
            .clone()
    };

    // Convert the stored wallet string into an Address type.
    wallet_str
        .parse::<Address>()
        .map_err(|_| "Invalid wallet address in session".to_string())
}
