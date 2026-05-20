/*
 * withdraw.rs
 */

use alloy::{
    network::TransactionBuilder,
    primitives::{Address, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
};
use eyre::{eyre, Result};

sol! {
    function withdraw() external;
    function pendingReturns(address) external view returns (uint256);
}

#[derive(Debug)]
pub struct WithdrawResult {
    pub tx_hash: String,
    pub amount_withdrawn_wei: U256,
    pub amount_withdrawn_eth: String,
}

pub async fn withdraw<P>(
    provider: &P,
    contract_address: Address,
    caller: Address,
) -> Result<WithdrawResult>
where
    P: Provider + ?Sized,
{
    let pending_wei =
        get_pending_returns(provider, contract_address, caller).await?;

    if pending_wei.is_zero() {
        return Err(eyre!(
            "No pending funds available for withdrawal"
        ));
    }

    let calldata = withdrawCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_from(caller)
        .with_to(contract_address)
        .with_input(calldata);

    let receipt = provider
        .send_transaction(tx)
        .await?
        .get_receipt()
        .await?;

    if !receipt.status() {
        return Err(eyre!("Withdraw transaction reverted"));
    }

    let remaining =
        get_pending_returns(provider, contract_address, caller).await?;

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