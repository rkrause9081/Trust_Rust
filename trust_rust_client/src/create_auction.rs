use alloy::{
    network::TransactionBuilder,
    primitives::{Address, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
    sol_types::{SolCall, SolEvent},
};

use eyre::{eyre, Result};

sol! {
    function createAuction(
        uint256 biddingTimeSeconds,
        uint256 startingBid,
        uint256 confirmationWindow,
        string title,
        string description
    ) external returns (address);

    event AuctionCreated(
        address indexed auctionAddress,
        address indexed seller,
        uint256 biddingTimeSeconds,
        uint256 endTime,
        uint256 startingBid,
        address admin,
        uint256 confirmationWindow
    );
}

#[derive(Debug, Clone)]
pub struct CreateAuctionResult {
    pub tx_hash: String,
    pub auction_address: Address,
    pub seller: Address,
    pub bidding_time_seconds: U256,
    pub end_time: U256,
    pub starting_bid_wei: U256,
    pub admin: Address,
    pub confirmation_window: U256,
}

pub async fn create_auction<P>(
    provider: &P,
    factory_address: Address,
    seller: Address,
    bidding_time_seconds: U256,
    starting_bid_wei: U256,
    confirmation_window: U256,
    title: String,
    description: String,
) -> Result<CreateAuctionResult>
where
    P: Provider + ?Sized,
{
    let calldata = createAuctionCall {
        biddingTimeSeconds: bidding_time_seconds,
        startingBid: starting_bid_wei,
        confirmationWindow: confirmation_window,
        title,
        description,
    }
    .abi_encode();

    let tx = TransactionRequest::default()
        .with_from(seller)
        .with_to(factory_address)
        .with_input(calldata);

    let receipt = provider
        .send_transaction(tx)
        .await?
        .get_receipt()
        .await?;

    for log in receipt.logs() {
        if let Ok(decoded) = AuctionCreated::decode_log(&log.inner) {
            let event = decoded.data;

            return Ok(CreateAuctionResult {
                tx_hash: format!("{:?}", receipt.transaction_hash),
                auction_address: event.auctionAddress,
                seller: event.seller,
                bidding_time_seconds: event.biddingTimeSeconds,
                end_time: event.endTime,
                starting_bid_wei: event.startingBid,
                admin: event.admin,
                confirmation_window: event.confirmationWindow,
            });
        }
    }

    Err(eyre!(
        "AuctionCreated event not found in transaction receipt"
    ))
}

pub async fn create_auction_with_default_confirmation<P>(
    provider: &P,
    factory_address: Address,
    seller: Address,
    bidding_time_seconds: U256,
    starting_bid_wei: U256,
    title: String,
    description: String,
) -> Result<CreateAuctionResult>
where
    P: Provider + ?Sized,
{
    let default_confirmation_window = U256::from(259_200u64);

    create_auction(
        provider,
        factory_address,
        seller,
        bidding_time_seconds,
        starting_bid_wei,
        default_confirmation_window,
        title,
        description,
    )
    .await
}