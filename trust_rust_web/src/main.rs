/*
 * main.rs
 *
 * Purpose:
 *     Web server entry point and Axum router setup.
 *
 *     Initializes application state, configures all HTTP routes,
 *     registers middleware layers, serves static assets, and
 *     starts the Axum web server.
 *
 * Responsibilities:
 *     - Initialize application state
 *     - Configure API routes
 *     - Configure authentication endpoints
 *     - Configure escrow endpoints
 *     - Register middleware layers
 *     - Serve static frontend assets
 *     - Start the HTTP server
 *
 * Non-Responsibilities:
 *     - Auction business logic
 *     - Escrow contract interactions
 *     - Authentication implementation
 *     - State management internals
 *
 * Architecture:
 *
 *               main.rs
 *                   ↓
 *              Axum Router
 *                   ↓
 *     ┌─────────────┼─────────────┐
 *     ↓             ↓             ↓
 *   Auth         Auction       Escrow
 *   Routes        Routes       Routes
 *     ↓             ↓             ↓
 *          Shared AppState
 *                   ↓
 *           TRUST Protocol
 */

mod auth;
mod routes;
mod state;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

use auth::{get_nonce, logout, verify_siwe};

use routes::{
    auction_bid::place_bid_handler,
    auction_create::create_auction_handler,
    auction_query::list_auctions_handler,
    auction_withdraw::withdraw_handler,
    escrow_routes::{
        claim_timeout_handler, confirm_receipt_handler, end_auction_handler, escrow_status_handler,
        refund_handler,
    },
};

use state::AppState;

/* -------------------------------------------------------------------------- */
/*                             Application Entry Point                        */
/* -------------------------------------------------------------------------- */

/**
 * Starts the TRUST Auction Protocol web server.
 *
 * Initializes shared application state, configures all API
 * routes and middleware, binds the HTTP listener, and starts
 * serving requests.
 *
 * # Startup Flow
 *
 * 1. Configure contract addresses.
 * 2. Initialize shared application state.
 * 3. Build the Axum router.
 * 4. Register authentication routes.
 * 5. Register auction routes.
 * 6. Register escrow routes.
 * 7. Configure middleware.
 * 8. Serve static frontend assets.
 * 9. Bind TCP listener.
 * 10. Start the HTTP server.
 *
 * # Routes
 *
 * Authentication:
 *     - /auth/nonce
 *     - /auth/verify
 *     - /auth/logout
 *
 * Auction:
 *     - /api/auctions
 *     - /api/create-auction
 *     - /api/bid
 *     - /api/withdraw
 *
 * Escrow:
 *     - /api/escrow/status/{auction_address}
 *     - /api/escrow/end
 *     - /api/escrow/confirm
 *     - /api/escrow/claim-timeout
 *     - /api/escrow/refund
 */
#[tokio::main]
async fn main() {
    /*
     * Keep this here for now to match your current setup.
     * Later, move it to .env so redeploying does not require recompiling.
     */
    let factory_address = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

    // Initialize shared application state used by all handlers.
    let state = Arc::new(
        AppState::new(factory_address)
            .await
            .expect("Failed to initialize application state"),
    );

    // Configure the application's HTTP router.
    let app = Router::new()

        /* ----------------------------- Auth routes ----------------------------- */

        // Generates a SIWE authentication nonce.
        .route("/auth/nonce", get(get_nonce))

        // Verifies a signed SIWE authentication request.
        .route("/auth/verify", post(verify_siwe))

        // Logs the current user out.
        .route("/auth/logout", post(logout))

        /* ---------------------------- Auction routes --------------------------- */

        // Returns available auction listings.
        .route("/api/auctions", get(list_auctions_handler))

        // Creates a new auction.
        .route("/api/create-auction", post(create_auction_handler))

        // Places a bid on an auction.
        .route("/api/bid", post(place_bid_handler))

        // Withdraws refundable funds.
        .route("/api/withdraw", post(withdraw_handler))

        /* ----------------------------- Escrow routes --------------------------- */

        // Returns escrow status for a specific auction.
        .route(
            "/api/escrow/status/{auction_address}",
            get(escrow_status_handler),
        )

        // Ends an auction after bidding closes.
        .route("/api/escrow/end", post(end_auction_handler))

        // Buyer confirms receipt and releases escrow.
        .route("/api/escrow/confirm", post(confirm_receipt_handler))

        // Seller claims escrow after timeout expires.
        .route("/api/escrow/claim-timeout", post(claim_timeout_handler))

        // Administrator flags escrow for refund.
        .route("/api/escrow/refund", post(refund_handler))

        /* ---------------------------- Static assets ---------------------------- */

        // Serves frontend assets from the static directory.
        .fallback_service(ServeDir::new("static"))

        /* ----------------------------- Middleware ------------------------------ */

        // Enables cookie management.
        .layer(CookieManagerLayer::new())

        // Enables automatic browser live reload during development.
        .layer(LiveReloadLayer::new())

        // Inject shared application state into handlers.
        .with_state(state);

    // Bind the web server to port 3000 on all interfaces.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind server");

    println!("T.R.U.S.T Auction Protocol running on http://localhost:3000");

    // Start serving HTTP requests.
    axum::serve(listener, app)
        .await
        .expect("Server crashed");
}