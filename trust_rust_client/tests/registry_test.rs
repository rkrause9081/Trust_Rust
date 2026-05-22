/*
 * registry_test.rs
 *
 * Purpose:
 *     Independent integration test for AuctionFactory registry
 *     read/query calls.
 */

mod common;

use alloy::primitives::U256;
use common::{
    connect_test_provider, create_test_auction, get_factory_test_address, get_seller_test_address,
    load_test_env,
};
use trust_rust_client::registry::{
    get_auction_by_index, get_auction_count, get_auction_registry_item, get_auctions_by_seller,
    get_auctions_paginated, is_registered_auction,
};

#[tokio::test]
async fn test_registry_reads_created_auction() {
    load_test_env();

    println!("\n================ REGISTRY TEST START ================");

    let provider = connect_test_provider()
        .await
        .expect("failed to connect provider");

    let factory_address = get_factory_test_address();
    let seller_address = get_seller_test_address();

    let count_before = get_auction_count(&provider, factory_address)
        .await
        .expect("auctionCount call failed");

    let created = create_test_auction(
        &provider,
        factory_address,
        seller_address,
        "Registry Test Auction",
    )
    .await;

    let count_after = get_auction_count(&provider, factory_address)
        .await
        .expect("auctionCount after create failed");

    assert_eq!(
        count_after,
        count_before + U256::from(1u64),
        "registry count should increase by one"
    );

    let exists = is_registered_auction(&provider, factory_address, created.auction_address)
        .await
        .expect("isRegisteredAuction call failed");

    assert!(exists, "created auction should be registered");

    let item_by_address =
        get_auction_registry_item(&provider, factory_address, created.auction_address)
            .await
            .expect("getAuctionRegistryItem call failed");

    assert_eq!(item_by_address.auction_address, created.auction_address);

    assert_eq!(item_by_address.seller, seller_address);

    assert_eq!(
        item_by_address.bidding_time_seconds,
        created.bidding_time_seconds
    );

    assert_eq!(item_by_address.starting_bid_wei, created.starting_bid_wei);

    assert_eq!(
        item_by_address.confirmation_window,
        created.confirmation_window
    );

    assert!(item_by_address.exists, "registry item should exist");

    let item_by_index = get_auction_by_index(&provider, factory_address, count_before)
        .await
        .expect("getAuctionByIndex call failed");

    assert_eq!(item_by_index.auction_address, created.auction_address);

    let page = get_auctions_paginated(&provider, factory_address, count_before, U256::from(1u64))
        .await
        .expect("getAuctionsPaginated call failed");

    assert_eq!(page.len(), 1);

    assert_eq!(page[0].auction_address, created.auction_address);

    let seller_auctions = get_auctions_by_seller(&provider, factory_address, seller_address)
        .await
        .expect("getAuctionsBySeller call failed");

    assert!(
        seller_auctions.contains(&created.auction_address),
        "seller auction list should contain created auction"
    );

    println!("\n================ REGISTRY TEST PASSED ================\n");
}
