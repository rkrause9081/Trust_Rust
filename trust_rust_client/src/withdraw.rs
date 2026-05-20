/*
 * withdraw.rs
 *
 * Purpose:
 *     On-chain withdrawal of funds queued in pendingReturns.
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

// Define the interface
sol! {
    function withdraw() external;
    function pendingReturns(address) external view returns (uint256);
}

/// Result of a successful withdrawal
#[derive(Debug)]
pub struct WithdrawResult {
    pub tx_hash: String,
    pub amount_withdrawn_wei: U256,
    pub amount_withdrawn_eth: f64,
}

/// Withdraw all pending returns for the caller.
pub async fn withdraw<P>(
    provider: &P,
    contract_address: Address,
    caller: Address,
) -> Result<WithdrawResult>
where
    P: Provider + ?Sized,          // ← Fixed
{
    let pending_wei = get_pending_returns(provider, contract_address, caller).await?;

    if pending_wei.is_zero() {
        return Err(eyre::eyre!(
            "No pending funds to withdraw for address {}",
            caller
        ));
    }

    let calldata = withdrawCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_from(caller)
        .with_to(contract_address)
        .with_input(calldata);

    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    // Verify funds were withdrawn
    let remaining = get_pending_returns(provider, contract_address, caller).await?;
    if !remaining.is_zero() {
        return Err(eyre::eyre!(
            "Withdrawal tx succeeded but funds were not transferred."
        ));
    }

    let amount_eth = pending_wei.to::<u128>() as f64 / 1e18;

    Ok(WithdrawResult {
        tx_hash: format!("{:?}", receipt.transaction_hash),
        amount_withdrawn_wei: pending_wei,
        amount_withdrawn_eth: amount_eth,
    })
}

/// Read pending returns for an address (view function)
pub async fn get_pending_returns<P>(
    provider: &P,
    contract_address: Address,
    account: Address,
) -> Result<U256>
where
    P: Provider + ?Sized,          // ← Fixed
{
    let calldata = pendingReturnsCall(account).abi_encode();

    let tx = TransactionRequest::default()
        .with_to(contract_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;
    let amount = pendingReturnsCall::abi_decode_returns(&response)?;

    Ok(amount)
}