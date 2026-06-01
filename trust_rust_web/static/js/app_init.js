/*
 * app_init.js
 *
 * Frontend bootstrap only.
 */

async function initializeApp() {
    try {
        console.log("Initializing T.R.U.S.T frontend...");

        restoreWalletUiFromStorage?.();

        const loginBtn = document.getElementById("loginBtn");

        if (loginBtn && loginBtn.dataset.bound !== "true") {
            loginBtn.dataset.bound = "true";
            loginBtn.addEventListener("click", async () => {
                try {
                    await siweLogin();
                } catch (err) {
                    console.error("Login initialization failed:", err);
                }
            });
        }

        setupBidForm?.();
        setupCreateAuctionForm?.();

        /*
         * Keep this enabled if you want cards visible before login.
         * Set to false if you want to avoid any blockchain calls until SIWE completes.
         */
        const LOAD_AUCTIONS_ON_STARTUP = true;

        if (LOAD_AUCTIONS_ON_STARTUP && typeof refreshAuctions === "function") {
            await refreshAuctions({ showLoading: true });
        }

        console.log("Frontend initialized successfully.");
    } catch (err) {
        console.error("Application initialization failed:", err);

        const grid = document.getElementById("auctionGrid");

        if (grid) {
            grid.innerHTML = `
                <div class="card">
                    <p style="color:#ff4d6d; margin:0;">
                        Error: ${err.message}
                    </p>
                </div>
            `;
        }
    }
}

document.addEventListener("DOMContentLoaded", initializeApp);
