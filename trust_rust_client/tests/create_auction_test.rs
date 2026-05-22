/*
 * create_auction_test.rs
 *
 * Purpose:
 *     Independent integration test for on-chain auction creation
 *     through AuctionFactory.
 */

mod common;

use alloy::primitives::{Address, U256};
use common::{
    DEFAULT_BIDDING_TIME_SECONDS, DEFAULT_STARTING_BID_WEI, connect_test_provider,
    create_test_auction, get_factory_test_address, get_seller_test_address, load_test_env,
};

#[tokio::test]
async fn test_create_auction_emits_created_event() {
    load_test_env();

    println!("\n================ CREATE AUCTION TEST START ================");

    let provider = connect_test_provider()
        .await
        .expect("failed to connect provider");

    let factory_address = get_factory_test_address();
    let seller_address = get_seller_test_address();

    let result = create_test_auction(
        &provider,
        factory_address,
        seller_address,
        "Create Auction Test",
    )
    .await;

    println!("\n================ AUCTION CREATED ================");
    println!("{:#?}", result);

    assert_ne!(
        result.auction_address,
        Address::ZERO,
        "auction address should not be zero"
    );

    assert_eq!(
        result.seller, seller_address,
        "seller address should match transaction sender"
    );

    assert_eq!(
        result.bidding_time_seconds,
        U256::from(DEFAULT_BIDDING_TIME_SECONDS),
        "bidding duration should match input"
    );

    assert_eq!(
        result.starting_bid_wei,
        U256::from(DEFAULT_STARTING_BID_WEI),
        "starting bid should match input"
    );

    assert!(
        result.confirmation_window > U256::ZERO,
        "confirmation window should be greater than zero"
    );

    assert!(
        result.end_time > U256::ZERO,
        "auction end time should be greater than zero"
    );

    assert_ne!(
        result.admin,
        Address::ZERO,
        "admin address should not be zero"
    );

    assert!(
        !result.tx_hash.is_empty(),
        "transaction hash should not be empty"
    );

    println!("\n================ CREATE AUCTION TEST PASSED ================\n");
}
