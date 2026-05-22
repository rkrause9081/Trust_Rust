/*
 * bidding.rs
 *
 * Purpose:
 *     Provides on-chain auction bidding utilities and highest bid
 *     retrieval logic.
 *
 * Responsibilities:
 *     - Encode bid contract calls
 *     - Submit payable bid transactions
 *     - Query the current highest bid
 *     - Return strongly-typed bid results
 *
 * Non-Responsibilities:
 *     - HTTP request handling
 *     - Environment configuration
 *     - Provider initialization
 *     - Auction creation
 *
 * Architecture:
 *
 *     main.rs / handlers
 *              ↓
 *         bidding.rs
 *              ↓
 *      Auction Contract
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

/* -------------------------------------------------------------------------- */
/*                          Solidity Contract Bindings                        */
/* -------------------------------------------------------------------------- */

sol! {
    function bid() external payable;
    function highestBid() external view returns (uint256);
}

/* -------------------------------------------------------------------------- */
/*                               Result Structs                               */
/* -------------------------------------------------------------------------- */

/**
 * Result returned after successfully placing a bid.
 *
 * Contains the transaction hash and the updated highest bid
 * after the bid transaction has been confirmed.
 */
#[derive(Debug)]
pub struct BidResult {
    pub tx_hash: String,
    pub highest_bid_wei: U256,
}

/* -------------------------------------------------------------------------- */
/*                              Bidding Logic                                 */
/* -------------------------------------------------------------------------- */

/**
 * Places a payable bid on an auction contract.
 *
 * Encodes the `bid()` contract call, submits the transaction
 * with ETH value attached, waits for the transaction receipt,
 * then queries the updated highest bid.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `contract_address` - Auction contract address.
 * * `bidder` - Wallet address placing the bid.
 * * `bid_amount_wei` - Bid amount in wei.
 *
 * # Returns
 *
 * `BidResult` containing the transaction hash and updated highest bid.
 *
 * # Errors
 *
 * Returns an error if:
 *     - Transaction submission fails
 *     - Receipt retrieval fails
 *     - Highest bid query fails
 */
pub async fn place_bid<P>(
    provider: &P,
    contract_address: Address,
    bidder: Address,
    bid_amount_wei: U256,
) -> Result<BidResult>
where
    P: Provider + ?Sized,
{
    let calldata = bidCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_from(bidder)
        .with_to(contract_address)
        .with_value(bid_amount_wei)
        .with_input(calldata);

    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    let highest_bid = get_highest_bid(provider, contract_address).await?;

    Ok(BidResult {
        tx_hash: format!("{:?}", receipt.transaction_hash),
        highest_bid_wei: highest_bid,
    })
}

/**
 * Reads the current highest bid from an auction contract.
 *
 * Encodes the `highestBid()` view call, sends it as an RPC
 * call, then decodes the returned value as a `U256`.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `contract_address` - Auction contract address.
 *
 * # Returns
 *
 * Current highest bid amount in wei.
 *
 * # Errors
 *
 * Returns an error if:
 *     - The RPC call fails
 *     - Return data cannot be decoded
 */
pub async fn get_highest_bid<P>(provider: &P, contract_address: Address) -> Result<U256>
where
    P: Provider + ?Sized,
{
    let calldata = highestBidCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_to(contract_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    let highest_bid = highestBidCall::abi_decode_returns(&response)?;

    Ok(highest_bid)
}
