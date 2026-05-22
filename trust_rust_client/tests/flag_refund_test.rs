/*
 * flag_refund_test.rs
 *
 * Purpose:
 *     Independent integration test for admin refund flow and
 *     buyer withdrawal.
 */

mod common;

use common::{
    DEFAULT_BIDDING_TIME_SECONDS, connect_test_provider, create_test_auction_with_bid,
    get_admin_test_address, get_bidder_address, get_factory_test_address, get_seller_test_address,
    hardhat_advance_time, load_test_env, print_balance,
};
use trust_rust_client::{
    escrow::{end_auction, flag_refund},
    withdraw::withdraw,
};

#[tokio::test]
async fn test_flag_refund() {
    load_test_env();

    println!("\n================ FLAG REFUND + WITHDRAW TEST START ================");

    let provider = connect_test_provider()
        .await
        .expect("failed to connect provider");

    let factory_address = get_factory_test_address();
    let seller_address = get_seller_test_address();
    let buyer_address = get_bidder_address();
    let admin_address = get_admin_test_address();

    let created = create_test_auction_with_bid(
        &provider,
        factory_address,
        seller_address,
        buyer_address,
        "Flag Refund Test Auction",
    )
    .await;

    let auction_address = created.auction_address;

    println!("Auction: {:?}", auction_address);
    println!("Buyer:   {:?}", buyer_address);
    println!("Admin:   {:?}", admin_address);

    println!("\n=== BALANCES BEFORE ===");
    print_balance(&provider, "Buyer", buyer_address).await;

    println!("\nFast-forwarding + ending auction...");

    hardhat_advance_time(&provider, DEFAULT_BIDDING_TIME_SECONDS)
        .await
        .expect("time advance failed");

    let _ = end_auction(&provider, auction_address, buyer_address)
        .await
        .expect("end_auction failed");

    let flag_result = flag_refund(&provider, auction_address, admin_address)
        .await
        .expect("flag_refund failed");

    assert!(
        !flag_result.tx_hash.is_empty(),
        "flag refund transaction hash should not be empty"
    );

    assert!(flag_result.refund_flagged, "refund should be flagged");

    let withdraw_result = withdraw(&provider, auction_address, buyer_address)
        .await
        .expect("withdraw failed");

    assert!(
        !withdraw_result.tx_hash.is_empty(),
        "withdraw transaction hash should not be empty"
    );

    assert!(
        withdraw_result.amount_withdrawn_wei > alloy::primitives::U256::ZERO,
        "withdrawn amount should be greater than zero"
    );

    println!("Flag result: {:#?}", flag_result);
    println!("Withdraw result: {:#?}", withdraw_result);

    println!("\n=== BALANCES AFTER WITHDRAW ===");
    print_balance(&provider, "Buyer", buyer_address).await;

    println!("\n================ FLAG REFUND + WITHDRAW TEST PASSED ================\n");
}
