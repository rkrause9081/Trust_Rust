/*
 * main.rs
 *
 * Purpose:
 *     Web server entry point and top-level Axum application setup.
 *
 * Responsibilities:
 *     - Initialize shared application state
 *     - Register authentication routes
 *     - Register auction API routes
 *     - Register escrow API routes
 *     - Serve static frontend assets
 *     - Attach middleware layers
 *     - Start the HTTP server
 *
 * Non-Responsibilities:
 *     - Authentication verification internals
 *     - Blockchain transaction logic
 *     - Request handler business logic
 *     - Persistent storage management
 *
 * Architecture:
 *
 *      Browser / API Client
 *              ↓
 *          Axum Router
 *              ↓
 *      Route Handlers + AppState
 *              ↓
 *      trust_rust_client / Blockchain
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

use auth::{get_nonce, verify_siwe};

use routes::{
    auction::create_auction_handler,
    auction_list::list_auctions,
    bidding::place_bid_handler,
    escrow::{
        claim_timeout_handler, confirm_receipt_handler, end_auction_handler, escrow_status_handler,
        refund_handler,
    },
    withdraw::withdraw_handler,
};

use state::AppState;

/* -------------------------------------------------------------------------- */
/*                              Application Entry                             */
/* -------------------------------------------------------------------------- */

/**
 * Starts the Trust Rust web server.
 *
 * Initializes shared state, builds the Axum router, registers
 * API routes and middleware, then binds the server to port 3000.
 */
#[tokio::main]
async fn main() {
    let factory_address = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

    let state = Arc::new(
        AppState::new(factory_address)
            .await
            .expect("Failed to initialize state"),
    );

    let app = Router::new()
        /* ------------------------------------------------------------------ */
        /*                              Auth Routes                           */
        /* ------------------------------------------------------------------ */
        .route("/auth/nonce", get(get_nonce))
        .route("/auth/verify", post(verify_siwe))
        /* ------------------------------------------------------------------ */
        /*                             Auction Routes                         */
        /* ------------------------------------------------------------------ */
        .route("/api/create-auction", post(create_auction_handler))
        .route("/api/auctions", get(list_auctions))
        .route("/api/bid", post(place_bid_handler))
        .route("/api/withdraw", post(withdraw_handler))
        /* ------------------------------------------------------------------ */
        /*                              Escrow Routes                         */
        /* ------------------------------------------------------------------ */
        .route(
            "/api/escrow/status/{auction_address}",
            get(escrow_status_handler),
        )
        .route("/api/escrow/end", post(end_auction_handler))
        .route("/api/escrow/confirm", post(confirm_receipt_handler))
        .route("/api/escrow/claim-timeout", post(claim_timeout_handler))
        .route("/api/escrow/refund", post(refund_handler))
        /* ------------------------------------------------------------------ */
        /*                            Static Frontend                         */
        /* ------------------------------------------------------------------ */
        .fallback_service(ServeDir::new("static"))
        /* ------------------------------------------------------------------ */
        /*                              Middleware                            */
        /* ------------------------------------------------------------------ */
        .layer(CookieManagerLayer::new())
        .layer(LiveReloadLayer::new())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("T.R.U.S.T Auction Protocol running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
