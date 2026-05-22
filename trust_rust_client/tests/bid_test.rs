/*
 * bid_test.rs
 *
 * Purpose:
 *     Independent integration test for on-chain bid placement.
 *
 *     This test creates a fresh auction, places a bid, then verifies
 *     that the highest bid was updated correctly.
 */

mod common;

use alloy::primitives::U256;
use common::{
    DEFAULT_BID_AMOUNT_WEI, connect_test_provider, create_test_auction, get_bidder_address,
    get_factory_test_address, get_seller_test_address, load_test_env,
};
use trust_rust_client::bidding::{get_highest_bid, place_bid};

#[tokio::test]
async fn test_place_bid_updates_highest_bid() {
    load_test_env();

    println!("\n================ BID TEST START ================");

    let provider = connect_test_provider()
        .await
        .expect("failed to connect provider");

    let factory_address = get_factory_test_address();
    let seller_address = get_seller_test_address();
    let bidder_address = get_bidder_address();

    let created = create_test_auction(
        &provider,
        factory_address,
        seller_address,
        "Bid Test Auction",
    )
    .await;

    let auction_address = created.auction_address;
    let bid_amount = U256::from(DEFAULT_BID_AMOUNT_WEI);

    println!("Factory address: {:?}", factory_address);
    println!("Auction address: {:?}", auction_address);
    println!("Bidder address:  {:?}", bidder_address);

    let highest_before = get_highest_bid(&provider, auction_address)
        .await
        .expect("highestBid before bid failed");

    println!("\nHighest bid before transaction:");
    println!("  highest_before_wei: {}", highest_before);

    let result = place_bid(&provider, auction_address, bidder_address, bid_amount)
        .await
        .expect("place_bid failed");

    println!("\n================ BID TRANSACTION CONFIRMED ================");
    println!("{:#?}", result);

    let highest_after = get_highest_bid(&provider, auction_address)
        .await
        .expect("highestBid after bid failed");

    assert_eq!(
        highest_after, result.highest_bid_wei,
        "highest bid returned by place_bid should match direct contract read"
    );

    assert!(
        highest_after >= bid_amount,
        "highest bid should be at least the submitted bid amount"
    );

    assert!(
        highest_after >= highest_before,
        "highest bid should not decrease after placing a bid"
    );

    assert!(
        !result.tx_hash.is_empty(),
        "transaction hash should not be empty"
    );

    println!("\n================ BID TEST PASSED ================\n");
}
