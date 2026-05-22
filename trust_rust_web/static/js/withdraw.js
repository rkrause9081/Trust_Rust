/*
 * withdraw.js
 *
 * Purpose:
 *     Handles frontend withdrawal modal state and withdrawal confirmation.
 *
 * Responsibilities:
 *     - Open and close the withdrawal modal
 *     - Track the selected auction for withdrawal
 *     - Submit withdrawal requests to the backend API
 *     - Display withdrawal status and transaction metadata
 *     - Refresh auction data after successful withdrawals
 *
 * Non-Responsibilities:
 *     - Wallet authentication
 *     - Blockchain transaction execution
 *     - Backend withdrawal validation
 *     - Auction card rendering
 *
 * Architecture:
 *
 *      Withdraw Modal
 *             ↓
 *        withdraw.js
 *             ↓
 *      Withdraw API Route
 *             ↓
 *      Auction Contract
 */

/* -------------------------------------------------------------------------- */
/*                              Modal State                                   */
/* -------------------------------------------------------------------------- */

/**
 * Currently selected auction address for withdrawal.
 *
 * @type {string|null}
 */
let selectedWithdrawAuction = null;

/* -------------------------------------------------------------------------- */
/*                              Modal Controls                                */
/* -------------------------------------------------------------------------- */

/**
 * Opens the withdrawal modal for a selected auction.
 *
 * @param {string} auctionAddress
 */
function openWithdrawModal(auctionAddress) {
    selectedWithdrawAuction = auctionAddress;

    const modal = document.getElementById("withdrawModal");
    const addressEl = document.getElementById("withdrawAuctionAddress");
    const statusEl = document.getElementById("withdrawStatus");

    if (!modal || !addressEl || !statusEl) {
        return;
    }

    addressEl.textContent = auctionAddress;
    statusEl.textContent = "";
    statusEl.style.color = "var(--muted)";

    modal.classList.add("show");
    modal.setAttribute("aria-hidden", "false");
}

/**
 * Closes the withdrawal modal and clears local modal state.
 */
function closeWithdrawModal() {
    const modal = document.getElementById("withdrawModal");
    const statusEl = document.getElementById("withdrawStatus");

    selectedWithdrawAuction = null;

    if (statusEl) {
        statusEl.textContent = "";
        statusEl.style.color = "var(--muted)";
    }

    if (modal) {
        modal.classList.remove("show");
        modal.setAttribute("aria-hidden", "true");
    }
}

/* -------------------------------------------------------------------------- */
/*                          Withdrawal Confirmation                           */
/* -------------------------------------------------------------------------- */

/**
 * Confirms and submits the selected withdrawal request.
 *
 * @returns {Promise<void>}
 */
async function confirmWithdraw() {
    const statusEl = document.getElementById("withdrawStatus");

    if (!selectedWithdrawAuction) {
        statusEl.textContent = "No auction selected.";
        statusEl.style.color = "var(--danger)";
        return;
    }

    statusEl.textContent = "Withdrawing funds on-chain...";
    statusEl.style.color = "var(--accent)";

    try {
        const res = await fetch("/api/withdraw", {
            method: "POST",

            headers: {
                "Content-Type": "application/json",
            },

            credentials: "include",

            body: JSON.stringify({
                auction_address: selectedWithdrawAuction,
            }),
        });

        const data = await parseJsonResponse(res);

        statusEl.innerHTML = `
            ✅ Withdrawal successful<br>
            <strong>Amount:</strong> ${data.amount_withdrawn_eth} ETH<br>
            <strong>TX:</strong> ${data.tx_hash}
        `;
        statusEl.style.color = "var(--success)";

        if (typeof loadActiveAuctions === "function") {
            await loadActiveAuctions();
        }

        setTimeout(closeWithdrawModal, 1800);
    } catch (err) {
        console.error("Withdraw failed:", err);
        statusEl.textContent = "❌ " + err.message;
        statusEl.style.color = "var(--danger)";
    }
}

/* -------------------------------------------------------------------------- */
/*                              Global Exports                                */
/* -------------------------------------------------------------------------- */

window.openWithdrawModal = openWithdrawModal;
window.closeWithdrawModal = closeWithdrawModal;
window.confirmWithdraw = confirmWithdraw;