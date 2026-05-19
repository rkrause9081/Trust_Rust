/*
 * state.rs
 *
 * Purpose:
 *     Defines thread-safe shared application state.
 *
 *     This module is responsible for:
 *         - Storing active nonces (for SIWE replay protection)
 *         - Tracking active user sessions
 *
 *     It does NOT:
 *         - Handle authentication logic
 *         - Persist data to disk (in-memory only)
 *
 * Note: For production, replace in-memory HashMaps with Redis or a database.
 */

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use alloy::{
    network::Ethereum,
    primitives::Address,
    providers::Provider,
};
use eyre::Result;
use url::Url;

#[derive(Clone)]
pub struct AppState {
    // Auth
    pub nonces: Arc<Mutex<HashMap<String, String>>>,
    pub sessions: Arc<Mutex<HashMap<String, String>>>,

    // Blockchain
    pub rpc_provider: Arc<dyn Provider<Ethereum> + Send + Sync>,
    pub factory_address: Address,
}

impl AppState {
    pub async fn new(factory_address: &str) -> Result<Self> {
        let rpc_url: Url = "http://127.0.0.1:8545".parse()?;

        let provider = alloy::providers::ProviderBuilder::new()
            .connect_http(rpc_url);

        let factory_addr: Address = factory_address.parse()?;

        Ok(Self {
            nonces: Arc::new(Mutex::new(HashMap::new())),
            sessions: Arc::new(Mutex::new(HashMap::new())),
            rpc_provider: Arc::new(provider) as Arc<dyn Provider<Ethereum> + Send + Sync>,
            factory_address: factory_addr,
        })
    }
}