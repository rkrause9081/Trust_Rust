/*
 * bidding.js
 *
 * Purpose:
 *     Handles frontend bid submission and withdrawal actions.
 *
 * Responsibilities:
 *     - Convert ETH input values into wei strings
 *     - Parse backend JSON responses
 *     - Submit bid requests to the backend API
 *     - Submit withdrawal requests to the backend API
 *     - Update UI status messages
 *     - Refresh auction data after successful actions
 *
 * Non-Responsibilities:
 *     - Wallet authentication
 *     - Auction card rendering
 *     - Backend validation
 *     - Blockchain transaction execution
 *
 * Architecture:
 *
 *      Bid / Withdraw UI
 *              ↓
 *          bidding.js
 *              ↓
 *        Backend API Routes
 *              ↓
 *      trust_rust_client
 */

/* -------------------------------------------------------------------------- */
/*                                Bid Helpers                                 */
/* -------------------------------------------------------------------------- */

/**
 * Converts a user-entered ETH amount into a wei-denominated string.
 *
 * Supports up to 18 decimal places.
 *
 * @param {string|number} ethValue
 * @returns {string}
 * @throws {Error} If the amount is empty, invalid, or not greater than zero.
 */
function ethToWeiString(ethValue) {
    const raw = String(ethValue || "").trim();

    if (!raw || Number(raw) <= 0) {
        throw new Error("Amount must be greater than 0");
    }

    if (!/^\d+(\.\d{1,18})?$/.test(raw)) {
        throw new Error("Invalid ETH amount");
    }

    const [whole, fraction = ""] = raw.split(".");

    const fractionPadded = fraction.padEnd(18, "0");

    return (
        BigInt(whole || "0") * 10n ** 18n +
        BigInt(fractionPadded || "0")
    ).toString();
}

/* -------------------------------------------------------------------------- */
/*                             Bid Form Handling                              */
/* -------------------------------------------------------------------------- */

/**
 * Initializes the bid form submit handler.
 *
 * This handler:
 *     - Reads selected auction and ETH bid amount
 *     - Converts ETH to wei
 *     - Sends the bid request to the backend
 *     - Displays success or error status
 *     - Refreshes active auctions after a successful bid
 */
function setupBidForm() {
    const form = document.getElementById("bidForm");

    if (!form) {
        return;
    }

    form.addEventListener("submit", async (e) => {
        e.preventDefault();

        const statusEl =
            document.getElementById("bidStatus");

        const auctionAddress =
            document.getElementById("bidAuction")?.value;

        const bidAmountInput =
            document.getElementById("bidAmount")?.value;

        if (!auctionAddress || !bidAmountInput) {
            statusEl.textContent =
                "Please select an auction and enter a bid amount.";

            statusEl.style.color =
                "#ff4d6d";

            return;
        }

        let bidAmountWei;

        try {
            bidAmountWei =
                ethToWeiString(bidAmountInput);
        } catch (err) {
            statusEl.textContent =
                "❌ " + err.message;

            statusEl.style.color =
                "#ff4d6d";

            return;
        }

        statusEl.textContent =
            "Submitting bid on-chain...";

        statusEl.style.color =
            "#00e5ff";

        try {
            const res = await fetch("/api/bid", {
                method: "POST",

                headers: {
                    "Content-Type": "application/json",
                },

                credentials: "include",

                body: JSON.stringify({
                    auction_address: auctionAddress,
                    bid_amount_wei: bidAmountWei,
                }),
            });

            const data =
                await parseJsonResponse(res);

            statusEl.innerHTML = `
                ✅ Bid placed successfully!<br>
                <strong>TX:</strong> ${data.tx_hash || "pending"}
            `;

            statusEl.style.color =
                "#32ff9a";

            form.reset();

            if (
                typeof loadActiveAuctions ===
                "function"
            ) {
                await loadActiveAuctions();
            }
        } catch (err) {
            console.error(
                "Bid submit error:",
                err
            );

            statusEl.textContent =
                "❌ " + err.message;

            statusEl.style.color =
                "#ff4d6d";
        }
    });
}

/* -------------------------------------------------------------------------- */
/*                             Withdraw Handling                              */
/* -------------------------------------------------------------------------- */

/**
 * Submits a withdrawal request for a selected auction.
 *
 * @param {string} auctionAddress
 * @returns {Promise<void>}
 */
async function handleWithdraw(auctionAddress) {
    try {
        const res = await fetch("/api/withdraw", {
            method: "POST",

            headers: {
                "Content-Type": "application/json",
            },

            credentials: "include",

            body: JSON.stringify({
                auction_address: auctionAddress,
            }),
        });

        const data =
            await parseJsonResponse(res);

        alert(
            `Withdraw successful!\nTX: ${data.tx_hash || "pending"}`
        );

        if (
            typeof loadActiveAuctions ===
            "function"
        ) {
            await loadActiveAuctions();
        }
    } catch (err) {
        console.error(
            "Withdraw error:",
            err
        );

        alert(
            "Withdraw failed: " +
            err.message
        );
    }
}