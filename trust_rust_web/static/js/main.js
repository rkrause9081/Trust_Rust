/*
 * main.js
 *
 * Purpose:
 *     Frontend application bootstrap and global UI initialization.
 *
 * Responsibilities:
 *     - Initialize frontend event handlers
 *     - Connect authentication flows
 *     - Initialize auction loading
 *     - Register startup lifecycle hooks
 *     - Coordinate frontend modules
 *
 * Non-Responsibilities:
 *     - Blockchain interaction
 *     - SIWE backend verification
 *     - Auction rendering internals
 *     - Escrow transaction execution
 *
 * Architecture:
 *
 *        Browser Load
 *              ↓
 *           main.js
 *              ↓
 *      Frontend Modules
 *              ↓
 *      Backend API Routes
 */

/* -------------------------------------------------------------------------- */
/*                           Frontend Bootstrap                               */
/* -------------------------------------------------------------------------- */

/**
 * Initializes the frontend application.
 *
 * This function:
 *     - Registers login button handlers
 *     - Initializes bid form behavior
 *     - Loads active auction data
 *     - Connects startup UI interactions
 */
async function initializeApp() {
    try {
        console.log(
            "Initializing T.R.U.S.T frontend..."
        );

        /* ------------------------------------------------------------------ */
        /*                         Authentication Setup                        */
        /* ------------------------------------------------------------------ */

        const loginBtn =
            document.getElementById("loginBtn");

        if (loginBtn) {
            loginBtn.addEventListener(
                "click",
                async () => {
                    try {
                        await siweLogin();
                    } catch (err) {
                        console.error(
                            "Login initialization failed:",
                            err
                        );
                    }
                }
            );
        }

        /* ------------------------------------------------------------------ */
        /*                           Bid Form Setup                            */
        /* ------------------------------------------------------------------ */

        if (
            typeof setupBidForm ===
            "function"
        ) {
            setupBidForm();
        }

        /* ------------------------------------------------------------------ */
        /*                        Initial Auction Load                         */
        /* ------------------------------------------------------------------ */

        if (
            typeof loadActiveAuctions ===
            "function"
        ) {
            await loadActiveAuctions();
        }

        console.log(
            "Frontend initialized successfully."
        );
    } catch (err) {
        console.error(
            "Application initialization failed:",
            err
        );
    }
}

/* -------------------------------------------------------------------------- */
/*                           Startup Lifecycle                                */
/* -------------------------------------------------------------------------- */

/**
 * Starts the frontend application once the DOM is fully loaded.
 */
document.addEventListener(
    "DOMContentLoaded",
    () => {
        initializeApp();
    }
);