use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use futures_util::StreamExt;
use jito_protos::{
    auth::{auth_service_client::AuthServiceClient, Role},
    bundle::{
        bundle_result::Result as BundleResultType, rejected::Reason, Accepted, Bundle,
        BundleResult, InternalError, SimulationFailure, StateAuctionBidRejected,
        WinningBatchBidRejected,
    },
    convert::proto_packet_from_versioned_tx,
    searcher::{
        searcher_service_client::SearcherServiceClient, SendBundleRequest, SendBundleResponse,
    },
};
use log::{info, warn};
use solana_client::nonblocking::rpc_client::RpcClient;
use std::str::FromStr;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    system_instruction,
    transaction::{Transaction, VersionedTransaction},
};
use jito_protos::shared::Header;
use thiserror::Error;
use tokio::time::timeout;
use tonic::{
    codegen::{Body, Bytes, InterceptedService, StdError},
    transport,
    transport::{Channel, Endpoint},
    Response, Status, Streaming,
};

use crate::token_authenticator::ClientInterceptor;

pub mod token_authenticator;

#[derive(Debug, Error)]
pub enum BlockEngineConnectionError {
    #[error("transport error {0}")]
    TransportError(#[from] transport::Error),
    #[error("client error {0}")]
    ClientError(#[from] Status),
}

#[derive(Debug, Error)]
pub enum BundleRejectionError {
    #[error("bundle lost state auction, auction: {0}, tip {1} lamports")]
    StateAuctionBidRejected(String, u64),
    #[error("bundle won state auction but failed global auction, auction {0}, tip {1} lamports")]
    WinningBatchBidRejected(String, u64),
    #[error("bundle simulation failure on tx {0}, message: {1}")]
    SimulationFailure(String, String),
    #[error("internal error {0}")]
    InternalError(String),
}

#[derive(Debug)]
pub struct SendBundleOptions {
    pub bundle_only: bool,
    pub valid_until_slot: Option<u64>,
}

pub type BlockEngineConnectionResult<T> = Result<T, BlockEngineConnectionError>;

pub async fn get_searcher_client_auth(
    block_engine_url: &str,
    auth_keypair: &Arc<Keypair>,
) -> BlockEngineConnectionResult<
    SearcherServiceClient<InterceptedService<Channel, ClientInterceptor>>,
> {
    let auth_channel = create_grpc_channel(block_engine_url).await?;
    let client_interceptor = ClientInterceptor::new(
        AuthServiceClient::new(auth_channel),
        auth_keypair,
        Role::Searcher,
    )
    .await?;

    let searcher_channel = create_grpc_channel(block_engine_url).await?;
    let searcher_client =
        SearcherServiceClient::with_interceptor(searcher_channel, client_interceptor);
    Ok(searcher_client)
}

pub async fn get_searcher_client_no_auth(
    block_engine_url: &str,
) -> BlockEngineConnectionResult<SearcherServiceClient<Channel>> {
    let searcher_channel = create_grpc_channel(block_engine_url).await?;
    let searcher_client = SearcherServiceClient::new(searcher_channel);
    Ok(searcher_client)
}

pub async fn create_grpc_channel(url: &str) -> BlockEngineConnectionResult<Channel> {
    let mut endpoint = Endpoint::from_shared(url.to_string()).expect("invalid url");
    if url.starts_with("https") {
        endpoint = endpoint.tls_config(tonic::transport::ClientTlsConfig::new())?;
    }
    Ok(endpoint.connect().await?)
}

pub async fn send_bundle_with_confirmation<T>(
    transactions: &[VersionedTransaction],
    rpc_client: &RpcClient,
    searcher_client: &mut SearcherServiceClient<T>,
    bundle_results_subscription: &mut Streaming<BundleResult>,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + 'static + Clone,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::Future: std::marker::Send,
{
    let bundle_signatures: Vec<Signature> =
        transactions.iter().map(|tx| tx.signatures[0]).collect();

    let result = send_bundle_no_wait(transactions, searcher_client).await?;

    // grab uuid from block engine + wait for results
    let uuid = result.into_inner().uuid;
    info!("Bundle sent. UUID: {:?}", uuid);

    info!("Waiting for 5 seconds to hear results...");
    let mut time_left = 5000;
    while let Ok(Some(Ok(results))) = timeout(
        Duration::from_millis(time_left),
        bundle_results_subscription.next(),
    )
    .await
    {
        let instant = Instant::now();
        info!("bundle results: {:?}", results);
        match results.result {
            Some(BundleResultType::Accepted(Accepted {
                slot: _s,
                validator_identity: _v,
            })) => {}
            Some(BundleResultType::Rejected(rejected)) => {
                match rejected.reason {
                    Some(Reason::WinningBatchBidRejected(WinningBatchBidRejected {
                        auction_id,
                        simulated_bid_lamports,
                        msg: _,
                    })) => {
                        return Err(Box::new(BundleRejectionError::WinningBatchBidRejected(
                            auction_id,
                            simulated_bid_lamports,
                        )))
                    }
                    Some(Reason::StateAuctionBidRejected(StateAuctionBidRejected {
                        auction_id,
                        simulated_bid_lamports,
                        msg: _,
                    })) => {
                        return Err(Box::new(BundleRejectionError::StateAuctionBidRejected(
                            auction_id,
                            simulated_bid_lamports,
                        )))
                    }
                    Some(Reason::SimulationFailure(SimulationFailure { tx_signature, msg })) => {
                        return Err(Box::new(BundleRejectionError::SimulationFailure(
                            tx_signature,
                            msg,
                        )))
                    }
                    Some(Reason::InternalError(InternalError { msg })) => {
                        return Err(Box::new(BundleRejectionError::InternalError(msg)))
                    }
                    _ => {}
                };
            }
            _ => {}
        }
        time_left -= instant.elapsed().as_millis() as u64;
    }

    let futs: Vec<_> = bundle_signatures
        .iter()
        .map(|sig| {
            rpc_client.get_signature_status_with_commitment(sig, CommitmentConfig::processed())
        })
        .collect();
    let results = futures_util::future::join_all(futs).await;
    if !results.iter().all(|r| matches!(r, Ok(Some(Ok(()))))) {
        warn!("Transactions in bundle did not land");
        return Err(Box::new(BundleRejectionError::InternalError(
            "Searcher service did not provide bundle status in time".into(),
        )));
    }
    info!("Bundle landed successfully");
    let url: String = rpc_client.url();
    let cluster = if url.contains("testnet") {
        "testnet"
    } else if url.contains("devnet") {
        "devnet"
    } else {
        "mainnet"
    };
    for sig in bundle_signatures.iter() {
        info!("https://solscan.io/tx/{}?cluster={}", sig, cluster);
    }
    Ok(())
}

pub async fn send_bundle_with_opts<T>(
    searcher_client: &mut SearcherServiceClient<T>,
    bundle: Bundle,
    opts: SendBundleOptions,
) -> Result<Response<SendBundleResponse>, Status>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + 'static + Clone,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::Future: std::marker::Send,
{
    let mut bundle = bundle;
    if let Some(header) = &mut bundle.header {
        header.bundle_only = opts.bundle_only;
        if let Some(slot) = opts.valid_until_slot {
            header.valid_until_slot = slot;
        }
    }

    searcher_client
        .send_bundle(SendBundleRequest {
            bundle: Some(bundle),
        })
        .await
}

pub async fn send_bundle_no_wait<T>(
    transactions: &[VersionedTransaction],
    searcher_client: &mut SearcherServiceClient<T>,
) -> Result<Response<SendBundleResponse>, Status>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + 'static + Clone,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::Future: std::marker::Send,
{
    // convert them to packets + send over
    let packets: Vec<_> = transactions
        .iter()
        .map(proto_packet_from_versioned_tx)
        .collect();

    searcher_client
        .send_bundle(SendBundleRequest {
            bundle: Some(Bundle {
                header: None,
                packets,
            }),
        })
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_bundle_submission_testnet() -> Result<(), Box<dyn std::error::Error>> {
        // Connect to testnet block engine
        let block_engine_url = "https://dallas.testnet.block-engine.jito.wtf";
        let client = get_searcher_client_no_auth(block_engine_url).await?;

        // Create a test keypair
        let payer = Keypair::new();

        // Create a test transaction (just a simple transfer)
        let transfer_ix = system_instruction::transfer(
            &payer.pubkey(),
            &Pubkey::new_unique(),
            1_000_000, // 0.001 SOL
        );

        // Get tip accounts
        let tip_accounts = vec![
            "96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5",
            "HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe",
        ].iter()
        .map(|s| Pubkey::from_str(s).unwrap())
        .collect::<Vec<_>>();

        // Create tip instruction (minimum 1000 lamports)
        let tip_ix = system_instruction::transfer(
            &payer.pubkey(),
            &tip_accounts[0],
            1_000,
        );

        // Combine transfer and tip in same transaction
        let message = Message::new(
            &[transfer_ix, tip_ix],
            Some(&payer.pubkey()),
        );

        // Create and sign transaction
        let mut tx = Transaction::new_unsigned(message);
        // Note: In real usage, you'd get a recent blockhash from the network
        tx.sign(&[&payer], tx.message.recent_blockhash);
        let versioned_tx = VersionedTransaction::from(tx);

        // Create bundle
        let bundle = Bundle {
            header: Some(Header {
                bundle_only: true,
                ..Default::default()
            }),
            packets: vec![proto_packet_from_versioned_tx(&versioned_tx)],
        };

        // Submit bundle with options
        let opts = SendBundleOptions {
            bundle_only: true,
            valid_until_slot: None, // Let it use default
        };

        let mut client = client;
        let result = send_bundle_with_opts(&mut client, bundle, opts).await?;
        println!("Bundle submission result: {:?}", result);

        Ok(())
    }
}
