/*
 * bidding.rs
 *
 * Purpose:
 *     On-chain bid placement and highest bid retrieval.
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

// Define the smart contract interface
sol! {
    function bid() external payable;
    function highestBid() external view returns (uint256);
}

/// Result returned after placing a bid
#[derive(Debug)]
pub struct BidResult {
    pub tx_hash: String,
    pub highest_bid_wei: U256,
}

/// Place a bid on the auction contract
pub async fn place_bid<P>(
    provider: &P,
    contract_address: Address,
    bidder: Address,
    bid_amount_wei: U256,
) -> Result<BidResult>
where
    P: Provider + ?Sized,          // ← Required for dyn Provider
{
    let calldata = bidCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_from(bidder)
        .with_to(contract_address)
        .with_value(bid_amount_wei)
        .with_input(calldata);

    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    // Get the updated highest bid after the transaction
    let highest_bid = get_highest_bid(provider, contract_address).await?;

    Ok(BidResult {
        tx_hash: format!("{:?}", receipt.transaction_hash),
        highest_bid_wei: highest_bid,
    })
}

/// Read the current highest bid from the auction contract
pub async fn get_highest_bid<P>(
    provider: &P,
    contract_address: Address,
) -> Result<U256>
where
    P: Provider + ?Sized,          // ← Required for dyn Provider
{
    let calldata = highestBidCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_to(contract_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    let highest_bid = highestBidCall::abi_decode_returns(&response)?;

    Ok(highest_bid)
}