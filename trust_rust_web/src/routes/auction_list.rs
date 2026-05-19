/*
 * auction_list.rs
 *
 * Purpose:
 *     Read-only handlers for fetching auction data from the on-chain registry.
 *     Used to populate the "Active Auctions" section on the frontend.
 *
 *     This file is SEPARATE from auction.rs (which only handles creation).
 */

use axum::{extract::State, Json};
use serde::Serialize;
use std::sync::Arc;
use alloy::primitives::U256;
use trust_rust_client::registry::{self, RegistryAuction};
use crate::state::AppState;

#[derive(Serialize)]
pub struct AuctionListResponse {
    pub success: bool,
    pub auctions: Vec<RegistryAuction>,
    pub count: usize,
}

/// GET /api/auctions
/// Returns auctions from the blockchain registry.
/// Frontend filters which ones are currently active.
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

    // Compute the length BEFORE moving `auctions`
    let count = auctions.len();

    Ok(Json(AuctionListResponse {
        success: true,
        auctions,      // move happens here
        count,
    }))
}