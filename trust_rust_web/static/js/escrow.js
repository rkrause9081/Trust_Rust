// ====================== ESCROW HELPERS ======================

async function escrowRequest(url, body = null) {
    const res = await fetch(url, {
        method: body ? "POST" : "GET",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
        body: body ? JSON.stringify(body) : undefined,
    });

    const text = await res.text();

    let data = {};

    try {
        data = text ? JSON.parse(text) : {};
    } catch {
        throw new Error(text || "Invalid server response");
    }

    if (!res.ok || data.success === false) {
        throw new Error(
            data.message ||
            data.error ||
            "Escrow request failed"
        );
    }

    return data;
}

// ====================== STATUS ======================

async function loadEscrowStatus(auctionAddress) {
    try {
        return await escrowRequest(
            `/api/escrow/status/${auctionAddress}`
        );
    } catch (err) {
        console.error("Escrow status error:", err);
        return null;
    }
}

// ====================== ACTIONS ======================

async function endAuction(auctionAddress) {
    try {
        const result = await escrowRequest(
            "/api/escrow/end",
            {
                auction_address: auctionAddress,
            }
        );

        alert(`Auction finalized.\nTx: ${result.tx_hash}`);

        if (typeof loadActiveAuctions === "function") {
            await loadActiveAuctions();
        }
    } catch (err) {
        alert(`Failed to finalize auction:\n${err.message}`);
    }
}

async function confirmReceipt(auctionAddress) {
    try {
        const result = await escrowRequest(
            "/api/escrow/confirm",
            {
                auction_address: auctionAddress,
            }
        );

        alert(`Receipt confirmed.\nTx: ${result.tx_hash}`);

        if (typeof loadActiveAuctions === "function") {
            await loadActiveAuctions();
        }
    } catch (err) {
        alert(`Failed to confirm receipt:\n${err.message}`);
    }
}

async function claimTimeout(auctionAddress) {
    try {
        const result = await escrowRequest(
            "/api/escrow/claim-timeout",
            {
                auction_address: auctionAddress,
            }
        );

        alert(`Timeout claim successful.\nTx: ${result.tx_hash}`);

        if (typeof loadActiveAuctions === "function") {
            await loadActiveAuctions();
        }
    } catch (err) {
        alert(`Failed to claim timeout:\n${err.message}`);
    }
}

async function flagRefund(auctionAddress) {
    try {
        const result = await escrowRequest(
            "/api/escrow/refund",
            {
                auction_address: auctionAddress,
            }
        );

        alert(`Refund flagged.\nTx: ${result.tx_hash}`);

        if (typeof loadActiveAuctions === "function") {
            await loadActiveAuctions();
        }
    } catch (err) {
        alert(`Failed to flag refund:\n${err.message}`);
    }
}

// ====================== UI HELPERS ======================

function formatEscrowStatusLabel(status, isActive) {
    if (status === "ActiveAuction" && isActive) {
        return "Bidding Active";
    }

    if (status === "ActiveAuction" && !isActive) {
        return "Ready To Finalize";
    }

    if (status === "AwaitingFinalization") {
        return "Ready To Finalize";
    }

    if (status === "AwaitingBuyerConfirmation") {
        return "Awaiting Buyer Confirmation";
    }

    if (status === "Complete") {
        return "Complete";
    }

    if (status === "Refunded") {
        return "Refunded";
    }

    return status || "Unknown";
}

function buildEscrowPanel(auctionAddress, escrow, isActive) {
    if (!escrow) {
        return `
            <div class="escrow-panel">
                <div class="escrow-header">Settlement</div>
                <div class="escrow-complete">
                    Failed to load escrow status
                </div>
            </div>
        `;
    }

    const status = escrow.status || "Unknown";

    const timeRemaining = Number(
        escrow.time_remaining_seconds || 0
    );

    const hours = Math.floor(timeRemaining / 3600);
    const minutes = Math.floor((timeRemaining % 3600) / 60);

    let buttons = "";

    if (
        (status === "ActiveAuction" && !isActive) ||
        status === "AwaitingFinalization"
    ) {
        buttons += `
            <button
                class="btn-primary"
                type="button"
                onclick="endAuction('${auctionAddress}')"
            >
                End Auction
            </button>
        `;
    }

    if (escrow.can_confirm_receipt) {
        buttons += `
            <button
                class="btn-primary"
                type="button"
                onclick="confirmReceipt('${auctionAddress}')"
            >
                Confirm Receipt
            </button>
        `;
    }

    if (escrow.can_claim_timeout) {
        buttons += `
            <button
                class="btn-primary"
                type="button"
                onclick="claimTimeout('${auctionAddress}')"
            >
                Claim Timeout
            </button>
        `;
    }

    if (escrow.can_flag_refund) {
        buttons += `
            <button
                class="btn-ghost"
                type="button"
                onclick="flagRefund('${auctionAddress}')"
            >
                Flag Refund
            </button>
        `;
    }

    return `
        <div class="escrow-panel">
            <div class="escrow-header">
                Settlement
            </div>

            <div class="escrow-status">
                Status:
                <span class="accent">
                    ${formatEscrowStatusLabel(status, isActive)}
                </span>
            </div>

            ${
                timeRemaining > 0
                    ? `
                        <div class="escrow-time">
                            Buyer confirmation window:
                            ${hours}h ${minutes}m
                        </div>
                    `
                    : ""
            }

            ${
                buttons
                    ? `
                        <div class="escrow-actions">
                            ${buttons}
                        </div>
                    `
                    : `
                        <div class="escrow-complete">
                            No escrow actions available
                        </div>
                    `
            }
        </div>
    `;
}

// expose globally
window.loadEscrowStatus = loadEscrowStatus;
window.buildEscrowPanel = buildEscrowPanel;
window.endAuction = endAuction;
window.confirmReceipt = confirmReceipt;
window.claimTimeout = claimTimeout;
window.flagRefund = flagRefund;