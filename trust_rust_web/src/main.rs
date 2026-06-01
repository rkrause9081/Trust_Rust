/*
 * main.rs
 *
 * Web server entry point and Axum router setup.
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

#[tokio::main]
async fn main() {
    /*
     * Keep this here for now to match your current setup.
     * Later, move it to .env so redeploying does not require recompiling.
     */
    let factory_address = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

    let state = Arc::new(
        AppState::new(factory_address)
            .await
            .expect("Failed to initialize application state"),
    );

    let app = Router::new()
        /* ----------------------------- Auth routes ----------------------------- */
        .route("/auth/nonce", get(get_nonce))
        .route("/auth/verify", post(verify_siwe))
        .route("/auth/logout", post(logout))
        /* ---------------------------- Auction routes --------------------------- */
        .route("/api/auctions", get(list_auctions_handler))
        .route("/api/create-auction", post(create_auction_handler))
        .route("/api/bid", post(place_bid_handler))
        .route("/api/withdraw", post(withdraw_handler))
        /* ----------------------------- Escrow routes --------------------------- */
        .route(
            "/api/escrow/status/{auction_address}",
            get(escrow_status_handler),
        )
        .route("/api/escrow/end", post(end_auction_handler))
        .route("/api/escrow/confirm", post(confirm_receipt_handler))
        .route("/api/escrow/claim-timeout", post(claim_timeout_handler))
        .route("/api/escrow/refund", post(refund_handler))
        /* ---------------------------- Static assets ---------------------------- */
        .fallback_service(ServeDir::new("static"))
        /* ----------------------------- Middleware ------------------------------ */
        .layer(CookieManagerLayer::new())
        .layer(LiveReloadLayer::new())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind server");

    println!("T.R.U.S.T Auction Protocol running on http://localhost:3000");

    axum::serve(listener, app).await.expect("Server crashed");
}
