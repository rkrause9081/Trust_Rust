/*
 * escrow.rs
 *
 * Purpose:
 *     Provides escrow settlement and post-auction lifecycle
 *     interaction utilities.
 *
 * Responsibilities:
 *     - Confirm auction receipt
 *     - Handle seller timeout claims
 *     - Flag escrow refunds
 *     - Query escrow status
 *     - Query escrow permissions
 *     - Execute settlement-related transactions
 *
 * Non-Responsibilities:
 *     - HTTP request handling
 *     - Provider initialization
 *     - Environment configuration
 *     - Auction creation
 *
 * Architecture:
 *
 *     main.rs / handlers
 *              ↓
 *          escrow.rs
 *              ↓
 *       Auction Contract
 *              ↓
 *      Escrow State Logic
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

/* -------------------------------------------------------------------------- */
/*                               Result Structs                               */
/* -------------------------------------------------------------------------- */

/**
 * Result returned after confirming auction receipt.
 */
#[derive(Debug)]
pub struct ConfirmReceiptResult {
    pub tx_hash: String,
    pub escrow_settled: bool,
}

/**
 * Result returned after claiming escrow settlement
 * following confirmation timeout expiration.
 */
#[derive(Debug)]
pub struct ClaimAfterTimeoutResult {
    pub tx_hash: String,
    pub escrow_settled: bool,
}

/**
 * Result returned after flagging an escrow refund.
 */
#[derive(Debug)]
pub struct FlagRefundResult {
    pub tx_hash: String,
    pub refund_flagged: bool,
}

/* -------------------------------------------------------------------------- */
/*                               Escrow Status                                */
/* -------------------------------------------------------------------------- */

/**
 * Represents the current escrow lifecycle state.
 */
#[derive(Debug, Clone, Copy)]
pub enum EscrowStatus {
    ActiveAuction,
    AwaitingFinalization,
    AwaitingBuyerConfirmation,
    Complete,
    Refunded,
}

impl EscrowStatus {
    /**
     * Converts a raw contract enum value into a typed `EscrowStatus`.
     *
     * Unknown values default to `ActiveAuction`.
     *
     * # Arguments
     *
     * * `value` - Raw uint8 status value returned by the contract.
     *
     * # Returns
     *
     * Typed `EscrowStatus`.
     */
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

/* -------------------------------------------------------------------------- */
/*                           Escrow Transaction Logic                         */
/* -------------------------------------------------------------------------- */

/**
 * Confirms successful auction receipt.
 *
 * Typically called by the winning bidder after receiving
 * the auctioned item, allowing escrow settlement completion.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `contract_address` - Auction contract address.
 * * `bidder` - Winning bidder wallet address.
 *
 * # Returns
 *
 * `ConfirmReceiptResult` containing settlement metadata.
 *
 * # Errors
 *
 * Returns an error if:
 *     - Transaction submission fails
 *     - Receipt retrieval fails
 */
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

/**
 * Claims escrow settlement after the buyer confirmation
 * timeout window expires.
 *
 * Typically called by the seller if the buyer fails
 * to confirm receipt within the configured confirmation window.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `contract_address` - Auction contract address.
 * * `seller` - Seller wallet address.
 *
 * # Returns
 *
 * `ClaimAfterTimeoutResult` containing settlement metadata.
 *
 * # Errors
 *
 * Returns an error if:
 *     - Transaction submission fails
 *     - Receipt retrieval fails
 */
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

/**
 * Flags an escrow refund through an admin-authorized action.
 *
 * Typically used for dispute resolution or administrative
 * intervention during escrow settlement.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `contract_address` - Auction contract address.
 * * `admin` - Authorized admin wallet address.
 *
 * # Returns
 *
 * `FlagRefundResult` containing refund metadata.
 *
 * # Errors
 *
 * Returns an error if:
 *     - Transaction submission fails
 *     - Receipt retrieval fails
 */
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

/* -------------------------------------------------------------------------- */
/*                              Escrow Queries                                */
/* -------------------------------------------------------------------------- */

/**
 * Retrieves the remaining buyer confirmation window time.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `contract_address` - Auction contract address.
 *
 * # Returns
 *
 * Remaining confirmation time in seconds.
 *
 * # Errors
 *
 * Returns an error if:
 *     - RPC call fails
 *     - Return decoding fails
 */
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

    let seconds_remaining: U256 = timeRemainingForConfirmationCall::abi_decode_returns(&response)?;

    let seconds = seconds_remaining.try_into().unwrap_or(0u64);

    Ok(seconds)
}

/**
 * Ends an auction after the bidding period expires.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `contract_address` - Auction contract address.
 * * `caller` - Wallet address ending the auction.
 *
 * # Returns
 *
 * Transaction hash of the auction finalization transaction.
 *
 * # Errors
 *
 * Returns an error if:
 *     - Transaction submission fails
 *     - Receipt retrieval fails
 */
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

/**
 * Retrieves the current escrow lifecycle status.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `contract_address` - Auction contract address.
 *
 * # Returns
 *
 * Typed `EscrowStatus`.
 *
 * # Errors
 *
 * Returns an error if:
 *     - RPC call fails
 *     - Return decoding fails
 */
pub async fn get_escrow_status<P>(provider: &P, contract_address: Address) -> Result<EscrowStatus>
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

/* -------------------------------------------------------------------------- */
/*                          Escrow Permission Checks                          */
/* -------------------------------------------------------------------------- */

/**
 * Checks whether a caller can confirm receipt.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `contract_address` - Auction contract address.
 * * `caller` - Wallet address being checked.
 *
 * # Returns
 *
 * `true` if receipt confirmation is allowed.
 *
 * # Errors
 *
 * Returns an error if the RPC call or decoding fails.
 */
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

/**
 * Checks whether a caller can claim escrow settlement
 * after timeout expiration.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `contract_address` - Auction contract address.
 * * `caller` - Wallet address being checked.
 *
 * # Returns
 *
 * `true` if timeout claiming is allowed.
 *
 * # Errors
 *
 * Returns an error if the RPC call or decoding fails.
 */
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

/**
 * Checks whether a caller can flag a refund.
 *
 * # Arguments
 *
 * * `provider` - Active Alloy provider instance.
 * * `contract_address` - Auction contract address.
 * * `caller` - Wallet address being checked.
 *
 * # Returns
 *
 * `true` if refund flagging is allowed.
 *
 * # Errors
 *
 * Returns an error if the RPC call or decoding fails.
 */
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
