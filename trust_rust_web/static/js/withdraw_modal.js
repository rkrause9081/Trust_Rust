/*
 * withdraw_modal.js
 *
 * Withdraw modal behavior only.
 */

let selectedWithdrawAuction = null;

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

async function confirmWithdraw() {
    const statusEl = document.getElementById("withdrawStatus");

    if (!statusEl) {
        return;
    }

    if (!selectedWithdrawAuction) {
        statusEl.textContent = "No auction selected.";
        statusEl.style.color = "var(--danger, #ff4d6d)";
        return;
    }

    statusEl.textContent = "Withdrawing funds on-chain...";
    statusEl.style.color = "var(--accent, #00e5ff)";

    try {
        const data = await window.auctionApi.withdraw(selectedWithdrawAuction);

        statusEl.innerHTML = `
            ✅ Withdrawal successful<br>
            ${
                data.amount_withdrawn_eth
                    ? `<strong>Amount:</strong> ${data.amount_withdrawn_eth} ETH<br>`
                    : ""
            }
            <strong>TX:</strong> ${data.tx_hash || "pending"}
        `;
        statusEl.style.color = "var(--success, #32ff9a)";

        await refreshAuctions({ showLoading: false });

        setTimeout(closeWithdrawModal, 1800);
    } catch (err) {
        console.error("Withdraw failed:", err);
        statusEl.textContent = `❌ ${err.message}`;
        statusEl.style.color = "var(--danger, #ff4d6d)";
    }
}

/*
 * Backward-compatible helper for any old buttons still calling handleWithdraw().
 */
async function handleWithdraw(auctionAddress) {
    openWithdrawModal(auctionAddress);
}

window.openWithdrawModal = openWithdrawModal;
window.closeWithdrawModal = closeWithdrawModal;
window.confirmWithdraw = confirmWithdraw;
window.handleWithdraw = handleWithdraw;
