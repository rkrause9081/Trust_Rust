/*
 * auction_query.rs
 *
 * Purpose:
 *     Read-only auction registry route.
 *
 *     Retrieves all registered auctions from the factory
 *     registry and returns them as a JSON response for
 *     frontend consumption.
 *
 * Responsibilities:
 *     - Query registered auction addresses
 *     - Load registry metadata for each auction
 *     - Build API response payloads
 *     - Return auction listings to clients
 *
 * Non-Responsibilities:
 *     - Auction creation
 *     - Bid placement
 *     - Escrow operations
 *     - Registry contract implementation
 *
 * Architecture:
 *
 *        Client Request
 *               ↓
 *       auction_query.rs
 *               ↓
 *       Registry Client
 *               ↓
 *       AuctionFactory
 *               ↓
 *       Auction Registry
 *
 * Endpoint:
 *     GET /api/auctions
 */

use std::sync::Arc;

use axum::{extract::State, Json};
use serde::Serialize;

use trust_rust_client::registry::{self, RegistryAuction};

use crate::state::AppState;

/* -------------------------------------------------------------------------- */
/*                             Response Structures                            */
/* -------------------------------------------------------------------------- */

/**
 * Auction listing API response.
 *
 * Contains all loaded registry auctions along
 * with metadata describing the result set.
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
 * Returns all registered auctions.
 *
 * Retrieves every registered auction address from the
 * auction factory registry, loads the registry metadata
 * for each auction individually, and returns the results
 * as a JSON response.
 *
 * # Implementation Notes
 *
 * Uses individual registry lookups rather than
 * getAuctionsPaginated() while debugging local
 * Hardhat environments.
 *
 * Workflow:
 *     1. Load all registered auction addresses
 *     2. Load registry metadata for each address
 *     3. Build auction list
 *     4. Return JSON response
 *
 * # Returns
 *
 * Success response:
 *
 * {
 *     "success": true,
 *     "auctions": [...],
 *     "count": n
 * }
 *
 * # Errors
 *
 * Returns an error if:
 *     - Registry address lookup fails
 *     - Registry item retrieval fails
 */
pub async fn list_auctions_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AuctionListResponse>, String> {
    /*
     * Safer than getAuctionsPaginated() while you are debugging local Hardhat noise:
     * 1. read auction addresses
     * 2. read each registry item individually
     */

    // Load all registered auction addresses from the factory.
    let addresses = registry::get_auctions(
        state.rpc_provider.as_ref(),
        state.factory_address,
    )
    .await
    .map_err(|e| format!("Failed to load auction addresses: {e}"))?;

    // Preallocate storage for loaded auction records.
    let mut auctions = Vec::with_capacity(addresses.len());

    // Load registry metadata for every auction individually.
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

    // Calculate the total number of loaded auctions.
    let count = auctions.len();

    // Return auction data to the client.
    Ok(Json(AuctionListResponse {
        success: true,
        auctions,
        count,
    }))
}
