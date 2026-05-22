/*
 * auction_list.rs
 *
 * Purpose:
 *     Provides read-only HTTP handlers for retrieving auction
 *     registry data from the blockchain.
 *
 * Responsibilities:
 *     - Fetch auction registry entries
 *     - Return paginated auction lists
 *     - Format auction data as JSON API responses
 *     - Support frontend auction discovery views
 *
 * Non-Responsibilities:
 *     - Auction creation
 *     - Authentication handling
 *     - Blockchain transaction execution
 *     - Frontend-side filtering logic
 *
 * Architecture:
 *
 *      Browser / Frontend
 *              ↓
 *        auction_list.rs
 *              ↓
 *           AppState
 *              ↓
 *      trust_rust_client
 *              ↓
 *       Auction Registry
 *
 * Notes:
 *     Frontend logic is responsible for filtering
 *     auctions by status (active, ended, etc.).
 */

use std::sync::Arc;

use alloy::primitives::U256;

use axum::{extract::State, Json};

use serde::Serialize;

use trust_rust_client::registry::{self, RegistryAuction};

use crate::state::AppState;

/* -------------------------------------------------------------------------- */
/*                               Response Types                               */
/* -------------------------------------------------------------------------- */

/**
 * Response returned when fetching auction registry data.
 */
#[derive(Serialize)]
pub struct AuctionListResponse {
    pub success: bool,
    pub auctions: Vec<RegistryAuction>,
    pub count: usize,
}

/* -------------------------------------------------------------------------- */
/*                              Route Handlers                                */
/* -------------------------------------------------------------------------- */

/**
 * Retrieves auction registry entries from the blockchain.
 *
 * This handler:
 *     - Queries the on-chain auction registry
 *     - Retrieves a paginated set of auctions
 *     - Returns auction metadata as JSON
 *
 * Current pagination settings:
 *     - offset: 0
 *     - limit: 100
 *
 * # Arguments
 *
 * * `state` - Shared Axum application state.
 *
 * # Returns
 *
 * JSON response containing:
 *     - success status
 *     - auction registry entries
 *     - total returned count
 *
 * # Errors
 *
 * Returns a string error if the registry query fails.
 */
pub async fn list_auctions(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AuctionListResponse>, String> {
    let auctions = registry::get_auctions_paginated(
        state.rpc_provider.as_ref(),
        state.factory_address,
        U256::from(0u64),
        U256::from(100u64),
    )
    .await
    .map_err(|e| format!("Failed to load auctions: {}", e))?;

    let count = auctions.len();

    Ok(Json(AuctionListResponse {
        success: true,
        auctions,
        count,
    }))
}
