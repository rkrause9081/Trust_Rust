/*
 * withdraw_modal.js
 *
 * Purpose:
 *     Withdrawal modal UI behavior and withdrawal workflow.
 *
 *     Manages the withdrawal confirmation modal, executes
 *     escrow withdrawal requests, displays transaction
 *     results, and refreshes auction data after successful
 *     withdrawals.
 *
 * Responsibilities:
 *     - Open withdrawal modal
 *     - Close withdrawal modal
 *     - Track selected auction
 *     - Execute withdrawal requests
 *     - Display withdrawal status messages
 *     - Refresh auction data after withdrawals
 *
 * Non-Responsibilities:
 *     - Withdrawal implementation
 *     - Auction rendering
 *     - Authentication
 *     - Backend API implementation
 *
 * Architecture:
 *
 *         User Action
 *              ↓
 *       withdraw_modal.js
 *              ↓
 *        auction_api.js
 *              ↓
 *        Backend API
 *              ↓
 *       Escrow Contract
 */

/* -------------------------------------------------------------------------- */
/*                               Modal State                                  */
/* -------------------------------------------------------------------------- */

/**
 * Tracks the currently selected auction
 * for withdrawal operations.
 */
let selectedWithdrawAuction = null;

/* -------------------------------------------------------------------------- */
/*                           Modal Open / Close Logic                         */
/* -------------------------------------------------------------------------- */

/**
 * Opens the withdrawal confirmation modal.
 *
 * Displays the selected auction address and
 * prepares the withdrawal status area.
 *
 * # Arguments
 *
 * * `auctionAddress` - Auction contract address.
 */
function openWithdrawModal(auctionAddress) {
    // Store the selected auction.
    selectedWithdrawAuction = auctionAddress;

    const modal = document.getElementById("withdrawModal");
    const addressEl = document.getElementById("withdrawAuctionAddress");
    const statusEl = document.getElementById("withdrawStatus");

    // Ensure required UI elements exist.
    if (!modal || !addressEl || !statusEl) {
        return;
    }

    // Display selected auction information.
    addressEl.textContent = auctionAddress;

    // Reset status display.
    statusEl.textContent = "";
    statusEl.style.color = "var(--muted)";

    // Show the modal.
    modal.classList.add("show");
    modal.setAttribute("aria-hidden", "false");
}

/**
 * Closes the withdrawal modal and resets
 * temporary UI state.
 */
function closeWithdrawModal() {
    const modal = document.getElementById("withdrawModal");
    const statusEl = document.getElementById("withdrawStatus");

    // Clear selected auction.
    selectedWithdrawAuction = null;

    // Reset status display.
    if (statusEl) {
        statusEl.textContent = "";
        statusEl.style.color = "var(--muted)";
    }

    // Hide modal.
    if (modal) {
        modal.classList.remove("show");
        modal.setAttribute("aria-hidden", "true");
    }
}

/* -------------------------------------------------------------------------- */
/*                           Withdrawal Execution                             */
/* -------------------------------------------------------------------------- */

/**
 * Executes a withdrawal transaction.
 *
 * Validates the selected auction, submits the
 * withdrawal request through the API layer,
 * displays transaction results, refreshes
 * auction data, and closes the modal after
 * successful completion.
 *
 * # Errors
 *
 * Displays an error message if:
 *     - No auction is selected
 *     - Withdrawal fails
 *     - API request fails
 */
async function confirmWithdraw() {
    const statusEl = document.getElementById("withdrawStatus");

    if (!statusEl) {
        return;
    }

    // Ensure an auction has been selected.
    if (!selectedWithdrawAuction) {
        statusEl.textContent = "No auction selected.";
        statusEl.style.color = "var(--danger, #ff4d6d)";
        return;
    }

    // Display pending withdrawal status.
    statusEl.textContent = "Withdrawing funds on-chain...";
    statusEl.style.color = "var(--accent, #00e5ff)";

    try {
        // Submit withdrawal request.
        const data = await window.auctionApi.withdraw(
            selectedWithdrawAuction
        );

        // Display withdrawal results.
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

        // Refresh auction data after blockchain state changes.
        await refreshAuctions({ showLoading: false });

        // Close the modal after a short delay.
        setTimeout(closeWithdrawModal, 1800);
    } catch (err) {
        console.error("Withdraw failed:", err);

        // Display user-friendly error message.
        statusEl.textContent = `❌ ${err.message}`;
        statusEl.style.color = "var(--danger, #ff4d6d)";
    }
}

/* -------------------------------------------------------------------------- */
/*                       Backward Compatibility Helpers                       */
/* -------------------------------------------------------------------------- */

/*
 * Backward-compatible helper for any legacy buttons
 * still calling handleWithdraw().
 */

/**
 * Opens the withdrawal modal for the supplied auction.
 *
 * Maintained for compatibility with older UI code.
 */
async function handleWithdraw(auctionAddress) {
    openWithdrawModal(auctionAddress);
}

/* -------------------------------------------------------------------------- */
/*                              Global Exports                                */
/* -------------------------------------------------------------------------- */

// Expose withdrawal modal helpers globally.
window.openWithdrawModal = openWithdrawModal;
window.closeWithdrawModal = closeWithdrawModal;
window.confirmWithdraw = confirmWithdraw;
window.handleWithdraw = handleWithdraw;