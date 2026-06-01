/*
 * auction_query.rs
 *
 * Read-only auction registry route.
 */

use std::sync::Arc;

use axum::{extract::State, Json};
use serde::Serialize;

use trust_rust_client::registry::{self, RegistryAuction};

use crate::state::AppState;

#[derive(Serialize)]
pub struct AuctionListResponse {
    pub success: bool,
    pub auctions: Vec<RegistryAuction>,
    pub count: usize,
}

pub async fn list_auctions_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AuctionListResponse>, String> {
    /*
     * Safer than getAuctionsPaginated() while you are debugging local Hardhat noise:
     * 1. read auction addresses
     * 2. read each registry item individually
     */
    let addresses = registry::get_auctions(state.rpc_provider.as_ref(), state.factory_address)
        .await
        .map_err(|e| format!("Failed to load auction addresses: {e}"))?;

    let mut auctions = Vec::with_capacity(addresses.len());

    for address in addresses {
        let auction = registry::get_auction_registry_item(
            state.rpc_provider.as_ref(),
            state.factory_address,
            address,
        )
        .await
        .map_err(|e| format!("Failed to load auction {address}: {e}"))?;

        auctions.push(auction);
    }

    let count = auctions.len();

    Ok(Json(AuctionListResponse {
        success: true,
        auctions,
        count,
    }))
}
