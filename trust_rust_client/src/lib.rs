/*
 * lib.rs
 *
 * Purpose:
 *     Crate root — declares and exports all public modules.
 */

pub mod config;
pub mod auction_loader;
pub mod bidding;
pub mod create_auction;
pub mod registry;
pub mod escrow;
pub mod withdraw;

// === Public re-exports for easy use from trust_rust_web ===

pub use create_auction::{
    create_auction,
    create_auction_with_default_confirmation,
    CreateAuctionResult,
};

pub use escrow::{
    confirm_receipt,
    claim_after_timeout,
    flag_refund,
    end_auction,
    get_time_remaining_for_confirmation,
    get_escrow_status,
    can_confirm_receipt,
    can_claim_timeout,
    can_flag_refund,
    ConfirmReceiptResult,
    ClaimAfterTimeoutResult,
    FlagRefundResult,
    EscrowStatus,
};

pub use withdraw::{
    withdraw,
    get_pending_returns,
    WithdrawResult,
};