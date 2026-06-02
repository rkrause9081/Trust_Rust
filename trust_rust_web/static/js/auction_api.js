/*
 * auction_api.js
 *
 * Single frontend API boundary for auction-related backend calls.
 * No rendering or DOM logic belongs here.
 */

async function apiParseJsonResponse(response) {
    const text = await response.text();

    let data = {};
    try {
        data = text ? JSON.parse(text) : {};
    } catch {
        throw new Error(text || "Invalid server response");
    }

    if (!response.ok || data.success === false) {
        throw new Error(
            data.error ||
            data.message ||
            `Request failed (${response.status})`
        );
    }

    return data;
}

async function apiGetAuctions() {
    const res = await fetch("/api/auctions", {
        method: "GET",
        credentials: "include",
    });

    return apiParseJsonResponse(res);
}

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

async function apiCreateAuction(payload) {
    const res = await fetch("/api/create-auction", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify(payload),
    });

    return apiParseJsonResponse(res);
}

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

async function apiGetEscrowStatus(auctionAddress) {
    const res = await fetch(`/api/escrow/status/${auctionAddress}`, {
        method: "GET",
        credentials: "include",
    });

    return apiParseJsonResponse(res);
}

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

window.auctionApi = {
    getAuctions: apiGetAuctions,
    placeBid: apiPlaceBid,
    createAuction: apiCreateAuction,
    withdraw: apiWithdraw,
    getEscrowStatus: apiGetEscrowStatus,
    escrowAction: apiEscrowAction,
};
