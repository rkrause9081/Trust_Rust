/*
 * create_auction.js
 *
 * Frontend handler for creating auctions.
 */

function ethToWeiString(ethValue) {
    const normalized = String(ethValue).trim();

    if (!/^\d+(\.\d{0,18})?$/.test(normalized)) {
        throw new Error("Invalid starting bid");
    }

    const [whole, fraction = ""] = normalized.split(".");
    const paddedFraction = fraction.padEnd(18, "0");

    return BigInt(whole + paddedFraction).toString();
}

async function createAuction(event) {
    event.preventDefault();

    const status = document.getElementById("createStatus");
    status.textContent = "Creating auction...";

    try {
        const title = document.getElementById("auctionTitle").value.trim();
        const description = document.getElementById("auctionDescription").value.trim();
        const assetType = document.getElementById("assetType").value;
        const startingBidInput = document.getElementById("startingBid").value;
        const endDate = document.getElementById("endDate").value;

        if (!title || !description) {
            throw new Error("Missing title or description");
        }

        const endTimestamp = Math.floor(new Date(endDate).getTime() / 1000);
        const now = Math.floor(Date.now() / 1000);

        if (!Number.isFinite(endTimestamp) || endTimestamp <= now) {
            throw new Error("Auction end date must be in the future");
        }

        const biddingTimeSeconds = endTimestamp - now;
        const startingBidWei = ethToWeiString(startingBidInput);

        const response = await fetch("/api/create-auction", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            credentials: "include",
            body: JSON.stringify({
                bidding_time_seconds: biddingTimeSeconds,
                starting_bid_wei: startingBidWei,
                title,
                description: `[${assetType}] ${description}`,
            }),
        });

        const data = await response.json();

        if (!response.ok) {
            throw new Error(data.error || data.message || "Failed to create auction");
        }

        status.textContent = `Auction created: ${data.auction_address}`;
        document.getElementById("createAuctionForm").reset();

        if (typeof loadAuctions === "function") {
            await loadAuctions();
        }
    } catch (err) {
        console.error(err);
        status.textContent = err.message;
    }
}

document.addEventListener("DOMContentLoaded", () => {
    const form = document.getElementById("createAuctionForm");

    if (form) {
        form.addEventListener("submit", createAuction);
    }
});
