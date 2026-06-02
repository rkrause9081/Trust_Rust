/*
 * bid_form.js
 *
 * Purpose:
 *     Bid submission form behavior and validation logic.
 *
 *     Handles bid form interactions, validates ETH bid amounts,
 *     converts values into wei, submits bid transactions through
 *     the API layer, and refreshes auction data after successful
 *     blockchain confirmation.
 *
 * Responsibilities:
 *     - Validate bid input
 *     - Convert ETH values into wei
 *     - Submit bid requests
 *     - Display bid status messages
 *     - Refresh auction data after bids
 *     - Prevent duplicate event registration
 *
 * Non-Responsibilities:
 *     - Auction rendering
 *     - API implementation
 *     - Authentication
 *     - Blockchain transaction execution
 *
 * Architecture:
 *
 *         User Input
 *              ↓
 *         bid_form.js
 *              ↓
 *        auction_api.js
 *              ↓
 *         Backend API
 *              ↓
 *      Blockchain Network
 */

/* -------------------------------------------------------------------------- */
/*                           ETH / Wei Conversion                             */
/* -------------------------------------------------------------------------- */

/**
 * Converts an ETH amount string into wei.
 *
 * Validates numeric formatting and supports
 * up to 18 decimal places.
 *
 * # Arguments
 *
 * * `ethValue` - ETH value entered by the user.
 *
 * # Returns
 *
 * String representation of the value in wei.
 *
 * # Errors
 *
 * Throws if:
 *     - Amount is empty
 *     - Amount is zero or negative
 *     - Amount format is invalid
 */
function ethToWeiString(ethValue) {
    const raw = String(ethValue || "").trim();

    // Ensure a positive amount was provided.
    if (!raw || Number(raw) <= 0) {
        throw new Error("Amount must be greater than 0");
    }

    // Validate ETH decimal formatting.
    if (!/^\d+(\.\d{1,18})?$/.test(raw)) {
        throw new Error("Invalid ETH amount");
    }

    const [whole, fraction = ""] = raw.split(".");
    const fractionPadded = fraction.padEnd(18, "0");

    // Convert ETH into wei using integer arithmetic.
    return (
        BigInt(whole || "0") * 10n ** 18n +
        BigInt(fractionPadded || "0")
    ).toString();
}

/* -------------------------------------------------------------------------- */
/*                              Bid Form Setup                                */
/* -------------------------------------------------------------------------- */

/**
 * Initializes the bid submission form.
 *
 * Registers the submit handler, validates user input,
 * submits bid transactions through the API layer, and
 * refreshes auction data after successful transactions.
 *
 * Workflow:
 *     1. Validate form fields
 *     2. Convert ETH to wei
 *     3. Submit bid request
 *     4. Display transaction results
 *     5. Refresh auction data
 */
function setupBidForm() {
    const form = document.getElementById("bidForm");

    // Prevent duplicate event registration.
    if (!form || form.dataset.bound === "true") {
        return;
    }

    form.dataset.bound = "true";

    form.addEventListener("submit", async (event) => {
        event.preventDefault();

        const statusEl = document.getElementById("bidStatus");
        const auctionAddress = document.getElementById("bidAuction")?.value;
        const bidAmountInput = document.getElementById("bidAmount")?.value;

        if (!statusEl) {
            return;
        }

        // Ensure required form fields are populated.
        if (!auctionAddress || !bidAmountInput) {
            statusEl.textContent =
                "Please select an auction and enter a bid amount.";
            statusEl.style.color = "var(--danger, #ff4d6d)";
            return;
        }

        let bidAmountWei;

        // Convert ETH amount into wei.
        try {
            bidAmountWei = ethToWeiString(bidAmountInput);
        } catch (err) {
            statusEl.textContent = `❌ ${err.message}`;
            statusEl.style.color = "var(--danger, #ff4d6d)";
            return;
        }

        // Display pending transaction status.
        statusEl.textContent = "Submitting bid on-chain...";
        statusEl.style.color = "var(--accent, #00e5ff)";

        try {
            // Submit bid transaction.
            const data = await window.auctionApi.placeBid(
                auctionAddress,
                bidAmountWei
            );

            statusEl.innerHTML = `
                ✅ Bid placed successfully!<br>
                <strong>TX:</strong> ${data.tx_hash || "pending"}<br>
                Refreshing auction cards...
            `;
            statusEl.style.color = "var(--success, #32ff9a)";

            /*
             * Important synchronization step:
             * After transaction confirmation, reload
             * auction data directly from the backend.
             */
            await refreshAuctions({ showLoading: false });

            // Clear form after successful submission.
            form.reset();

            statusEl.innerHTML = `
                ✅ Bid placed successfully!<br>
                <strong>TX:</strong> ${data.tx_hash || "pending"}
            `;
        } catch (err) {
            console.error("Bid submit error:", err);

            statusEl.textContent = `❌ ${err.message}`;
            statusEl.style.color = "var(--danger, #ff4d6d)";
        }
    });
}

/* -------------------------------------------------------------------------- */
/*                              Global Exports                                */
/* -------------------------------------------------------------------------- */

// Expose ETH conversion helper globally.
window.ethToWeiString = ethToWeiString;

// Expose bid form initialization helper globally.
window.setupBidForm = setupBidForm;