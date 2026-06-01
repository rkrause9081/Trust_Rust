/*
 * bid_form.js
 *
 * Bid form behavior only.
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

function setupBidForm() {
    const form = document.getElementById("bidForm");

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

        if (!auctionAddress || !bidAmountInput) {
            statusEl.textContent = "Please select an auction and enter a bid amount.";
            statusEl.style.color = "var(--danger, #ff4d6d)";
            return;
        }

        let bidAmountWei;

        try {
            bidAmountWei = ethToWeiString(bidAmountInput);
        } catch (err) {
            statusEl.textContent = `❌ ${err.message}`;
            statusEl.style.color = "var(--danger, #ff4d6d)";
            return;
        }

        statusEl.textContent = "Submitting bid on-chain...";
        statusEl.style.color = "var(--accent, #00e5ff)";

        try {
            const data = await window.auctionApi.placeBid(auctionAddress, bidAmountWei);

            statusEl.innerHTML = `
                ✅ Bid placed successfully!<br>
                <strong>TX:</strong> ${data.tx_hash || "pending"}<br>
                Refreshing auction cards...
            `;
            statusEl.style.color = "var(--success, #32ff9a)";

            /*
             * This is the important bug fix:
             * after the on-chain tx completes, always reload from /api/auctions.
             */
            await refreshAuctions({ showLoading: false });

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

window.ethToWeiString = ethToWeiString;
window.setupBidForm = setupBidForm;
