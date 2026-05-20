let selectedWithdrawAuction = null;

function openWithdrawModal(auctionAddress) {
    selectedWithdrawAuction = auctionAddress;

    const modal = document.getElementById("withdrawModal");
    const addressEl = document.getElementById("withdrawAuctionAddress");
    const statusEl = document.getElementById("withdrawStatus");

    if (!modal || !addressEl || !statusEl) return;

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

        const text = await res.text();
        let data = {};

        try {
            data = text ? JSON.parse(text) : {};
        } catch {
            throw new Error(text || "Invalid server response");
        }

        if (!res.ok || data.success === false) {
            throw new Error(data.message || data.error || text || `Server error ${res.status}`);
        }

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

window.openWithdrawModal = openWithdrawModal;
window.closeWithdrawModal = closeWithdrawModal;
window.confirmWithdraw = confirmWithdraw;