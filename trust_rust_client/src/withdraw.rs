/*
 * withdraw.rs
 *
 * Purpose:
 *     Provides withdrawal utilities for reclaiming pending auction funds.
 *
 * Responsibilities:
 *     - Query pending return balances
 *     - Execute withdrawal transactions
 *     - Validate withdrawal success
 *     - Format withdrawn wei amounts as ETH strings
 *
 * Non-Responsibilities:
 *     - Auction creation
 *     - Bid placement
 *     - Escrow settlement decisions
 *     - HTTP request handling
 *
 * Architecture:
 *
 *     main.rs / handlers
 *              ↓
 *         withdraw.rs
 *              ↓
 *       Auction Contract
 *              ↓
 *      Pending Returns Mapping
 */

use alloy::{
    network::TransactionBuilder,
    primitives::{Address, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
};

use eyre::{Result, eyre};

/* -------------------------------------------------------------------------- */
/*                          Solidity Contract Bindings                        */
/* -------------------------------------------------------------------------- */

sol! {
    function withdraw() external;
    function pendingReturns(address) external view returns (uint256);
}

/* -------------------------------------------------------------------------- */
/*                               Result Structs                               */
/* -------------------------------------------------------------------------- */

/**
 * Result returned after a successful withdrawal.
 *
 * Contains the transaction hash, the withdrawn amount in wei,
 * and a human-readable ETH display string.
 */
#[derive(Debug)]
pub struct WithdrawResult {
    pub tx_hash: String,
    pub amount_withdrawn_wei: U256,
    pub amount_withdrawn_eth: String,
}

/* -------------------------------------------------------------------------- */
/*                            Withdrawal Logic                                */
/* -------------------------------------------------------------------------- */

/**
 * Withdraws pending funds for a caller.
 *
 * Checks the caller's pending return balance before submitting
 * the withdrawal transaction. After confirmation, the pending
 * balance is checked again to verify that funds were cleared.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `contract_address` - Auction contract address.
 * * `caller` - Wallet address withdrawing funds.
 *
 * # Returns
 *
 * `WithdrawResult` containing transaction and amount metadata.
 *
 * # Errors
 *
 * Returns an error if:
 *     - No pending funds are available
 *     - Transaction submission fails
 *     - Receipt retrieval fails
 *     - The transaction reverts
 *     - Pending balance remains after withdrawal
 */
pub async fn withdraw<P>(
    provider: &P,
    contract_address: Address,
    caller: Address,
) -> Result<WithdrawResult>
where
    P: Provider + ?Sized,
{
    let pending_wei = get_pending_returns(provider, contract_address, caller).await?;

    if pending_wei.is_zero() {
        return Err(eyre!("No pending funds available for withdrawal"));
    }

    let calldata = withdrawCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_from(caller)
        .with_to(contract_address)
        .with_input(calldata);

    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    if !receipt.status() {
        return Err(eyre!("Withdraw transaction reverted"));
    }

    let remaining = get_pending_returns(provider, contract_address, caller).await?;

    if !remaining.is_zero() {
        return Err(eyre!(
            "Withdrawal transaction completed but pending balance still exists"
        ));
    }

    Ok(WithdrawResult {
        tx_hash: format!("{:?}", receipt.transaction_hash),
        amount_withdrawn_wei: pending_wei,
        amount_withdrawn_eth: wei_to_eth_string(pending_wei),
    })
}

/* -------------------------------------------------------------------------- */
/*                              Withdrawal Queries                            */
/* -------------------------------------------------------------------------- */

/**
 * Retrieves the pending return balance for an account.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `contract_address` - Auction contract address.
 * * `account` - Wallet address being queried.
 *
 * # Returns
 *
 * Pending return balance in wei.
 *
 * # Errors
 *
 * Returns an error if:
 *     - RPC call fails
 *     - Return data cannot be decoded
 */
pub async fn get_pending_returns<P>(
    provider: &P,
    contract_address: Address,
    account: Address,
) -> Result<U256>
where
    P: Provider + ?Sized,
{
    let calldata = pendingReturnsCall(account).abi_encode();

    let tx = TransactionRequest::default()
        .with_to(contract_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    let amount = pendingReturnsCall::abi_decode_returns(&response)?;

    Ok(amount)
}

/* -------------------------------------------------------------------------- */
/*                              Formatting Helpers                            */
/* -------------------------------------------------------------------------- */

/**
 * Converts a wei-denominated value into a short ETH display string.
 *
 * Keeps four decimal digits after the ETH decimal point for
 * readable CLI output.
 *
 * # Arguments
 *
 * * `value` - Amount in wei.
 *
 * # Returns
 *
 * Human-readable ETH amount string.
 */
fn wei_to_eth_string(value: U256) -> String {
    let wei_per_eth = U256::from(10u128.pow(18));

    let whole = value / wei_per_eth;
    let fraction = value % wei_per_eth;

    let fraction_str = format!("{:018}", fraction)
        .chars()
        .take(4)
        .collect::<String>();

    format!("{}.{}", whole, fraction_str)
}
