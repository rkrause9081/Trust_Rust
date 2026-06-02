/*
 * escrow_actions.js
 *
 * Escrow status rendering and action handlers.
 */

function escrowStatusColor(status) {
    switch (status) {
        case "Complete":
            return "#32ff9a";
        case "Refunded":
            return "#ffb84d";
        case "AwaitingBuyerConfirmation":
            return "#00e5ff";
        case "AwaitingFinalization":
            return "#b388ff";
        case "ActiveAuction":
            return "#6b7c8f";
        default:
            return "#6b7c8f";
    }
}

function formatRemainingTime(seconds) {
    const value = Number(seconds || 0);

    if (value <= 0) {
        return "Expired";
    }

    const days = Math.floor(value / 86400);
    const hours = Math.floor((value % 86400) / 3600);
    const minutes = Math.floor((value % 3600) / 60);

    if (days > 0) {
        return `${days}d ${hours}h`;
    }

    if (hours > 0) {
        return `${hours}h ${minutes}m`;
    }

    return `${minutes}m`;
}

async function loadEscrowStatus(auctionAddress) {
    try {
        return await window.auctionApi.getEscrowStatus(auctionAddress);
    } catch (err) {
        console.error("Failed to load escrow status:", err);
        return null;
    }
}

async function runEscrowAction(endpoint, auctionAddress, successLabel) {
    try {
        const data = await window.auctionApi.escrowAction(endpoint, auctionAddress);

        alert(`${successLabel}\nTX: ${data.tx_hash || "pending"}`);

        await refreshAuctions({ showLoading: false });
    } catch (err) {
        console.error(`${successLabel} failed:`, err);
        alert(`${successLabel} failed: ${err.message}`);
    }
}

function handleEndAuction(auctionAddress) {
    return runEscrowAction("/api/escrow/end", auctionAddress, "Auction ended");
}

function handleConfirmReceipt(auctionAddress) {
    return runEscrowAction("/api/escrow/confirm", auctionAddress, "Receipt confirmed");
}

function handleClaimTimeout(auctionAddress) {
    return runEscrowAction("/api/escrow/claim-timeout", auctionAddress, "Timeout claimed");
}

function handleRefund(auctionAddress) {
    return runEscrowAction("/api/escrow/refund", auctionAddress, "Refund flagged");
}

function buildEscrowPanel(auctionAddress, escrow) {
    if (!escrow) {
        return `
            <div class="escrow-panel">
                <div class="escrow-status-line">
                    Escrow status unavailable
                </div>
            </div>
        `;
    }

    const status = escrow.status || "Unknown";
    const color = escrowStatusColor(status);
    const remaining = formatRemainingTime(escrow.time_remaining_seconds);

    let actions = "";

    if (escrow.can_confirm_receipt) {
        actions += `
            <button class="btn-primary" onclick="handleConfirmReceipt('${auctionAddress}')">
                Confirm Receipt
            </button>
        `;
    }

    if (escrow.can_claim_timeout) {
        actions += `
            <button class="btn-secondary" onclick="handleClaimTimeout('${auctionAddress}')">
                Claim Timeout
            </button>
        `;
    }

    if (escrow.can_flag_refund) {
        actions += `
            <button class="btn-danger" onclick="handleRefund('${auctionAddress}')">
                Flag Refund
            </button>
        `;
    }

    if (status === "AwaitingFinalization") {
        actions += `
            <button class="btn-secondary" onclick="handleEndAuction('${auctionAddress}')">
                End Auction
            </button>
        `;
    }

    return `
        <div class="escrow-panel">
            <div class="escrow-status-line">
                Escrow Status:
                <span style="color:${color};">${status}</span>
            </div>

            <div class="escrow-status-line">
                Confirmation Window:
                ${remaining}
            </div>

            ${
                actions
                    ? `<div class="escrow-actions">${actions}</div>`
                    : ""
            }
        </div>
    `;
}

window.loadEscrowStatus = loadEscrowStatus;
window.buildEscrowPanel = buildEscrowPanel;
window.handleEndAuction = handleEndAuction;
window.handleConfirmReceipt = handleConfirmReceipt;
window.handleClaimTimeout = handleClaimTimeout;
window.handleRefund = handleRefund;
