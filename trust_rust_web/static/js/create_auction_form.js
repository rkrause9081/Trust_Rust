/*
 * create_auction_form.js
 *
 * Purpose:
 *     Auction creation form behavior and submission logic.
 *
 *     Handles user auction creation requests, validates form
 *     input, converts user-friendly values into blockchain
 *     compatible formats, submits auction creation transactions,
 *     and refreshes auction listings after success.
 *
 * Responsibilities:
 *     - Validate auction form input
 *     - Convert ETH values into wei
 *     - Calculate auction duration
 *     - Submit auction creation requests
 *     - Display creation status messages
 *     - Refresh auction listings
 *     - Prevent duplicate event registration
 *
 * Non-Responsibilities:
 *     - Auction rendering
 *     - Authentication
 *     - API implementation
 *     - Blockchain transaction execution
 *
 * Architecture:
 *
 *          User Input
 *               ↓
 *      create_auction_form.js
 *               ↓
 *         auction_api.js
 *               ↓
 *          Backend API
 *               ↓
 *        Auction Factory
 */

/* -------------------------------------------------------------------------- */
/*                        Auction Form Initialization                         */
/* -------------------------------------------------------------------------- */

/**
 * Initializes the auction creation form.
 *
 * Registers the form submission handler, validates
 * user input, converts values into blockchain-friendly
 * formats, submits the auction creation request, and
 * refreshes auction listings after success.
 *
 * Workflow:
 *     1. Validate form fields
 *     2. Calculate auction duration
 *     3. Convert ETH to wei
 *     4. Submit auction request
 *     5. Display transaction results
 *     6. Refresh auction listings
 */
function setupCreateAuctionForm() {
    const form = document.getElementById("createAuctionForm");

    // Prevent duplicate event registration.
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

        // Display pending transaction status.
        status.textContent = "Creating auction...";
        status.style.color = "var(--accent, #00e5ff)";

        try {
            // Collect form values.
            const title = document.getElementById("auctionTitle")?.value.trim();
            const description = document.getElementById("auctionDescription")?.value.trim();
            const assetType = document.getElementById("assetType")?.value || "Other";
            const startingBidInput = document.getElementById("startingBid")?.value;
            const endDate = document.getElementById("endDate")?.value;

            // Validate required fields.
            if (!title || !description) {
                throw new Error("Missing title or description.");
            }

            // Convert selected end date into a Unix timestamp.
            const endTimestamp = Math.floor(
                new Date(endDate).getTime() / 1000
            );

            const now = Math.floor(Date.now() / 1000);

            // Ensure the auction expires in the future.
            if (!Number.isFinite(endTimestamp) || endTimestamp <= now) {
                throw new Error("Auction end date must be in the future.");
            }

            // Convert expiration date into auction duration.
            const biddingTimeSeconds = endTimestamp - now;

            // Convert ETH value into wei.
            const startingBidWei = ethToWeiString(startingBidInput);

            // Submit auction creation request.
            const data = await window.auctionApi.createAuction({
                bidding_time_seconds: biddingTimeSeconds,
                starting_bid_wei: startingBidWei,
                title,
                description: `[${assetType}] ${description}`,
            });

            // Display auction creation success message.
            status.textContent =
                `Auction created: ${data.auction_address}`;

            status.style.color = "var(--success, #32ff9a)";

            // Clear form after successful creation.
            form.reset();

            /*
             * Old file called loadAuctions(); that function did not exist.
             * Always refresh through the shared state coordinator.
             */

            // Refresh auction listings from the backend.
            await refreshAuctions({ showLoading: true });
        } catch (err) {
            console.error("Create auction failed:", err);

            // Display user-friendly error message.
            status.textContent = `❌ ${err.message}`;
            status.style.color = "var(--danger, #ff4d6d)";
        }
    });
}

/* -------------------------------------------------------------------------- */
/*                              Global Exports                                */
/* -------------------------------------------------------------------------- */

// Expose auction form initialization helper globally.
window.setupCreateAuctionForm = setupCreateAuctionForm;