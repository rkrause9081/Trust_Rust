/*
 * create_auction.rs
 *
 * Purpose:
 *     Provides auction creation utilities and event decoding logic
 *     for interacting with the auction factory contract.
 *
 * Responsibilities:
 *     - Encode createAuction contract calls
 *     - Submit auction creation transactions
 *     - Decode AuctionCreated events from receipts
 *     - Return strongly-typed auction creation results
 *
 * Non-Responsibilities:
 *     - HTTP request handling
 *     - Environment configuration
 *     - Wallet management
 *     - Provider initialization
 *
 * Architecture:
 *
 *     main.rs / handlers
 *              ↓
 *      create_auction.rs
 *              ↓
 *      Auction Factory Contract
 *              ↓
 *      AuctionCreated Event
 */

use alloy::{
    network::TransactionBuilder,
    primitives::{Address, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
    sol_types::{SolCall, SolEvent},
};

use eyre::{Result, eyre};

/* -------------------------------------------------------------------------- */
/*                          Solidity Contract Bindings                        */
/* -------------------------------------------------------------------------- */

sol! {
    function createAuction(
        uint256 biddingTimeSeconds,
        uint256 startingBid,
        uint256 confirmationWindow,
        string title,
        string description
    ) external returns (address);

    event AuctionCreated(
        address indexed auctionAddress,
        address indexed seller,
        uint256 biddingTimeSeconds,
        uint256 endTime,
        uint256 startingBid,
        address admin,
        uint256 confirmationWindow
    );
}

/* -------------------------------------------------------------------------- */
/*                               Result Structs                               */
/* -------------------------------------------------------------------------- */

/**
 * Strongly-typed auction creation result.
 *
 * Contains transaction metadata and decoded event data
 * returned from the AuctionCreated event emitted by
 * the factory contract.
 */
#[derive(Debug, Clone)]
pub struct CreateAuctionResult {
    pub tx_hash: String,
    pub auction_address: Address,
    pub seller: Address,
    pub bidding_time_seconds: U256,
    pub end_time: U256,
    pub starting_bid_wei: U256,
    pub admin: Address,
    pub confirmation_window: U256,
}

/* -------------------------------------------------------------------------- */
/*                           Auction Creation Logic                           */
/* -------------------------------------------------------------------------- */

/**
 * Creates a new auction through the factory contract.
 *
 * Encodes the createAuction contract call, submits the
 * transaction to the blockchain, then scans the transaction
 * receipt for the emitted AuctionCreated event.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `factory_address` - Auction factory contract address.
 * * `seller` - Seller wallet address.
 * * `bidding_time_seconds` - Auction bidding duration.
 * * `starting_bid_wei` - Initial auction bid amount in wei.
 * * `confirmation_window` - Post-auction confirmation period.
 * * `title` - Auction title.
 * * `description` - Auction description.
 *
 * # Returns
 *
 * Strongly-typed `CreateAuctionResult` containing
 * decoded blockchain event data.
 *
 * # Errors
 *
 * Returns an error if:
 *     - Transaction submission fails
 *     - Receipt retrieval fails
 *     - Event decoding fails
 *     - AuctionCreated event is not found
 */
#[allow(clippy::too_many_arguments)]
pub async fn create_auction<P>(
    provider: &P,
    factory_address: Address,
    seller: Address,
    bidding_time_seconds: U256,
    starting_bid_wei: U256,
    confirmation_window: U256,
    title: String,
    description: String,
) -> Result<CreateAuctionResult>
where
    P: Provider + ?Sized,
{
    let calldata = createAuctionCall {
        biddingTimeSeconds: bidding_time_seconds,
        startingBid: starting_bid_wei,
        confirmationWindow: confirmation_window,
        title,
        description,
    }
    .abi_encode();

    let tx = TransactionRequest::default()
        .with_from(seller)
        .with_to(factory_address)
        .with_input(calldata);

    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    for log in receipt.logs() 
    {
        if let Ok(decoded) = AuctionCreated::decode_log(&log.inner) 
        {
            let event = decoded.data;

            return Ok(CreateAuctionResult 
            {
                tx_hash: format!("{:?}", receipt.transaction_hash),
                auction_address: event.auctionAddress,
                seller: event.seller,
                bidding_time_seconds: event.biddingTimeSeconds,
                end_time: event.endTime,
                starting_bid_wei: event.startingBid,
                admin: event.admin,
                confirmation_window: event.confirmationWindow,
            });
        }
    }

    Err(eyre!(
        "AuctionCreated event not found in transaction receipt"
    ))
}

/**
 * Creates a new auction using the default confirmation window.
 *
 * Uses a default confirmation period of:
 *     259,200 seconds (72 hours)
 *
 * This is a convenience wrapper around `create_auction()`
 * to simplify common auction creation flows.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `factory_address` - Auction factory contract address.
 * * `seller` - Seller wallet address.
 * * `bidding_time_seconds` - Auction bidding duration.
 * * `starting_bid_wei` - Initial auction bid amount in wei.
 * * `title` - Auction title.
 * * `description` - Auction description.
 *
 * # Returns
 *
 * Strongly-typed `CreateAuctionResult`.
 *
 * # Errors
 *
 * Propagates any errors returned by `create_auction()`.
 */
pub async fn create_auction_with_default_confirmation<P>(
    provider: &P,
    factory_address: Address,
    seller: Address,
    bidding_time_seconds: U256,
    starting_bid_wei: U256,
    title: String,
    description: String,
) -> Result<CreateAuctionResult>
where
    P: Provider + ?Sized,
{
    let default_confirmation_window = U256::from(259_200u64);

    create_auction(
        provider,
        factory_address,
        seller,
        bidding_time_seconds,
        starting_bid_wei,
        default_confirmation_window,
        title,
        description,
    )
    .await
}
