mod event_loops;

use std::{
    collections::{hash_map::Entry, HashMap, HashSet}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
},
    path::PathBuf,
    result,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
},
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
};

use clap::Parser;
use env_logger::TimestampPrecision;
use histogram::Histogram;
use jito_protos::{
    bundle::{Bundle, BundleResult, bundle_result::Result as BundleResultEnum, rejected::Reason}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
},
    searcher::{
        searcher_service_client::SearcherServiceClient,
        ConnectedLeadersRequest, SendBundleResponse, NextScheduledLeaderRequest,
    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
},
    shared::Header,
    packet::Packet,
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
};
use jito_searcher_client::{
    get_searcher_client_auth, get_searcher_client_no_auth,
    BlockEngineConnectionError, SendBundleOptions,
    send_bundle_with_opts,
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
};
use log::*;
use rand::{thread_rng, Rng}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
};
use solana_client::{
    client_error::ClientError,
    nonblocking::{pubsub_client::PubsubClientError, rpc_client::RpcClient}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
},
    rpc_response,
    rpc_response::RpcBlockUpdate,
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
};
use solana_metrics::{datapoint_info, set_host_id}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
};
use solana_sdk::{
    clock::Slot,
    commitment_config::{CommitmentConfig, CommitmentLevel}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
},
    hash::Hash,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signature, Signer}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
},
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
},
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
};
use spl_memo::build_memo;
use thiserror::Error;
use tokio::{
    runtime::Builder,
    sync::mpsc::{channel, Receiver}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
},
    time::interval,
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
};
use tonic::{
    codegen::{Body, Bytes, StdError}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
},
    Response, Status,
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
};

use crate::event_loops::{
    block_subscribe_loop, bundle_results_loop, auction_monitor_loop, 
    slot_subscribe_loop, AuctionStateWrapper,
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// URL of the block engine.
    #[arg(long, env)]
    block_engine_url: String,

    /// Account pubkeys to backrun
    #[arg(long, env)]
    backrun_accounts: Vec<Pubkey>,

    /// Path to keypair file used to sign and pay for transactions
    #[arg(long, env)]
    payer_keypair: PathBuf,

    /// Path to keypair file used to authenticate with the Jito Block Engine
    #[arg(long, env)]
    auth_keypair: Option<PathBuf>,

    /// RPC Websocket URL.
    #[arg(long, env)]
    pubsub_url: String,

    /// RPC HTTP URL.
    #[arg(long, env)]
    rpc_url: String,

    /// Message to pass into the memo program as part of a bundle.
    #[arg(long, env, default_value = "jito backrun")]
    message: String,

    /// Minimum tip amount in lamports
    #[arg(long, env, default_value = "100000")]
    min_tip_amount: u64,
    
    /// Maximum tip amount in lamports
    #[arg(long, env, default_value = "1000000")]
    max_tip_amount: u64,
    
    /// Number of slots to look ahead for auctions
    #[arg(long, env, default_value = "2")]
    auction_lookahead_slots: u64,

    /// Region to connect to (amsterdam, frankfurt, ny, tokyo, slc)
    #[arg(long, env, default_value = "amsterdam")]
    region: String,

    /// Comma-separated list of regions to request cross-region data from.
    #[arg(long, env, value_delimiter = ',')]
    regions: Vec<String>,

    /// Subscribe and print bundle results.
    #[arg(long, env, default_value_t = true)]
    subscribe_bundle_results: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

#[derive(Debug, Error)]
#[allow(dead_code)]
enum BackrunError {
    #[error("TonicError {0}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}")]
    TonicError(#[from] tonic::transport::Error),
    #[error("GrpcError {0}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}")]
    GrpcError(#[from] Status),
    #[error("RpcError {0}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}")]
    RpcError(#[from] ClientError),
    #[error("PubSubError {0}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}")]
    PubSubError(#[from] PubsubClientError),
    #[error("BlockEngineConnectionError {0}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}")]
    BlockEngineConnectionError(#[from] BlockEngineConnectionError),
    #[error("Shutdown")]
    Shutdown,
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct AuctionBundle {
    transactions: Vec<VersionedTransaction>,
    tip_amount: u64,
    target_slot: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

#[derive(Default)]
struct BlockStats {
    bundles_sent: Vec<(AuctionBundle, tonic::Result<Response<SendBundleResponse>>)>,
    send_elapsed: u64,
    send_rt_per_packet: Histogram,
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

type Result<T> = result::Result<T, BackrunError>;

fn generate_tip_accounts() -> Vec<Pubkey> {
    vec![
        "96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5",
        "HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe", 
        "Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY",
        "ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49",
        "DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh",
        "ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt",
        "DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL", 
        "3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT"
    ].iter()
    .map(|s| Pubkey::from_str(s).unwrap())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

async fn prepare_auction_bundle(
    transactions: Vec<VersionedTransaction>,
    tip_amount: u64,
    tip_account: &Pubkey,
    payer: &Keypair,
    blockhash: Hash,
) -> Result<Bundle> {
    let tip_tx = Transaction::new_signed_with_payer(
        &[
            transfer(&payer.pubkey(), tip_account, tip_amount),
            build_memo(format!("jito tip: {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}", tip_amount).as_bytes(), &[]),
        ],
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    let packets = transactions.iter()
        .chain(std::iter::once(&VersionedTransaction::from(tip_tx)))
        .map(|tx| {
            let mut packet = Packet::default();
            packet.data = bincode::serialize(tx).unwrap();
            packet.meta = Some(Default::default());
            packet
        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
})
        .collect();

    Ok(Bundle {
        header: Some(Header {
            bundle_only: true,
            ..Default::default()
        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}),
        packets,
    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
})
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

async fn maintenance_tick<T>(
    searcher_client: &mut SearcherServiceClient<T>,
    rpc_client: &RpcClient,
    leader_schedule: &mut HashMap<Pubkey, HashSet<Slot>>,
    blockhash: &mut Hash,
    regions: Vec<String>,
) -> Result<()>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + 'static + Clone,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::Future: std::marker::Send,
{
    *blockhash = rpc_client
        .get_latest_blockhash_with_commitment(CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
})
        .await?
        .0;
    let new_leader_schedule = searcher_client
        .get_connected_leaders(ConnectedLeadersRequest {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
})
        .await?
        .into_inner()
        .connected_validators
        .iter()
        .fold(HashMap::new(), |mut hmap, (pubkey, slot_list)| {
            hmap.insert(
                Pubkey::from_str(pubkey).unwrap(),
                slot_list.slots.iter().cloned().collect(),
            );
            hmap
        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
});
    if new_leader_schedule != *leader_schedule {
        info!("connected_validators: {:?}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}", new_leader_schedule.keys());
        *leader_schedule = new_leader_schedule;
    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

    let next_scheduled_leader = searcher_client
        .get_next_scheduled_leader(NextScheduledLeaderRequest { regions }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
})
        .await?
        .into_inner();
    info!(
        "next_scheduled_leader: {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
} in {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
} slots from {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}",
        next_scheduled_leader.next_leader_identity,
        next_scheduled_leader.next_leader_slot - next_scheduled_leader.current_slot,
        next_scheduled_leader.next_leader_region
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

fn print_block_stats(
    block_stats: &mut HashMap<Slot, BlockStats>,
    block: rpc_response::Response<RpcBlockUpdate>,
    leader_schedule: &HashMap<Pubkey, HashSet<Slot>>,
    block_signatures: &mut HashMap<Slot, HashSet<Signature>>,
) {
    const KEEP_SIGS_SLOTS: u64 = 20;

    if let Some(stats) = block_stats.get(&block.context.slot) {
        datapoint_info!(
            "bundles-sent",
            ("slot", block.context.slot, i64),
            ("bundles", stats.bundles_sent.len(), i64),
            ("total_send_elapsed_us", stats.send_elapsed, i64),
            (
                "sent_rt_pp_min",
                stats.send_rt_per_packet.minimum().unwrap_or_default(),
                i64
            ),
            (
                "sent_rt_pp_max",
                stats.send_rt_per_packet.maximum().unwrap_or_default(),
                i64
            ),
            (
                "sent_rt_pp_avg",
                stats.send_rt_per_packet.mean().unwrap_or_default(),
                i64
            ),
            (
                "sent_rt_pp_p50",
                stats
                    .send_rt_per_packet
                    .percentile(50.0)
                    .unwrap_or_default(),
                i64
            ),
            (
                "sent_rt_pp_p90",
                stats
                    .send_rt_per_packet
                    .percentile(90.0)
                    .unwrap_or_default(),
                i64
            ),
            (
                "sent_rt_pp_p95",
                stats
                    .send_rt_per_packet
                    .percentile(95.0)
                    .unwrap_or_default(),
                i64
            ),
        );
    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

    let maybe_leader = leader_schedule
        .iter()
        .find(|(_, slots)| slots.contains(&block.context.slot))
        .map(|(leader, _)| leader);

    if let Some(b) = &block.value.block {
        if let Some(sigs) = &b.signatures {
            let block_signatures_set: HashSet<Signature> = sigs
                .iter()
                .map(|s| Signature::from_str(s).unwrap())
                .collect();

            if let Some(leader) = maybe_leader {
                let bundles_sent_before_slot: HashMap<Slot, &[(AuctionBundle, tonic::Result<Response<SendBundleResponse>>)]> = 
                    block_stats
                        .iter()
                        .filter(|(slot, _)| **slot <= block.context.slot)
                        .map(|(slot, stats)| (*slot, stats.bundles_sent.as_slice()))
                        .collect();

                let num_bundles_sent: usize = bundles_sent_before_slot
                    .values()
                    .map(|bundles_sent| bundles_sent.len())
                    .sum();

                let num_bundles_sent_ok: usize = bundles_sent_before_slot
                    .values()
                    .map(|bundles_sent| {
                        bundles_sent
                            .iter()
                            .filter(|(_, send_response)| send_response.is_ok())
                            .count()
                    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
})
                    .sum();

                datapoint_info!(
                    "leader-bundle-stats",
                    ("slot", block.context.slot, i64),
                    ("leader", leader.to_string(), String),
                    ("block_txs", block_signatures_set.len(), i64),
                    ("num_bundles_sent", num_bundles_sent, i64),
                    ("num_bundles_sent_ok", num_bundles_sent_ok, i64),
                    (
                        "num_bundles_sent_err",
                        num_bundles_sent - num_bundles_sent_ok,
                        i64
                    ),
                );

                if block.context.slot % 4 == 3 {
                    block_stats.clear();
                }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
            }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

    if let Some(b) = &block.value.block {
        if let Some(sigs) = &b.signatures {
            block_signatures.insert(
                block.context.slot,
                sigs.iter()
                    .map(|s| Signature::from_str(s).unwrap())
                    .collect(),
            );
        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

    block_signatures.retain(|slot, _| *slot > block.context.slot - KEEP_SIGS_SLOTS);
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
async fn run_searcher_loop<T>(
    mut searcher_client: SearcherServiceClient<T>,
    keypair: &Keypair,
    rpc_url: String,
    regions: Vec<String>,
    min_tip_amount: u64,
    max_tip_amount: u64,
    mut slot_receiver: Receiver<Slot>,
    mut block_receiver: Receiver<rpc_response::Response<RpcBlockUpdate>>,
    mut bundle_results_receiver: Receiver<BundleResult>,
    mut auction_receiver: Receiver<AuctionStateWrapper>,
) -> Result<()>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + 'static + Clone,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::Future: std::marker::Send,
{
    let mut leader_schedule: HashMap<Pubkey, HashSet<Slot>> = HashMap::new();
    let mut block_stats: HashMap<Slot, BlockStats> = HashMap::new();
    let mut block_signatures: HashMap<Slot, HashSet<Signature>> = HashMap::new();
    let mut _highest_slot = 0;  // Mark as unused since we're not using it yet
    let mut _is_leader_slot = false;  // Mark as unused since we're not using it yet

    let mut rng = thread_rng();
    let tip_accounts = generate_tip_accounts();
    info!("tip accounts: {:?}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}", tip_accounts);

    let rpc_client = RpcClient::new(rpc_url);
    let mut blockhash = rpc_client
        .get_latest_blockhash_with_commitment(CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
})
        .await?
        .0;

    let mut tick = interval(Duration::from_secs(5));
    loop {
        tokio::select! {
            _ = tick.tick() => {
                maintenance_tick(&mut searcher_client, &rpc_client, &mut leader_schedule, &mut blockhash, regions.clone()).await?;
            }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
            Some(bundle_result) = bundle_results_receiver.recv() => {
                match &bundle_result.result {
                    Some(BundleResultEnum::Rejected(rejected)) => {
                        match &rejected.reason {
                            Some(Reason::StateAuctionBidRejected(r)) => {
                                warn!(
                                    "Bundle {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
} rejected from state auction: need {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
} lamports",
                                    bundle_result.bundle_id, r.simulated_bid_lamports
                                );
                            }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                            Some(Reason::WinningBatchBidRejected(r)) => {
                                warn!(
                                    "Bundle {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
} won state auction but rejected from batch {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}: {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
} lamports",
                                    bundle_result.bundle_id, r.auction_id, r.simulated_bid_lamports
                                );
                            }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                            Some(Reason::SimulationFailure(r)) => {
                                error!(
                                    "Bundle {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
} failed simulation at tx {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}: {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}",
                                    bundle_result.bundle_id, r.tx_signature, r.msg
                                );
                            }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                            Some(Reason::DroppedBundle(r)) => {
                                warn!("Bundle {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
} dropped: {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}", bundle_result.bundle_id, r.msg);
                            }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                            _ => {
                                error!("Bundle {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
} rejected with unknown reason", bundle_result.bundle_id);
                            }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                    Some(BundleResultEnum::Accepted(accepted)) => {
                        info!(
                            "Bundle accepted for slot {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
} by validator {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}",
                            accepted.slot, accepted.validator_identity
                        );
                    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                    Some(BundleResultEnum::Processed(processed)) => {
                        info!(
                            "Bundle processed in slot {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
} by {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
} at index {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}",
                            processed.slot, processed.validator_identity, processed.bundle_index
                        );
                    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                    Some(BundleResultEnum::Finalized(_)) => {
                        info!("Bundle {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
} finalized", bundle_result.bundle_id);
                    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                    _ => {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
            }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
            Some(auction_state) = auction_receiver.recv() => {
                let tip_amount = auction_state.min_bid_lamports.max(min_tip_amount)
                    .min(max_tip_amount);
                
                let tip_account = if !auction_state.tip_accounts.is_empty() {
                    Pubkey::from_str(&auction_state.tip_accounts[rng.gen_range(0..auction_state.tip_accounts.len())])
                        .unwrap()
                }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
} else {
                    tip_accounts[rng.gen_range(0..tip_accounts.len())]
                }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
};
                
                if let Ok(bundle) = prepare_auction_bundle(
                    vec![],
                    tip_amount,
                    &tip_account,
                    keypair,
                    blockhash,
                ).await {
                    let now = Instant::now();
                    let opts = SendBundleOptions {
                        bundle_only: true,
                        valid_until_slot: Some(auction_state.next_auction_slot + 1),
                    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
};
                    
                    match send_bundle_with_opts(&mut searcher_client, bundle, opts).await {
                        Ok(response) => {
                            let send_elapsed = now.elapsed().as_micros() as u64;
                            match block_stats.entry(auction_state.next_auction_slot) {
                                Entry::Occupied(mut entry) => {
                                    let stats = entry.get_mut();
                                    stats.bundles_sent.push((
                                        AuctionBundle {
                                            transactions: vec![],
                                            tip_amount,
                                            target_slot: auction_state.next_auction_slot,
                                        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
},
                                        Ok(response),
                                    ));
                                    stats.send_elapsed += send_elapsed;
                                    let _ = stats.send_rt_per_packet.increment(send_elapsed);
                                }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                                Entry::Vacant(entry) => {
                                    let mut send_rt_per_packet = Histogram::new();
                                    let _ = send_rt_per_packet.increment(send_elapsed);
                                    entry.insert(BlockStats {
                                        bundles_sent: vec![(
                                            AuctionBundle {
                                                transactions: vec![],
                                                tip_amount,
                                                target_slot: auction_state.next_auction_slot,
                                            }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
},
                                            Ok(response),
                                        )],
                                        send_elapsed,
                                        send_rt_per_packet,
                                    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
});
                                }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                            }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                        Err(e) => {
                            error!("Failed to submit to auction: {}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}", e);
                        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
                }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
            }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
            Some(slot) = slot_receiver.recv() => {
                _highest_slot = slot;
                _is_leader_slot = leader_schedule.iter().any(|(_, slots)| slots.contains(&_highest_slot));
            }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
            Some(block) = block_receiver.recv() => {
                print_block_stats(&mut block_stats, block, &leader_schedule, &mut block_signatures);
            }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::builder()
        .format_timestamp(Some(TimestampPrecision::Micros))
        .init();
    let args: Args = Args::parse();

    let payer_keypair = Arc::new(read_keypair_file(&args.payer_keypair).expect("parse kp file"));
    let auth_keypair = args
        .auth_keypair
        .as_ref()
        .map(|path| Arc::new(read_keypair_file(path).expect("parse kp file")));

    set_host_id(
        auth_keypair
            .as_ref()
            .map(|kp| kp.pubkey().to_string())
            .unwrap_or(uuid::Uuid::new_v4().to_string()),
    );

    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    match auth_keypair {
        Some(auth_keypair) => {
            let searcher_client_auth = runtime.block_on(
            get_searcher_client_auth(
                args.block_engine_url.as_str(),
                &auth_keypair,
            ))
            .expect("Failed to get searcher client with auth. Note: If you don't pass in the auth keypair, we can attempt to connect to the no auth endpoint");
            start_searcher_loop(runtime, searcher_client_auth, &payer_keypair, args)
        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
        None => {
            let searcher_client_no_auth = runtime.block_on(
                get_searcher_client_no_auth(
                    args.block_engine_url.as_str(),
                ))
                .expect("Failed to get searcher client with auth. Note: If you don't pass in the auth keypair, we can attempt to connect to the no auth endpoint");
            start_searcher_loop(runtime, searcher_client_no_auth, &payer_keypair, args)
        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn start_searcher_loop<T>(
    runtime: tokio::runtime::Runtime,
    searcher_client: SearcherServiceClient<T>,
    payer_keypair: &Keypair,
    args: Args,
) -> Result<()>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + 'static + Clone,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::Future: std::marker::Send,
{
    runtime.block_on(async move {
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));

        if args.subscribe_bundle_results {
            tokio::spawn(bundle_results_loop(
                searcher_client.clone(),
                bundle_results_sender,
            ));
        }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}

        let result = run_searcher_loop(
            searcher_client,
            payer_keypair,
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        )
        .await;
        error!("searcher loop exited result: {result:?}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}");

        Ok(())
    }

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
})
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        system_program,
    };

    #[tokio::test]
    async fn test_searcher_testnet() -> Result<()> {
        // Setup test parameters
        let args = Args {
            block_engine_url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
            backrun_accounts: vec![], // No specific accounts to backrun for this test
            payer_keypair: PathBuf::from("test_keypair.json"), // We'll create this
            auth_keypair: None, // No auth needed for testnet
            pubsub_url: "wss://api.testnet.solana.com".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
            message: "test backrun".to_string(),
            min_tip_amount: 1_000,
            max_tip_amount: 100_000,
            auction_lookahead_slots: 2,
            region: "dallas".to_string(),
            regions: vec!["dallas".to_string()],
            subscribe_bundle_results: true,
        };

        // Create a test keypair
        let payer = Keypair::new();
        // Save it for the test
        std::fs::write(
            "test_keypair.json",
            serde_json::to_string(&Vec::from(payer.to_bytes()))?,
        )?;

        // Request some test SOL (in real usage, you'd need to fund this account)
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        println!("Fund this address for test: {}", payer.pubkey());

        // Connect to block engine
        let searcher_client = get_searcher_client_no_auth(&args.block_engine_url).await?;

        // Create channels
        let (slot_sender, slot_receiver) = channel(100);
        let (block_sender, block_receiver) = channel(100);
        let (bundle_results_sender, bundle_results_receiver) = channel(100);
        let (auction_sender, auction_receiver) = channel(100);

        // Start the event loops
        let slot_handle = tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
        let block_handle = tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
        let auction_handle = tokio::spawn(auction_monitor_loop(
            searcher_client.clone(),
            auction_sender,
            args.regions.clone(),
        ));
        let bundle_results_handle = tokio::spawn(bundle_results_loop(
            searcher_client.clone(),
            bundle_results_sender,
        ));

        // Run the searcher loop for a short time
        let searcher_handle = tokio::spawn(run_searcher_loop(
            searcher_client,
            &Arc::new(payer),
            args.rpc_url,
            args.regions,
            args.min_tip_amount,
            args.max_tip_amount,
            slot_receiver,
            block_receiver,
            bundle_results_receiver,
            auction_receiver,
        ));

        // Let it run for 30 seconds
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Clean up
        slot_handle.abort();
        block_handle.abort();
        auction_handle.abort();
        bundle_results_handle.abort();
        searcher_handle.abort();

        // Clean up test keypair file
        std::fs::remove_file("test_keypair.json")?;

        Ok(())
    }
}
