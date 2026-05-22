/*
 * claim_after_timeout_test.rs
 *
 * Purpose:
 *     Independent integration test for seller timeout settlement.
 */

mod common;

use common::{
    DEFAULT_BIDDING_TIME_SECONDS, connect_test_provider, create_test_auction_with_bid,
    get_bidder_address, get_factory_test_address, get_seller_test_address, hardhat_advance_time,
    load_test_env, print_balance,
};
use trust_rust_client::escrow::{claim_after_timeout, end_auction};

#[tokio::test]
async fn test_claim_after_timeout() {
    load_test_env();

    println!("\n================ CLAIM AFTER TIMEOUT TEST START ================");

    let provider = connect_test_provider()
        .await
        .expect("failed to connect provider");

    let factory_address = get_factory_test_address();
    let seller_address = get_seller_test_address();
    let buyer_address = get_bidder_address();

    let created = create_test_auction_with_bid(
        &provider,
        factory_address,
        seller_address,
        buyer_address,
        "Claim Timeout Test Auction",
    )
    .await;

    let auction_address = created.auction_address;

    println!("Auction address: {:?}", auction_address);
    println!("Buyer:           {:?}", buyer_address);
    println!("Seller:          {:?}", seller_address);

    println!("\n=== BALANCES BEFORE ===");
    print_balance(&provider, "Buyer", buyer_address).await;
    print_balance(&provider, "Seller", seller_address).await;

    println!("\nFast-forwarding time + ending auction...");

    hardhat_advance_time(&provider, DEFAULT_BIDDING_TIME_SECONDS)
        .await
        .expect("time advance failed");

    let _ = end_auction(&provider, auction_address, buyer_address)
        .await
        .expect("end_auction failed");

    println!("\nFast-forwarding past confirmation window...");

    hardhat_advance_time(&provider, 259_200u64)
        .await
        .expect("timeout advance failed");

    let result = claim_after_timeout(&provider, auction_address, seller_address)
        .await
        .expect("claim_after_timeout failed");

    assert!(
        !result.tx_hash.is_empty(),
        "transaction hash should not be empty"
    );

    assert!(result.escrow_settled, "escrow should be settled");

    println!("Claim result: {:#?}", result);

    println!("\n=== BALANCES AFTER CLAIM ===");
    print_balance(&provider, "Buyer", buyer_address).await;
    print_balance(&provider, "Seller", seller_address).await;

    println!("\n================ CLAIM AFTER TIMEOUT TEST PASSED ================\n");
}
