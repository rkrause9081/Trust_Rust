/*
 * state.rs
 *
 * Purpose:
 *     Defines thread-safe shared application state for the
 *     Trust Rust web server.
 *
 * Responsibilities:
 *     - Store active SIWE nonces
 *     - Store active user sessions
 *     - Hold shared blockchain provider access
 *     - Store configured factory contract address
 *
 * Non-Responsibilities:
 *     - Authentication logic
 *     - Persistent session storage
 *     - Database management
 *     - HTTP route handling
 *
 * Architecture:
 *
 *          Axum Handlers
 *                 ↓
 *              AppState
 *                 ↓
 *      Shared Auth + Blockchain State
 *
 * Notes:
 *     Current storage is fully in-memory.
 *
 *     For production deployments:
 *         - Replace HashMaps with Redis or a database
 *         - Add session expiration/cleanup
 *         - Add distributed state synchronization
 */

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use alloy::{network::Ethereum, primitives::Address, providers::Provider};

use eyre::Result;
use url::Url;

/* -------------------------------------------------------------------------- */
/*                              Shared App State                              */
/* -------------------------------------------------------------------------- */

/**
 * Shared thread-safe application state.
 *
 * This state is cloned and shared across Axum handlers
 * through Arc-based interior mutability.
 */
#[derive(Clone)]
pub struct AppState {
    /* ---------------------------------------------------------------------- */
    /*                               Auth State                               */
    /* ---------------------------------------------------------------------- */
    /// Active SIWE nonces used for replay protection.
    pub nonces: Arc<Mutex<HashMap<String, String>>>,

    /// Active authenticated user sessions.
    pub sessions: Arc<Mutex<HashMap<String, String>>>,

    /* ---------------------------------------------------------------------- */
    /*                            Blockchain State                            */
    /* ---------------------------------------------------------------------- */
    /// Shared Alloy RPC provider instance.
    pub rpc_provider: Arc<dyn Provider<Ethereum> + Send + Sync>,

    /// Auction factory contract address.
    pub factory_address: Address,
}

/* -------------------------------------------------------------------------- */
/*                           AppState Construction                            */
/* -------------------------------------------------------------------------- */

impl AppState {
    /**
     * Creates a new shared application state instance.
     *
     * Initializes:
     *     - In-memory nonce storage
     *     - In-memory session storage
     *     - Alloy RPC provider connection
     *     - Parsed factory contract address
     *
     * # Arguments
     *
     * * `factory_address` - Auction factory contract address as a string.
     *
     * # Returns
     *
     * Fully initialized `AppState`.
     *
     * # Errors
     *
     * Returns an error if:
     *     - The RPC URL is invalid
     *     - The factory address cannot be parsed
     */
    pub async fn new(factory_address: &str) -> Result<Self> {
        let rpc_url: Url = "http://127.0.0.1:8545".parse()?;

        let provider = alloy::providers::ProviderBuilder::new().connect_http(rpc_url);

        let factory_addr: Address = factory_address.parse()?;

        Ok(Self {
            nonces: Arc::new(Mutex::new(HashMap::new())),

            sessions: Arc::new(Mutex::new(HashMap::new())),

            rpc_provider: Arc::new(provider) as Arc<dyn Provider<Ethereum> + Send + Sync>,

            factory_address: factory_addr,
        })
    }
}
