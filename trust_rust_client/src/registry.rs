/*
 * registry.rs
 *
 * Purpose:
 *     Provides on-chain auction registry query and data retrieval utilities.
 *
 * Responsibilities:
 *     - Query auction registry state
 *     - Retrieve auction metadata
 *     - Retrieve paginated auction results
 *     - Retrieve seller-associated auctions
 *     - Map Solidity registry structs into Rust types
 *
 * Non-Responsibilities:
 *     - Transaction execution
 *     - Auction creation
 *     - Escrow settlement
 *     - HTTP request handling
 *
 * Architecture:
 *
 *     main.rs / handlers
 *              ↓
 *         registry.rs
 *              ↓
 *      Auction Factory Contract
 *              ↓
 *        Registry Storage
 */

use alloy::{
    network::TransactionBuilder,
    primitives::{Address, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
};

use eyre::Result;
use serde::Serialize;

/* -------------------------------------------------------------------------- */
/*                          Solidity Contract Bindings                        */
/* -------------------------------------------------------------------------- */

sol! {
    struct AuctionRegistryItem {
        address auctionAddress;
        address seller;

        uint256 biddingTimeSeconds;
        uint256 endTime;

        uint256 startingBid;

        address highestBidder;
        uint256 highestBid;

        uint256 bidCount;

        address admin;

        uint256 confirmationWindow;
        uint256 createdAt;

        bool exists;

        string title;
        string description;
        string imagePlaceholder;
    }

    function getAuctions() external view returns (address[]);

    function auctionCount() external view returns (uint256);

    function getAuctionRegistryItem(
        address auctionAddress
    ) external view returns (AuctionRegistryItem);

    function getAuctionByIndex(
        uint256 index
    ) external view returns (AuctionRegistryItem);

    function getAuctionsPaginated(
        uint256 offset,
        uint256 limit
    ) external view returns (AuctionRegistryItem[]);

    function getAuctionsBySeller(
        address seller
    ) external view returns (address[]);

    function isRegisteredAuction(
        address auctionAddress
    ) external view returns (bool);
}

/* -------------------------------------------------------------------------- */
/*                               Registry Types                               */
/* -------------------------------------------------------------------------- */

/**
 * Strongly-typed Rust representation of an auction
 * registry entry returned by the factory contract.
 */
#[derive(Debug, Clone, Serialize)]
pub struct RegistryAuction {
    pub auction_address: Address,
    pub seller: Address,

    pub bidding_time_seconds: U256,
    pub end_time: U256,

    pub starting_bid_wei: U256,

    pub highest_bidder: Address,
    pub highest_bid_wei: U256,

    pub bid_count: U256,

    pub admin: Address,

    pub confirmation_window: U256,
    pub created_at: U256,

    pub exists: bool,

    pub title: String,
    pub description: String,
    pub image_placeholder: String,
}

/* -------------------------------------------------------------------------- */
/*                             Internal Mapping Logic                         */
/* -------------------------------------------------------------------------- */

/**
 * Converts a Solidity `AuctionRegistryItem` into a strongly-typed
 * Rust `RegistryAuction`.
 *
 * # Arguments
 *
 * * `item` - Solidity registry struct returned from the contract.
 *
 * # Returns
 *
 * Mapped Rust registry representation.
 */
fn map_registry_item(item: AuctionRegistryItem) -> RegistryAuction {
    RegistryAuction {
        auction_address: item.auctionAddress,
        seller: item.seller,

        bidding_time_seconds: item.biddingTimeSeconds,
        end_time: item.endTime,

        starting_bid_wei: item.startingBid,

        highest_bidder: item.highestBidder,
        highest_bid_wei: item.highestBid,

        bid_count: item.bidCount,

        admin: item.admin,

        confirmation_window: item.confirmationWindow,
        created_at: item.createdAt,

        exists: item.exists,

        title: item.title,
        description: item.description,
        image_placeholder: item.imagePlaceholder,
    }
}

/* -------------------------------------------------------------------------- */
/*                              Registry Queries                              */
/* -------------------------------------------------------------------------- */

/**
 * Retrieves the total number of registered auctions.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `factory_address` - Auction factory contract address.
 *
 * # Returns
 *
 * Total auction count as `U256`.
 *
 * # Errors
 *
 * Returns an error if:
 *     - RPC call fails
 *     - Return decoding fails
 */
pub async fn get_auction_count<P>(provider: &P, factory_address: Address) -> Result<U256>
where
    P: Provider + ?Sized,
{
    let calldata = auctionCountCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_to(factory_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    Ok(auctionCountCall::abi_decode_returns(&response)?)
}

/**
 * Retrieves all registered auction addresses.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `factory_address` - Auction factory contract address.
 *
 * # Returns
 *
 * Vector of registered auction addresses.
 *
 * # Errors
 *
 * Returns an error if:
 *     - RPC call fails
 *     - Return decoding fails
 */
pub async fn get_auctions<P>(provider: &P, factory_address: Address) -> Result<Vec<Address>>
where
    P: Provider + ?Sized,
{
    let calldata = getAuctionsCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_to(factory_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    Ok(getAuctionsCall::abi_decode_returns(&response)?)
}

/**
 * Retrieves detailed registry information for a specific auction.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `factory_address` - Auction factory contract address.
 * * `auction_address` - Auction contract address.
 *
 * # Returns
 *
 * Strongly-typed `RegistryAuction`.
 *
 * # Errors
 *
 * Returns an error if:
 *     - RPC call fails
 *     - Return decoding fails
 */
pub async fn get_auction_registry_item<P>(
    provider: &P,
    factory_address: Address,
    auction_address: Address,
) -> Result<RegistryAuction>
where
    P: Provider + ?Sized,
{
    let calldata = getAuctionRegistryItemCall {
        auctionAddress: auction_address,
    }
    .abi_encode();

    let tx = TransactionRequest::default()
        .with_to(factory_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    let item = getAuctionRegistryItemCall::abi_decode_returns(&response)?;

    Ok(map_registry_item(item))
}

/**
 * Retrieves a registry entry by its index position.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `factory_address` - Auction factory contract address.
 * * `index` - Registry index position.
 *
 * # Returns
 *
 * Strongly-typed `RegistryAuction`.
 *
 * # Errors
 *
 * Returns an error if:
 *     - RPC call fails
 *     - Return decoding fails
 */
pub async fn get_auction_by_index<P>(
    provider: &P,
    factory_address: Address,
    index: U256,
) -> Result<RegistryAuction>
where
    P: Provider + ?Sized,
{
    let calldata = getAuctionByIndexCall { index }.abi_encode();

    let tx = TransactionRequest::default()
        .with_to(factory_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    let item = getAuctionByIndexCall::abi_decode_returns(&response)?;

    Ok(map_registry_item(item))
}

/**
 * Retrieves paginated registry auction results.
 *
 * Useful for frontend pagination or large registry traversal.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `factory_address` - Auction factory contract address.
 * * `offset` - Pagination starting index.
 * * `limit` - Maximum number of results to return.
 *
 * # Returns
 *
 * Vector of `RegistryAuction` entries.
 *
 * # Errors
 *
 * Returns an error if:
 *     - RPC call fails
 *     - Return decoding fails
 */
pub async fn get_auctions_paginated<P>(
    provider: &P,
    factory_address: Address,
    offset: U256,
    limit: U256,
) -> Result<Vec<RegistryAuction>>
where
    P: Provider + ?Sized,
{
    let calldata = getAuctionsPaginatedCall { offset, limit }.abi_encode();

    let tx = TransactionRequest::default()
        .with_to(factory_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    let items = getAuctionsPaginatedCall::abi_decode_returns(&response)?;

    Ok(items.into_iter().map(map_registry_item).collect())
}

/**
 * Retrieves all auction addresses created by a seller.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `factory_address` - Auction factory contract address.
 * * `seller` - Seller wallet address.
 *
 * # Returns
 *
 * Vector of seller-created auction addresses.
 *
 * # Errors
 *
 * Returns an error if:
 *     - RPC call fails
 *     - Return decoding fails
 */
pub async fn get_auctions_by_seller<P>(
    provider: &P,
    factory_address: Address,
    seller: Address,
) -> Result<Vec<Address>>
where
    P: Provider + ?Sized,
{
    let calldata = getAuctionsBySellerCall { seller }.abi_encode();

    let tx = TransactionRequest::default()
        .with_to(factory_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    Ok(getAuctionsBySellerCall::abi_decode_returns(&response)?)
}

/**
 * Checks whether an auction address is registered.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `factory_address` - Auction factory contract address.
 * * `auction_address` - Auction contract address.
 *
 * # Returns
 *
 * `true` if the auction exists in the registry.
 *
 * # Errors
 *
 * Returns an error if:
 *     - RPC call fails
 *     - Return decoding fails
 */
pub async fn is_registered_auction<P>(
    provider: &P,
    factory_address: Address,
    auction_address: Address,
) -> Result<bool>
where
    P: Provider + ?Sized,
{
    let calldata = isRegisteredAuctionCall {
        auctionAddress: auction_address,
    }
    .abi_encode();

    let tx = TransactionRequest::default()
        .with_to(factory_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    Ok(isRegisteredAuctionCall::abi_decode_returns(&response)?)
}
