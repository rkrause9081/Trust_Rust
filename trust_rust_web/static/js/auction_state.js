/*
 * auction_state.js
 *
 * Shared auction cache and refresh coordinator.
 * This is the only file that should decide when the whole auction UI reloads.
 */

const auctionState = {
    auctions: [],
    isLoading: false,
    lastLoadedAt: null,
};

function getAuctionAddress(auction) {
    return auction.auction_address || auction.auctionAddress || "";
}

function getEndTime(auction) {
    return Number(auction.end_time || auction.endTime || 0);
}

function getStartingBidWei(auction) {
    return String(auction.starting_bid_wei || auction.startingBid || "0");
}

function getHighestBidWei(auction) {
    return String(auction.highest_bid_wei || auction.highestBid || "0");
}

function getHighestBidder(auction) {
    return auction.highest_bidder || auction.highestBidder || "";
}

function getBidCount(auction) {
    return Number(auction.bid_count || auction.bidCount || 0);
}

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

async function refreshAuctions(options = {}) {
    const { showLoading = true } = options;

    if (!window.auctionApi) {
        throw new Error("auction_api.js is not loaded.");
    }

    const grid = document.getElementById("auctionGrid");

    if (showLoading && grid) {
        grid.innerHTML = `
            <p class="status-line">
                Loading auctions from blockchain...
            </p>
        `;
    }

    auctionState.isLoading = true;

    try {
        const data = await window.auctionApi.getAuctions();
        const rawAuctions = Array.isArray(data.auctions) ? data.auctions : [];

        auctionState.auctions = normalizeAuctions(rawAuctions);
        auctionState.lastLoadedAt = Date.now();

        if (typeof renderAuctionUI === "function") {
            await renderAuctionUI(auctionState.auctions);
        }

        return auctionState.auctions;
    } finally {
        auctionState.isLoading = false;
    }
}

/*
 * Backward-compatible alias so older files/buttons do not explode.
 * New code should call refreshAuctions().
 */
async function loadActiveAuctions() {
    return refreshAuctions();
}

window.auctionState = auctionState;
window.refreshAuctions = refreshAuctions;
window.loadActiveAuctions = loadActiveAuctions;
window.getAuctionAddress = getAuctionAddress;
window.getEndTime = getEndTime;
window.getStartingBidWei = getStartingBidWei;
window.getHighestBidWei = getHighestBidWei;
window.getHighestBidder = getHighestBidder;
window.getBidCount = getBidCount;
