/*
 * auction_state.js
 *
 * Purpose:
 *     Shared auction cache and refresh coordinator.
 *
 *     Maintains a centralized source of truth for auction
 *     data within the frontend application. Responsible for
 *     loading auction information from the backend, normalizing
 *     auction fields, and coordinating UI refreshes.
 *
 * Responsibilities:
 *     - Store loaded auction data
 *     - Normalize auction field formats
 *     - Refresh auction listings
 *     - Coordinate auction UI updates
 *     - Expose shared auction helpers
 *
 * Non-Responsibilities:
 *     - API implementation
 *     - Auction rendering
 *     - Authentication
 *     - Blockchain interactions
 *
 * Architecture:
 *
 *         Backend API
 *              ↓
 *       auction_state.js
 *              ↓
 *       Auction Cache
 *              ↓
 *       UI Rendering
 *
 * Note:
 *     This is the only file that should decide when
 *     the entire auction UI reloads.
 */

/* -------------------------------------------------------------------------- */
/*                              Shared State                                  */
/* -------------------------------------------------------------------------- */

/**
 * Global auction state container.
 *
 * Stores cached auction data and refresh
 * metadata used throughout the frontend.
 */
const auctionState = {
    auctions: [],
    isLoading: false,
    lastLoadedAt: null,
};

/* -------------------------------------------------------------------------- */
/*                         Auction Field Accessors                            */
/* -------------------------------------------------------------------------- */

/**
 * Returns a normalized auction address.
 *
 * Supports both snake_case and camelCase
 * backend response formats.
 */
function getAuctionAddress(auction) {
    return auction.auction_address || auction.auctionAddress || "";
}

/**
 * Returns a normalized auction end time.
 */
function getEndTime(auction) {
    return Number(auction.end_time || auction.endTime || 0);
}

/**
 * Returns a normalized starting bid value in wei.
 */
function getStartingBidWei(auction) {
    return String(auction.starting_bid_wei || auction.startingBid || "0");
}

/**
 * Returns a normalized highest bid value in wei.
 */
function getHighestBidWei(auction) {
    return String(auction.highest_bid_wei || auction.highestBid || "0");
}

/**
 * Returns a normalized highest bidder address.
 */
function getHighestBidder(auction) {
    return auction.highest_bidder || auction.highestBidder || "";
}

/**
 * Returns a normalized bid count.
 */
function getBidCount(auction) {
    return Number(auction.bid_count || auction.bidCount || 0);
}

/* -------------------------------------------------------------------------- */
/*                           Data Normalization                               */
/* -------------------------------------------------------------------------- */

/**
 * Normalizes auction data returned from the backend.
 *
 * Converts mixed field naming conventions into a
 * consistent frontend representation.
 *
 * # Arguments
 *
 * * `rawAuctions` - Raw auction array returned by the API.
 *
 * # Returns
 *
 * Normalized auction array.
 */
function normalizeAuctions(rawAuctions) {
    return rawAuctions.map((auction) => ({
        ...auction,
        auction_address: getAuctionAddress(auction),
        end_time: getEndTime(auction),
        starting_bid_wei: getStartingBidWei(auction),
        highest_bid_wei: getHighestBidWei(auction),
        highest_bidder: getHighestBidder(auction),
        bid_count: getBidCount(auction),
    }));
}

/* -------------------------------------------------------------------------- */
/*                          Auction Refresh Logic                             */
/* -------------------------------------------------------------------------- */

/**
 * Reloads auction data from the backend.
 *
 * Fetches auction information through the API layer,
 * updates the shared cache, and triggers a full UI
 * refresh when rendering support is available.
 *
 * # Arguments
 *
 * * `options.showLoading`
 *     Controls whether loading UI is displayed.
 *
 * # Returns
 *
 * Updated normalized auction list.
 *
 * # Errors
 *
 * Throws if:
 *     - auction_api.js is not loaded
 *     - Auction API request fails
 */
async function refreshAuctions(options = {}) {
    const { showLoading = true } = options;

    // Ensure API layer is available.
    if (!window.auctionApi) {
        throw new Error("auction_api.js is not loaded.");
    }

    const grid = document.getElementById("auctionGrid");

    // Display loading state while auctions are fetched.
    if (showLoading && grid) {
        grid.innerHTML = `
            <p class="status-line">
                Loading auctions from blockchain...
            </p>
        `;
    }

    auctionState.isLoading = true;

    try {
        // Load auction data from the backend.
        const data = await window.auctionApi.getAuctions();

        const rawAuctions = Array.isArray(data.auctions)
            ? data.auctions
            : [];

        // Normalize auction records.
        auctionState.auctions = normalizeAuctions(rawAuctions);

        // Store last successful refresh time.
        auctionState.lastLoadedAt = Date.now();

        // Refresh the auction UI if rendering support exists.
        if (typeof renderAuctionUI === "function") {
            await renderAuctionUI(auctionState.auctions);
        }

        return auctionState.auctions;
    } finally {
        auctionState.isLoading = false;
    }
}

/* -------------------------------------------------------------------------- */
/*                         Backward Compatibility                             */
/* -------------------------------------------------------------------------- */

/**
 * Legacy auction loading helper.
 *
 * Maintained for compatibility with older
 * frontend modules and event handlers.
 *
 * New code should use refreshAuctions().
 */
async function loadActiveAuctions() {
    return refreshAuctions();
}

/* -------------------------------------------------------------------------- */
/*                              Global Exports                                */
/* -------------------------------------------------------------------------- */

// Expose shared auction state.
window.auctionState = auctionState;

// Expose refresh helpers.
window.refreshAuctions = refreshAuctions;
window.loadActiveAuctions = loadActiveAuctions;

// Expose normalization helpers.
window.getAuctionAddress = getAuctionAddress;
window.getEndTime = getEndTime;
window.getStartingBidWei = getStartingBidWei;
window.getHighestBidWei = getHighestBidWei;
window.getHighestBidder = getHighestBidder;
window.getBidCount = getBidCount;