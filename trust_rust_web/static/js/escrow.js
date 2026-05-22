/*
 * escrow.js
 *
 * Purpose:
 *     Handles frontend escrow lifecycle interactions and
 *     escrow status rendering for auction settlement flows.
 *
 * Responsibilities:
 *     - Fetch escrow status from backend APIs
 *     - Render escrow action panels
 *     - Execute escrow-related API actions
 *     - Refresh auction UI after escrow updates
 *     - Display settlement status information
 *
 * Non-Responsibilities:
 *     - Wallet authentication
 *     - Smart contract execution
 *     - Backend authorization
 *     - Auction rendering logic
 *
 * Architecture:
 *
 *      Auction UI
 *           ↓
 *        escrow.js
 *           ↓
 *      Escrow API Routes
 *           ↓
 *      Blockchain Escrow Logic
 */

/* -------------------------------------------------------------------------- */
/*                              Status Helpers                                */
/* -------------------------------------------------------------------------- */

/**
 * Returns a frontend color associated with an escrow status.
 *
 * @param {string} status
 * @returns {string}
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

/**
 * Formats remaining confirmation seconds into a readable string.
 *
 * @param {number} seconds
 * @returns {string}
 */
function formatRemainingTime(seconds) {
    const value = Number(seconds || 0);

    if (value <= 0) {
        return "Expired";
    }

    const days =
        Math.floor(value / 86400);

    const hours =
        Math.floor((value % 86400) / 3600);

    const minutes =
        Math.floor((value % 3600) / 60);

    if (days > 0) {
        return `${days}d ${hours}h`;
    }

    if (hours > 0) {
        return `${hours}h ${minutes}m`;
    }

    return `${minutes}m`;
}

/* -------------------------------------------------------------------------- */
/*                           Escrow Status Loading                            */
/* -------------------------------------------------------------------------- */

/**
 * Loads escrow status information for an auction.
 *
 * @param {string} auctionAddress
 * @returns {Promise<Object|null>}
 */
async function loadEscrowStatus(
    auctionAddress
) {
    try {
        const res = await fetch(
            `/api/escrow/status/${auctionAddress}`,
            {
                method: "GET",
                credentials: "include",
            }
        );

        return await parseJsonResponse(res);
    } catch (err) {
        console.error(
            "Failed to load escrow status:",
            err
        );

        return null;
    }
}

/* -------------------------------------------------------------------------- */
/*                           Escrow API Actions                               */
/* -------------------------------------------------------------------------- */

/**
 * Executes an escrow API action.
 *
 * @param {string} endpoint
 * @param {string} auctionAddress
 * @returns {Promise<Object>}
 */
async function executeEscrowAction(
    endpoint,
    auctionAddress
) {
    const res = await fetch(endpoint, {
        method: "POST",

        headers: {
            "Content-Type":
                "application/json",
        },

        credentials: "include",

        body: JSON.stringify({
            auction_address:
                auctionAddress,
        }),
    });

    return await parseJsonResponse(res);
}

/**
 * Ends an auction after the bidding period expires.
 *
 * @param {string} auctionAddress
 * @returns {Promise<void>}
 */
async function handleEndAuction(
    auctionAddress
) {
    try {
        const data =
            await executeEscrowAction(
                "/api/escrow/end",
                auctionAddress
            );

        alert(
            `Auction ended!\nTX: ${data.tx_hash || "pending"}`
        );

        if (
            typeof loadActiveAuctions ===
            "function"
        ) {
            await loadActiveAuctions();
        }
    } catch (err) {
        console.error(
            "End auction error:",
            err
        );

        alert(
            "Failed to end auction: " +
            err.message
        );
    }
}

/**
 * Confirms receipt of the auctioned item.
 *
 * @param {string} auctionAddress
 * @returns {Promise<void>}
 */
async function handleConfirmReceipt(
    auctionAddress
) {
    try {
        const data =
            await executeEscrowAction(
                "/api/escrow/confirm",
                auctionAddress
            );

        alert(
            `Receipt confirmed!\nTX: ${data.tx_hash || "pending"}`
        );

        if (
            typeof loadActiveAuctions ===
            "function"
        ) {
            await loadActiveAuctions();
        }
    } catch (err) {
        console.error(
            "Confirm receipt error:",
            err
        );

        alert(
            "Failed to confirm receipt: " +
            err.message
        );
    }
}

/**
 * Claims escrow settlement after confirmation timeout expiration.
 *
 * @param {string} auctionAddress
 * @returns {Promise<void>}
 */
async function handleClaimTimeout(
    auctionAddress
) {
    try {
        const data =
            await executeEscrowAction(
                "/api/escrow/claim-timeout",
                auctionAddress
            );

        alert(
            `Timeout claimed!\nTX: ${data.tx_hash || "pending"}`
        );

        if (
            typeof loadActiveAuctions ===
            "function"
        ) {
            await loadActiveAuctions();
        }
    } catch (err) {
        console.error(
            "Claim timeout error:",
            err
        );

        alert(
            "Failed to claim timeout: " +
            err.message
        );
    }
}

/**
 * Flags an escrow refund action.
 *
 * @param {string} auctionAddress
 * @returns {Promise<void>}
 */
async function handleRefund(
    auctionAddress
) {
    try {
        const data =
            await executeEscrowAction(
                "/api/escrow/refund",
                auctionAddress
            );

        alert(
            `Refund flagged!\nTX: ${data.tx_hash || "pending"}`
        );

        if (
            typeof loadActiveAuctions ===
            "function"
        ) {
            await loadActiveAuctions();
        }
    } catch (err) {
        console.error(
            "Refund error:",
            err
        );

        alert(
            "Failed to flag refund: " +
            err.message
        );
    }
}

/* -------------------------------------------------------------------------- */
/*                           Escrow Panel Rendering                           */
/* -------------------------------------------------------------------------- */

/**
 * Builds the escrow status/action panel HTML for an auction card.
 *
 * @param {string} auctionAddress
 * @param {Object|null} escrow
 * @returns {string}
 */
function buildEscrowPanel(
    auctionAddress,
    escrow
) {
    if (!escrow) {
        return `
            <div class="escrow-panel">
                <div class="escrow-status-line">
                    Escrow status unavailable
                </div>
            </div>
        `;
    }

    const status =
        escrow.status || "Unknown";

    const color =
        escrowStatusColor(status);

    const remaining =
        formatRemainingTime(
            escrow.time_remaining_seconds
        );

    let actions = "";

    if (escrow.can_confirm_receipt) {
        actions += `
            <button
                class="btn-primary"
                onclick="handleConfirmReceipt('${auctionAddress}')"
            >
                Confirm Receipt
            </button>
        `;
    }

    if (escrow.can_claim_timeout) {
        actions += `
            <button
                class="btn-secondary"
                onclick="handleClaimTimeout('${auctionAddress}')"
            >
                Claim Timeout
            </button>
        `;
    }

    if (escrow.can_flag_refund) {
        actions += `
            <button
                class="btn-danger"
                onclick="handleRefund('${auctionAddress}')"
            >
                Flag Refund
            </button>
        `;
    }

    if (
        status ===
        "AwaitingFinalization"
    ) {
        actions += `
            <button
                class="btn-secondary"
                onclick="handleEndAuction('${auctionAddress}')"
            >
                End Auction
            </button>
        `;
    }

    return `
        <div class="escrow-panel">

            <div class="escrow-status-line">
                Escrow Status:
                <span style="color:${color};">
                    ${status}
                </span>
            </div>

            <div class="escrow-status-line">
                Confirmation Window:
                ${remaining}
            </div>

            ${
                actions
                    ? `
                        <div class="escrow-actions">
                            ${actions}
                        </div>
                    `
                    : ""
            }

        </div>
    `;
}