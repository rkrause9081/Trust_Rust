/*
 * session.rs
 *
 * Shared session helper for authenticated route handlers.
 */

use std::sync::Arc;

use alloy::primitives::Address;
use tower_cookies::Cookies;

use crate::state::AppState;

pub fn get_session_wallet(state: &Arc<AppState>, cookies: &Cookies) -> Result<Address, String> {
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
        .parse::<Address>()
        .map_err(|_| "Invalid wallet address in session".to_string())
}
