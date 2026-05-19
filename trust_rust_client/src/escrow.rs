/*
 * escrow.rs
 *
 * Purpose:
 *     On-chain escrow operations for post-auction settlement.
 *
 *     This module handles all state-changing transactions and view calls
 *     related to the escrow lifecycle defined in AuctionEscrow.sol.
 *
 *     It does NOT:
 *         - Handle HTTP requests
 *         - Manage provider connections (passed in from main.rs)
 *         - Load contract ABIs at runtime (sol!() macro handles this at compile time)
 *         - Handle application-level business logic or validation
 *
 *     It is called by:
 *         main.rs / Axum handlers    (controller / orchestration layer)
 *
 * System Position:
 *
 *     Axum route handler / main.rs  (HTTP + orchestration layer)
 *         ↓
 *     escrow.rs  ← THIS FILE (escrow transactions + view calls)
 *         ↓
 *     auction_loader.rs  (provider connection)
 *         ↓
 *     SimpleAuction.sol (via AuctionEscrow inheritance)
 *         ↓
 *     Hardhat / Ethereum node
 *
 * Escrow Lifecycle:
 *     After endAuction() is called, one of three paths resolves the escrow:
 *
 *         1. Buyer calls confirmReceipt()     → funds sent to seller immediately
 *         2. Seller calls claimAfterTimeout() → funds sent to seller after window expires
 *         3. Admin calls flagRefund()         → funds queued in pendingReturns for winner
 *                                             (winner must call withdraw() to collect)
 */

use alloy::{
    network::TransactionBuilder,
    primitives::{Address, U256},   // ← Make sure U256 is here
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
};
use eyre::Result;

// Define the AuctionEscrow interface at compile time.
// The sol!() macro generates strongly typed Rust structs for each function.
sol! {
    function confirmReceipt() external;
    function claimAfterTimeout() external;
    function flagRefund() external;
    function timeRemainingForConfirmation() external view returns (uint256);
    function endAuction() external;
    function withdraw() external;
}

/// Result returned after a successful `confirmReceipt()` call.
#[derive(Debug)]
pub struct ConfirmReceiptResult {
    pub tx_hash: String,
    pub escrow_settled: bool,
}

/// Result returned after a successful `claimAfterTimeout()` call.
#[derive(Debug)]
pub struct ClaimAfterTimeoutResult {
    pub tx_hash: String,
    pub escrow_settled: bool,
}

/// Result returned after a successful `flagRefund()` call.
#[derive(Debug)]
pub struct FlagRefundResult {
    pub tx_hash: String,
    pub refund_flagged: bool,
}

/// Confirm receipt of the item by the winning bidder.
///
/// Immediately releases escrowed funds to the seller.
///
/// Parameters:
///     provider:         Active Alloy provider.
///     contract_address: Address of the deployed SimpleAuction contract.
///     bidder:           Address of the winning bidder (msg.sender).
///
/// Returns:
///     ConfirmReceiptResult
///
/// Errors:
///     Reverts if caller is not the winner, auction hasn't ended,
///     or escrow is already settled.
pub async fn confirm_receipt<P>(
    provider: &P,
    contract_address: Address,
    bidder: Address,
) -> Result<ConfirmReceiptResult>
where
    P: Provider,
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

/// Claim funds after the buyer confirmation window has expired.
///
/// Called by the seller.
///
/// Parameters:
///     provider:         Active Alloy provider.
///     contract_address: Address of the deployed SimpleAuction contract.
///     seller:           Address of the seller (msg.sender).
///
/// Returns:
///     ClaimAfterTimeoutResult
///
/// Errors:
///     Reverts if caller is not the seller, confirmation window hasn't expired,
///     or escrow is already settled.
pub async fn claim_after_timeout<P>(
    provider: &P,
    contract_address: Address,
    seller: Address,
) -> Result<ClaimAfterTimeoutResult>
where
    P: Provider,
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

/// Flag a refund (called only by admin).
///
/// Queues funds in `pendingReturns` for the winner. The winner must still
/// call `withdraw()` to collect the refund.
///
/// Parameters:
///     provider:         Active Alloy provider.
///     contract_address: Address of the deployed SimpleAuction contract.
///     admin:            Address of the admin (msg.sender).
///
/// Returns:
///     FlagRefundResult
///
/// Errors:
///     Reverts if caller is not admin, auction hasn't ended,
///     escrow is already settled, or no funds in escrow.
pub async fn flag_refund<P>(
    provider: &P,
    contract_address: Address,
    admin: Address,
) -> Result<FlagRefundResult>
where
    P: Provider,
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

/// Get the remaining time (in seconds) for buyer confirmation.
///
/// Returns 0 if:
///   - The confirmation window has expired
///   - Escrow has already been settled
///   - Auction has not yet ended
pub async fn get_time_remaining_for_confirmation<P>(
    provider: &P,
    contract_address: Address,
) -> Result<u64>
where
    P: Provider,
{
    let calldata = timeRemainingForConfirmationCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_to(contract_address)
        .with_input(calldata);

    let response = provider.call(tx).await?;

    let seconds_remaining: U256 = timeRemainingForConfirmationCall::abi_decode_returns(&response)?;

    // Safe conversion from U256 to u64
    let seconds = seconds_remaining.try_into().unwrap_or(0u64);

    Ok(seconds)
}

/// End the auction (usually called by anyone after bidding time expires).
///
/// This moves the auction into a settled state so escrow functions can be called.
pub async fn end_auction<P>(
    provider: &P,
    contract_address: Address,
    caller: Address,
) -> Result<String>
where
    P: Provider,
{
    let calldata = endAuctionCall {}.abi_encode();

    let tx = TransactionRequest::default()
        .with_from(caller)
        .with_to(contract_address)
        .with_input(calldata);

    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    Ok(format!("{:?}", receipt.transaction_hash))
}