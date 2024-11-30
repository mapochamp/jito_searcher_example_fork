use std::time::Duration;

use futures_util::StreamExt;
use jito_protos::{
    bundle::BundleResult,
    searcher::{
        searcher_service_client::SearcherServiceClient,
        SubscribeBundleResultsRequest,
        GetRegionsRequest, GetAuctionStateRequest,
        AuctionState,
    },
};
use log::*;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
    rpc_response,
    rpc_response::{RpcBlockUpdate, SlotUpdate},
};
use solana_metrics::{datapoint_error, datapoint_info};
use solana_sdk::{
    clock::Slot,
    commitment_config::{CommitmentConfig, CommitmentLevel},
};
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};
use tokio::{sync::mpsc::Sender, time::sleep};
use tonic::{
    codegen::{Body, Bytes, StdError},
    Request,
};

#[derive(Debug, Clone)]
pub struct AuctionStateWrapper {
    pub current_slot: u64,
    pub next_auction_slot: u64,
    pub min_bid_lamports: u64,
    pub tip_accounts: Vec<String>,
    pub region: String,
}

impl From<AuctionState> for AuctionStateWrapper {
    fn from(state: AuctionState) -> Self {
        Self {
            current_slot: state.current_slot,
            next_auction_slot: state.next_auction_slot,
            min_bid_lamports: state.min_bid_lamports,
            tip_accounts: state.tip_accounts,
            region: state.region,
        }
    }
}

pub async fn auction_monitor_loop<T>(
    mut searcher_client: SearcherServiceClient<T>,
    auction_sender: Sender<AuctionStateWrapper>,
    regions: Vec<String>,
) where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + 'static + Clone,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::Future: std::marker::Send,
{
    let mut errors: usize = 0;
    
    // Get available regions
    match searcher_client.get_regions(Request::new(GetRegionsRequest {})).await {
        Ok(response) => {
            let regions_resp = response.into_inner();
            info!(
                "Connected to region: {}, Available regions: {:?}",
                regions_resp.current_region,
                regions_resp.available_regions
            );
        }
        Err(e) => {
            error!("Failed to get regions: {}", e);
        }
    }

    loop {
        sleep(Duration::from_millis(200)).await; // More frequent checks for auctions

        // Get current auction state
        match searcher_client
            .get_auction_state(Request::new(GetAuctionStateRequest { regions: regions.clone() }))
            .await 
        {
            Ok(response) => {
                let auction_state = response.into_inner().state;
                if auction_state.is_none() {
                    continue;
                }
                let auction_state = auction_state.unwrap();
                
                let slots_until_next = auction_state.next_auction_slot - auction_state.current_slot;
                
                // If we're within 2 slots of the next auction, prepare
                if slots_until_next <= 2 {
                    info!(
                        "Preparing for auction: slot {}, min bid {} lamports, region {}",
                        auction_state.next_auction_slot,
                        auction_state.min_bid_lamports,
                        auction_state.region,
                    );
                    
                    if let Err(e) = auction_sender.send(auction_state.into()).await {
                        error!("Failed to send auction state: {}", e);
                        datapoint_error!(
                            "auction_send_error",
                            ("errors", 1, i64),
                            ("error_str", e.to_string(), String)
                        );
                    }
                }
            }
            Err(e) => {
                errors += 1;
                datapoint_error!(
                    "auction_monitor_error",
                    ("errors", errors, i64),
                    ("error_str", e.to_string(), String)
                );
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

// slot update subscription loop that attempts to maintain a connection to an RPC server
pub async fn slot_subscribe_loop(pubsub_addr: String, slot_sender: Sender<Slot>) {
    let mut connect_errors: u64 = 0;
    let mut slot_subscribe_errors: u64 = 0;
    let mut slot_subscribe_disconnect_errors: u64 = 0;

    loop {
        sleep(Duration::from_secs(1)).await;

        match PubsubClient::new(&pubsub_addr).await {
            Ok(pubsub_client) => match pubsub_client.slot_updates_subscribe().await {
                Ok((mut slot_update_subscription, _unsubscribe_fn)) => {
                    while let Some(slot_update) = slot_update_subscription.next().await {
                        if let SlotUpdate::FirstShredReceived { slot, timestamp: _ } = slot_update {
                            datapoint_info!("slot_subscribe_slot", ("slot", slot, i64));
                            if slot_sender.send(slot).await.is_err() {
                                datapoint_error!("slot_subscribe_send_error", ("errors", 1, i64));
                                return;
                            }
                        }
                    }
                    slot_subscribe_disconnect_errors += 1;
                    datapoint_error!(
                        "slot_subscribe_disconnect_error",
                        ("errors", slot_subscribe_disconnect_errors, i64)
                    );
                }
                Err(e) => {
                    slot_subscribe_errors += 1;
                    datapoint_error!(
                        "slot_subscribe_error",
                        ("errors", slot_subscribe_errors, i64),
                        ("error_str", e.to_string(), String),
                    );
                }
            },
            Err(e) => {
                connect_errors += 1;
                datapoint_error!(
                    "slot_subscribe_pubsub_connect_error",
                    ("errors", connect_errors, i64),
                    ("error_str", e.to_string(), String)
                );
            }
        }
    }
}

// block subscription loop that attempts to maintain a connection to an RPC server
pub async fn block_subscribe_loop(
    pubsub_addr: String,
    block_receiver: Sender<rpc_response::Response<RpcBlockUpdate>>,
) {
    let mut connect_errors: u64 = 0;
    let mut block_subscribe_errors: u64 = 0;
    let mut block_subscribe_disconnect_errors: u64 = 0;

    loop {
        sleep(Duration::from_secs(1)).await;

        match PubsubClient::new(&pubsub_addr).await {
            Ok(pubsub_client) => match pubsub_client
                .block_subscribe(
                    RpcBlockSubscribeFilter::All,
                    Some(RpcBlockSubscribeConfig {
                        commitment: Some(CommitmentConfig {
                            commitment: CommitmentLevel::Confirmed,
                        }),
                        encoding: Some(UiTransactionEncoding::Base64),
                        transaction_details: Some(TransactionDetails::Signatures),
                        show_rewards: Some(true),
                        max_supported_transaction_version: None,
                    }),
                )
                .await
            {
                Ok((mut block_update_subscription, _unsubscribe_fn)) => {
                    while let Some(block_update) = block_update_subscription.next().await {
                        datapoint_info!(
                            "block_subscribe_slot",
                            ("slot", block_update.context.slot, i64)
                        );
                        if block_receiver.send(block_update).await.is_err() {
                            datapoint_error!("block_subscribe_send_error", ("errors", 1, i64));
                            return;
                        }
                    }
                    block_subscribe_disconnect_errors += 1;
                    datapoint_error!(
                        "block_subscribe_disconnect_error",
                        ("errors", block_subscribe_disconnect_errors, i64)
                    );
                }
                Err(e) => {
                    block_subscribe_errors += 1;
                    datapoint_error!(
                        "block_subscribe_error",
                        ("errors", block_subscribe_errors, i64),
                        ("error_str", e.to_string(), String),
                    );
                }
            },
            Err(e) => {
                connect_errors += 1;
                datapoint_error!(
                    "block_subscribe_pubsub_connect_error",
                    ("errors", connect_errors, i64),
                    ("error_str", e.to_string(), String)
                );
            }
        }
    }
}

pub async fn bundle_results_loop<T>(
    mut searcher_client: SearcherServiceClient<T>,
    bundle_results_sender: Sender<BundleResult>,
) where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + 'static + Clone,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::Future: std::marker::Send,
{
    let mut errors: usize = 0;

    loop {
        sleep(Duration::from_millis(1000)).await;
        match searcher_client
            .subscribe_bundle_results(Request::new(SubscribeBundleResultsRequest {}))
            .await
        {
            Ok(resp) => {
                let mut stream = resp.into_inner();
                while let Some(result) = stream.next().await {
                    match result {
                        Ok(bundle_result) => {
                            if bundle_results_sender.send(bundle_result).await.is_err() {
                                return;
                            }
                        }
                        Err(e) => {
                            errors += 1;
                            datapoint_error!(
                                "bundle_results_error",
                                ("errors", errors, i64),
                                ("error_str", e.to_string(), String)
                            );
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                errors += 1;
                datapoint_error!(
                    "bundle_results_subscription_error",
                    ("errors", errors, i64),
                    ("error_str", e.to_string(), String)
                );
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
