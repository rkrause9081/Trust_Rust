/*
 * escrow.rs
 *
 * Purpose:
 *     On-chain escrow operations for post-auction settlement.
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

sol! {
    function confirmReceipt() external;
    function claimAfterTimeout() external;
    function flagRefund() external;
    function timeRemainingForConfirmation() external view returns (uint256);
    function endAuction() external;
    function withdraw() external;

    function getEscrowStatus() external view returns (uint8);

    function canConfirmReceipt(address caller)
        external
        view
        returns (bool);

    function canClaimTimeout(address caller)
        external
        view
        returns (bool);

    function canFlagRefund(address caller)
        external
        view
        returns (bool);
}

#[derive(Debug)]
pub struct ConfirmReceiptResult {
    pub tx_hash: String,
    pub escrow_settled: bool,
}

#[derive(Debug)]
pub struct ClaimAfterTimeoutResult {
    pub tx_hash: String,
    pub escrow_settled: bool,
}

#[derive(Debug)]
pub struct FlagRefundResult {
    pub tx_hash: String,
    pub refund_flagged: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum EscrowStatus {
    ActiveAuction,
    AwaitingFinalization,
    AwaitingBuyerConfirmation,
    Complete,
    Refunded,
}

impl EscrowStatus {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::ActiveAuction,
            1 => Self::AwaitingFinalization,
            2 => Self::AwaitingBuyerConfirmation,
            3 => Self::Complete,
            4 => Self::Refunded,
            _ => Self::ActiveAuction,
        }
    }
}

pub async fn confirm_receipt<P>(
    provider: &P,
    contract_address: Address,
    bidder: Address,
) -> Result<ConfirmReceiptResult>
where
    P: Provider + ?Sized,
{
    let calldata = confirmReceiptCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_from(bidder)
        .with_to(contract_address)
        .with_input(calldata);

    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    Ok(ConfirmReceiptResult {
        tx_hash: format!("{:?}", receipt.transaction_hash),
        escrow_settled: true,
    })
}

pub async fn claim_after_timeout<P>(
    provider: &P,
    contract_address: Address,
    seller: Address,
) -> Result<ClaimAfterTimeoutResult>
where
    P: Provider + ?Sized,
{
    let calldata = claimAfterTimeoutCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_from(seller)
        .with_to(contract_address)
        .with_input(calldata);

    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    Ok(ClaimAfterTimeoutResult {
        tx_hash: format!("{:?}", receipt.transaction_hash),
        escrow_settled: true,
    })
}

pub async fn flag_refund<P>(
    provider: &P,
    contract_address: Address,
    admin: Address,
) -> Result<FlagRefundResult>
where
    P: Provider + ?Sized,
{
    let calldata = flagRefundCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_from(admin)
        .with_to(contract_address)
        .with_input(calldata);

    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    Ok(FlagRefundResult {
        tx_hash: format!("{:?}", receipt.transaction_hash),
        refund_flagged: true,
    })
}

pub async fn get_time_remaining_for_confirmation<P>(
    provider: &P,
    contract_address: Address,
) -> Result<u64>
where
    P: Provider + ?Sized,
{
    let calldata = timeRemainingForConfirmationCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_to(contract_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    let seconds_remaining: U256 =
        timeRemainingForConfirmationCall::abi_decode_returns(&response)?;

    let seconds = seconds_remaining.try_into().unwrap_or(0u64);

    Ok(seconds)
}

pub async fn end_auction<P>(
    provider: &P,
    contract_address: Address,
    caller: Address,
) -> Result<String>
where
    P: Provider + ?Sized,
{
    let calldata = endAuctionCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_from(caller)
        .with_to(contract_address)
        .with_input(calldata);

    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    Ok(format!("{:?}", receipt.transaction_hash))
}

pub async fn get_escrow_status<P>(
    provider: &P,
    contract_address: Address,
) -> Result<EscrowStatus>
where
    P: Provider + ?Sized,
{
    let calldata = getEscrowStatusCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_to(contract_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    let raw: u8 = getEscrowStatusCall::abi_decode_returns(&response)?;

    Ok(EscrowStatus::from_u8(raw))
}

pub async fn can_confirm_receipt<P>(
    provider: &P,
    contract_address: Address,
    caller: Address,
) -> Result<bool>
where
    P: Provider + ?Sized,
{
    let calldata = canConfirmReceiptCall { caller }.abi_encode();

    let tx = TransactionRequest::default()
        .with_to(contract_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    Ok(canConfirmReceiptCall::abi_decode_returns(&response)?)
}

pub async fn can_claim_timeout<P>(
    provider: &P,
    contract_address: Address,
    caller: Address,
) -> Result<bool>
where
    P: Provider + ?Sized,
{
    let calldata = canClaimTimeoutCall { caller }.abi_encode();

    let tx = TransactionRequest::default()
        .with_to(contract_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    Ok(canClaimTimeoutCall::abi_decode_returns(&response)?)
}

pub async fn can_flag_refund<P>(
    provider: &P,
    contract_address: Address,
    caller: Address,
) -> Result<bool>
where
    P: Provider + ?Sized,
{
    let calldata = canFlagRefundCall { caller }.abi_encode();

    let tx = TransactionRequest::default()
        .with_to(contract_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    Ok(canFlagRefundCall::abi_decode_returns(&response)?)
}