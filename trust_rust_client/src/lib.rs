/*
 * lib.rs
 *
 * Purpose:
 *     Crate root for the trust_rust_web library.
 *
 * Responsibilities:
 *     - Declare public crate modules
 *     - Re-export commonly used APIs
 *     - Provide a centralized external interface
 *
 * Non-Responsibilities:
 *     - Business logic execution
 *     - Blockchain interaction
 *     - Runtime orchestration
 *     - HTTP request handling
 *
 * Architecture:
 *
 *         External Applications
 *                  ↓
 *               lib.rs
 *                  ↓
 *          Public Module APIs
 *                  ↓
 *        Blockchain Interaction
 */

/* -------------------------------------------------------------------------- */
/*                              Public Modules                                */
/* -------------------------------------------------------------------------- */

pub mod auction_loader;
pub mod bidding;
pub mod config;
pub mod create_auction;
pub mod escrow;
pub mod registry;
pub mod withdraw;

/* -------------------------------------------------------------------------- */
/*                              Public Re-Exports                             */
/* -------------------------------------------------------------------------- */

/**
 * Auction creation utilities and result types.
 */
pub use create_auction::{
    CreateAuctionResult, create_auction, create_auction_with_default_confirmation,
};

/**
 * Escrow settlement utilities, permission checks,
 * and escrow lifecycle types.
 */
pub use escrow::{
    ClaimAfterTimeoutResult, ConfirmReceiptResult, EscrowStatus, FlagRefundResult,
    can_claim_timeout, can_confirm_receipt, can_flag_refund, claim_after_timeout, confirm_receipt,
    end_auction, flag_refund, get_escrow_status, get_time_remaining_for_confirmation,
};

/**
 * Withdrawal and pending return utilities.
 */
pub use withdraw::{WithdrawResult, get_pending_returns, withdraw};
