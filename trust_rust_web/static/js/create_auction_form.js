/*
 * create_auction_form.js
 *
 * Create auction form behavior only.
 */

function setupCreateAuctionForm() {
    const form = document.getElementById("createAuctionForm");

    if (!form || form.dataset.bound === "true") {
        return;
    }

    form.dataset.bound = "true";

    form.addEventListener("submit", async (event) => {
        event.preventDefault();

        const status = document.getElementById("createStatus");

        if (!status) {
            return;
        }

        status.textContent = "Creating auction...";
        status.style.color = "var(--accent, #00e5ff)";

        try {
            const title = document.getElementById("auctionTitle")?.value.trim();
            const description = document.getElementById("auctionDescription")?.value.trim();
            const assetType = document.getElementById("assetType")?.value || "Other";
            const startingBidInput = document.getElementById("startingBid")?.value;
            const endDate = document.getElementById("endDate")?.value;

            if (!title || !description) {
                throw new Error("Missing title or description.");
            }

            const endTimestamp = Math.floor(new Date(endDate).getTime() / 1000);
            const now = Math.floor(Date.now() / 1000);

            if (!Number.isFinite(endTimestamp) || endTimestamp <= now) {
                throw new Error("Auction end date must be in the future.");
            }

            const biddingTimeSeconds = endTimestamp - now;
            const startingBidWei = ethToWeiString(startingBidInput);

            const data = await window.auctionApi.createAuction({
                bidding_time_seconds: biddingTimeSeconds,
                starting_bid_wei: startingBidWei,
                title,
                description: `[${assetType}] ${description}`,
            });

            status.textContent = `Auction created: ${data.auction_address}`;
            status.style.color = "var(--success, #32ff9a)";

            form.reset();

            /*
             * Old file called loadAuctions(); that function did not exist.
             * Always refresh through the shared state coordinator.
             */
            await refreshAuctions({ showLoading: true });
        } catch (err) {
            console.error("Create auction failed:", err);
            status.textContent = `❌ ${err.message}`;
            status.style.color = "var(--danger, #ff4d6d)";
        }
    });
}

window.setupCreateAuctionForm = setupCreateAuctionForm;
