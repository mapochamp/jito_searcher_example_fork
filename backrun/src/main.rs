mod event_loops;

use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    path::PathBuf,
    result,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant}
};

use clap::Parser;
use env_logger::TimestampPrecision;
use histogram::Histogram;
use jito_protos::{
    bundle::{Bundle, BundleResult, bundle_result::Result as BundleResultEnum, rejected::Reason},
    searcher::{
        searcher_service_client::SearcherServiceClient,
        ConnectedLeadersRequest, SendBundleResponse, NextScheduledLeaderRequest,
    },
    shared::Header,
    packet::Packet,
};
use jito_searcher_client::{
    get_searcher_client_auth, get_searcher_client_no_auth,
    BlockEngineConnectionError, SendBundleOptions,
    send_bundle_with_opts,
};
use log::*;
use rand::{thread_rng, Rng};
use solana_client::{
    client_error::ClientError,
    nonblocking::{pubsub_client::PubsubClientError, rpc_client::RpcClient},
    rpc_response,
    rpc_response::RpcBlockUpdate,
};
use solana_metrics::{datapoint_info, set_host_id};
use solana_sdk::{
    clock::Slot,
    commitment_config::{CommitmentConfig, CommitmentLevel},
    hash::Hash,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signature, Signer},
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction}
};
use spl_memo::build_memo;
use thiserror::Error;
use tokio::{
    runtime::Builder,
    sync::mpsc::{channel, Receiver},
    time::interval,
};
use tonic::{
    codegen::{Body, Bytes, StdError},
    Response, Status,
};

use crate::event_loops::{
    block_subscribe_loop, bundle_results_loop, auction_monitor_loop, 
    slot_subscribe_loop, AuctionStateWrapper,
};

#[derive(Debug, Error)]
enum BackrunError {
    #[error("TonicError: {0}")]
    TonicError(#[from] tonic::transport::Error),
    
    #[error("GrpcError: {0}")]
    GrpcError(#[from] Status),
    
    #[error("RpcError: {0}")]
    RpcError(#[from] ClientError),
    
    #[error("PubSubError: {0}")]
    PubSubError(#[from] PubsubClientError),
    
    #[error("BlockEngineConnectionError: {0}")]
    BlockEngineConnectionError(#[from] BlockEngineConnectionError),
    
    #[error("Shutdown")]
    Shutdown,
}

// Rest of the code...
