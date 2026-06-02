/*
 * auction_api.js
 *
 * Purpose:
 *     Frontend API client for auction-related backend endpoints.
 *
 *     Provides a centralized interface for communicating with
 *     the backend auction and escrow APIs. All HTTP request
 *     logic is isolated here so UI components remain focused
 *     on rendering and user interaction.
 *
 * Responsibilities:
 *     - Execute auction API requests
 *     - Execute escrow API requests
 *     - Handle JSON response parsing
 *     - Normalize API error handling
 *     - Expose a unified frontend API interface
 *
 * Non-Responsibilities:
 *     - DOM manipulation
 *     - UI rendering
 *     - Form validation
 *     - Wallet authentication logic
 *
 * Architecture:
 *
 *          UI Components
 *                 ↓
 *          auction_api.js
 *                 ↓
 *           Backend API
 *                 ↓
 *     ┌───────────┼───────────┐
 *     ↓           ↓           ↓
 *  Auctions     Bidding     Escrow
 *
 * Endpoints:
 *     GET  /api/auctions
 *     POST /api/create-auction
 *     POST /api/bid
 *     POST /api/withdraw
 *     GET  /api/escrow/status/{auction}
 *     POST /api/escrow/*
 */

/* -------------------------------------------------------------------------- */
/*                          Shared Response Handling                          */
/* -------------------------------------------------------------------------- */

/**
 * Parses and validates API responses.
 *
 * Converts the response body into JSON and normalizes
 * backend error handling so all callers receive a
 * consistent exception format.
 *
 * # Arguments
 *
 * * `response` - Fetch API response object.
 *
 * # Returns
 *
 * Parsed JSON response object.
 *
 * # Errors
 *
 * Throws if:
 *     - JSON parsing fails
 *     - HTTP request failed
 *     - Backend returns success: false
 */
async function apiParseJsonResponse(response) {
    // Read the raw response body.
    const text = await response.text();

    let data = {};

    // Attempt JSON parsing.
    try {
        data = text ? JSON.parse(text) : {};
    } catch {
        throw new Error(text || "Invalid server response");
    }

    // Normalize API and HTTP errors.
    if (!response.ok || data.success === false) {
        throw new Error(
            data.error ||
            data.message ||
            `Request failed (${response.status})`
        );
    }

    return data;
}

/* -------------------------------------------------------------------------- */
/*                              Auction Endpoints                             */
/* -------------------------------------------------------------------------- */

/**
 * Retrieves all registered auctions.
 *
 * # Returns
 *
 * Auction listing response from the backend.
 */
async function apiGetAuctions() {
    const res = await fetch("/api/auctions", {
        method: "GET",
        credentials: "include",
    });

    return apiParseJsonResponse(res);
}

/**
 * Places a bid on an auction.
 *
 * # Arguments
 *
 * * `auctionAddress` - Auction contract address.
 * * `bidAmountWei` - Bid amount in wei.
 *
 * # Returns
 *
 * Bid transaction result.
 */
async function apiPlaceBid(auctionAddress, bidAmountWei) {
    const res = await fetch("/api/bid", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({
            auction_address: auctionAddress,
            bid_amount_wei: bidAmountWei,
        }),
    });

    return apiParseJsonResponse(res);
}

/**
 * Creates a new auction.
 *
 * # Arguments
 *
 * * `payload` - Auction creation request payload.
 *
 * # Returns
 *
 * Auction creation result.
 */
async function apiCreateAuction(payload) {
    const res = await fetch("/api/create-auction", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify(payload),
    });

    return apiParseJsonResponse(res);
}

/**
 * Withdraws refundable funds from an auction.
 *
 * # Arguments
 *
 * * `auctionAddress` - Auction contract address.
 *
 * # Returns
 *
 * Withdrawal transaction result.
 */
async function apiWithdraw(auctionAddress) {
    const res = await fetch("/api/withdraw", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({
            auction_address: auctionAddress,
        }),
    });

    return apiParseJsonResponse(res);
}

/* -------------------------------------------------------------------------- */
/*                               Escrow Endpoints                             */
/* -------------------------------------------------------------------------- */

/**
 * Retrieves escrow status information.
 *
 * # Arguments
 *
 * * `auctionAddress` - Auction contract address.
 *
 * # Returns
 *
 * Escrow status response.
 */
async function apiGetEscrowStatus(auctionAddress) {
    const res = await fetch(`/api/escrow/status/${auctionAddress}`, {
        method: "GET",
        credentials: "include",
    });

    return apiParseJsonResponse(res);
}

/**
 * Executes a generic escrow action.
 *
 * Used for actions such as:
 *     - end auction
 *     - confirm receipt
 *     - claim timeout
 *     - flag refund
 *
 * # Arguments
 *
 * * `endpoint` - Escrow API endpoint.
 * * `auctionAddress` - Auction contract address.
 *
 * # Returns
 *
 * Escrow action result.
 */
async function apiEscrowAction(endpoint, auctionAddress) {
    const res = await fetch(endpoint, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({
            auction_address: auctionAddress,
        }),
    });

    return apiParseJsonResponse(res);
}

/* -------------------------------------------------------------------------- */
/*                              Public API Export                             */
/* -------------------------------------------------------------------------- */

/**
 * Global auction API namespace.
 *
 * Exposes all backend communication methods
 * to frontend modules and UI components.
 */
window.auctionApi = {
    getAuctions: apiGetAuctions,
    placeBid: apiPlaceBid,
    createAuction: apiCreateAuction,
    withdraw: apiWithdraw,
    getEscrowStatus: apiGetEscrowStatus,
    escrowAction: apiEscrowAction,
};