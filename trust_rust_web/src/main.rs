mod auth;
mod state;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tower_cookies::CookieManagerLayer;

use auth::{get_nonce, verify_siwe};
use routes::auction::create_auction_handler;
use routes::auction_list::list_auctions;
use routes::bidding::place_bid_handler;
use routes::withdraw::withdraw_handler;
use state::AppState;

#[tokio::main]
async fn main() {
    // ←←← CHANGE THIS TO YOUR REAL AUCTION FACTORY ADDRESS ←←←
    let factory_address = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

    let state = Arc::new(
        AppState::new(factory_address)
            .await
            .expect("Failed to initialize state")
    );

    let app = Router::new()
        // Auth
        .route("/auth/nonce", get(get_nonce))
        .route("/auth/verify", post(verify_siwe))

        // Auction routes
        .route("/api/create-auction", post(create_auction_handler))
        .route("/api/auctions", get(list_auctions))
        .route("/api/bid", post(place_bid_handler))
        .route("/api/withdraw",post(withdraw_handler))

        // Serve frontend static files
        .fallback_service(ServeDir::new("static"))

        // Middleware
        .layer(CookieManagerLayer::new())
        .layer(LiveReloadLayer::new())

        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 T.R.U.S.T Auction Protocol running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}