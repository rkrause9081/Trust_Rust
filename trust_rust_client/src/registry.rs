/*
 * registry.rs
 *
 * Purpose:
 *     On-chain auction registry read/query layer.
 */

 use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    network::TransactionBuilder,
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
    
};
use eyre::Result;
use serde::Serialize;

sol! {
    struct AuctionRegistryItem {
        address auctionAddress;
        address seller;
        uint256 biddingTimeSeconds;
        uint256 endTime;
        uint256 startingBid;
        address admin;
        uint256 confirmationWindow;
        uint256 createdAt;
        bool exists;
    }

    function getAuctions() external view returns (address[]);
    function auctionCount() external view returns (uint256);
    function getAuctionRegistryItem(address auctionAddress) external view returns (AuctionRegistryItem);
    function getAuctionByIndex(uint256 index) external view returns (AuctionRegistryItem);
    function getAuctionsPaginated(uint256 offset, uint256 limit) external view returns (AuctionRegistryItem[]);
    function getAuctionsBySeller(address seller) external view returns (address[]);
    function isRegisteredAuction(address auctionAddress) external view returns (bool);
}

#[derive(Debug, Clone, Serialize)]
pub struct RegistryAuction {
    pub auction_address: Address,
    pub seller: Address,
    pub bidding_time_seconds: U256,
    pub end_time: U256,
    pub starting_bid_wei: U256,
    pub admin: Address,
    pub confirmation_window: U256,
    pub created_at: U256,
    pub exists: bool,
}

fn map_registry_item(item: AuctionRegistryItem) -> RegistryAuction {
    RegistryAuction {
        auction_address: item.auctionAddress,
        seller: item.seller,
        bidding_time_seconds: item.biddingTimeSeconds,
        end_time: item.endTime,
        starting_bid_wei: item.startingBid,
        admin: item.admin,
        confirmation_window: item.confirmationWindow,
        created_at: item.createdAt,
        exists: item.exists,
    }
}

pub async fn get_auction_count<P>(provider: &P, factory_address: Address) -> Result<U256>
where P: Provider + ?Sized,
{
    let calldata = auctionCountCall {}.abi_encode();
    let tx = TransactionRequest::default().with_to(factory_address).with_input(calldata);
    let response = provider.call(tx).await?;
    Ok(auctionCountCall::abi_decode_returns(&response)?)
}

pub async fn get_auctions<P>(provider: &P, factory_address: Address) -> Result<Vec<Address>>
where P: Provider + ?Sized,
{
    let calldata = getAuctionsCall {}.abi_encode();
    let tx = TransactionRequest::default().with_to(factory_address).with_input(calldata);
    let response = provider.call(tx).await?;
    Ok(getAuctionsCall::abi_decode_returns(&response)?)
}

pub async fn get_auction_registry_item<P>(provider: &P, factory_address: Address, auction_address: Address) -> Result<RegistryAuction>
where P: Provider + ?Sized,
{
    let calldata = getAuctionRegistryItemCall { auctionAddress: auction_address }.abi_encode();
    let tx = TransactionRequest::default().with_to(factory_address).with_input(calldata);
    let response = provider.call(tx).await?;
    let item = getAuctionRegistryItemCall::abi_decode_returns(&response)?;
    Ok(map_registry_item(item))
}

pub async fn get_auction_by_index<P>(provider: &P, factory_address: Address, index: U256) -> Result<RegistryAuction>
where P: Provider + ?Sized,
{
    let calldata = getAuctionByIndexCall { index }.abi_encode();
    let tx = TransactionRequest::default().with_to(factory_address).with_input(calldata);
    let response = provider.call(tx).await?;
    let item = getAuctionByIndexCall::abi_decode_returns(&response)?;
    Ok(map_registry_item(item))
}

pub async fn get_auctions_paginated<P>(provider: &P, factory_address: Address, offset: U256, limit: U256) -> Result<Vec<RegistryAuction>>
where P: Provider + ?Sized,
{
    let calldata = getAuctionsPaginatedCall { offset, limit }.abi_encode();
    let tx = TransactionRequest::default().with_to(factory_address).with_input(calldata);
    let response = provider.call(tx).await?;
    let items = getAuctionsPaginatedCall::abi_decode_returns(&response)?;
    Ok(items.into_iter().map(map_registry_item).collect())
}

pub async fn get_auctions_by_seller<P>(provider: &P, factory_address: Address, seller: Address) -> Result<Vec<Address>>
where P: Provider + ?Sized,
{
    let calldata = getAuctionsBySellerCall { seller }.abi_encode();
    let tx = TransactionRequest::default().with_to(factory_address).with_input(calldata);
    let response = provider.call(tx).await?;
    Ok(getAuctionsBySellerCall::abi_decode_returns(&response)?)
}

pub async fn is_registered_auction<P>(provider: &P, factory_address: Address, auction_address: Address) -> Result<bool>
where P: Provider + ?Sized, 
{
    let calldata = isRegisteredAuctionCall { auctionAddress: auction_address }.abi_encode();
    let tx = TransactionRequest::default().with_to(factory_address).with_input(calldata);
    let response = provider.call(tx).await?;
    Ok(isRegisteredAuctionCall::abi_decode_returns(&response)?)
}