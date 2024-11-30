use std::str::FromStr;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
    message::Message,
    pubkey::Pubkey,
};
use jito_protos::{
    searcher::{GetTipAccountsRequest, SendBundleRequest, SubscribeBundleResultsRequest},
    bundle::Bundle,
    shared::Header,
    packet::Packet,
};
use jito_searcher_client::get_searcher_client_no_auth;
use solana_client::nonblocking::rpc_client::RpcClient;
use log::*;
use tonic::Request;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_testnet_bundle_submission() -> Result<()> {
        // Setup logging
        env_logger::init();

        // Connect to testnet
        let block_engine_url = "https://dallas.testnet.block-engine.jito.wtf".to_string();
        let rpc_url = "https://api.testnet.solana.com".to_string();

        info!("Connecting to testnet block engine at {}", block_engine_url);
        info!("Using RPC URL: {}", rpc_url);

        // Setup clients
        let mut searcher_client = get_searcher_client_no_auth(&block_engine_url).await?;
        let rpc_client = RpcClient::new(rpc_url);

        // Create test keypairs
        let payer = Keypair::new();
        let recipient = Keypair::new();

        info!("Created test keypairs:");
        info!("Payer: {}", payer.pubkey());
        info!("Recipient: {}", recipient.pubkey());

        // Request airdrop for payer on testnet
        info!("Requesting airdrop for payer...");
        let airdrop_sig = rpc_client.request_airdrop(&payer.pubkey(), 1_000_000_000).await?;
        rpc_client.confirm_transaction(&airdrop_sig).await?;
        
        let balance = rpc_client.get_balance(&payer.pubkey()).await?;
        info!("Payer balance: {} SOL", balance as f64 / 1_000_000_000.0);

        // Get tip accounts from block engine
        info!("Fetching tip accounts...");
        let tip_accounts_response = searcher_client
            .get_tip_accounts(Request::new(GetTipAccountsRequest {}))
            .await?
            .into_inner();

        let tip_account = Pubkey::from_str(&tip_accounts_response.accounts[0])?;
        info!("Using tip account: {}", tip_account);

        // Get recent blockhash
        let recent_blockhash = rpc_client.get_latest_blockhash().await?;
        info!("Got recent blockhash: {}", recent_blockhash);

        // Create transaction with tip and strategy
        let tip_ix = system_instruction::transfer(
            &payer.pubkey(),
            &tip_account,
            1_000 // Minimum tip amount
        );
        
        let strategy_ix = system_instruction::transfer(
            &payer.pubkey(),
            &recipient.pubkey(),
            10_000
        );

        let message = Message::new(
            &[tip_ix, strategy_ix],
            Some(&payer.pubkey())
        );
        
        let tx = Transaction::new(
            &[&payer],
            message,
            recent_blockhash
        );

        info!("Created transaction with tip and strategy");

        // Convert transaction to packet
        let packet = Packet {
            data: bincode::serialize(&tx)?.to_vec(),
            meta: None,
        };

        // Create and verify bundle
        let bundle = Bundle {
            header: Some(Header {
                minimum_tip: 1_000,
                bundle_only: true,
                valid_until_slot: 0, // Current slot + some offset
                ts: None,
            }),
            packets: vec![packet],
        };

        info!("Created bundle with {} packets", bundle.packets.len());

        // Submit bundle to testnet
        let bundle_request = SendBundleRequest {
            bundle: Some(bundle),
        };

        info!("Submitting bundle to testnet...");
        match searcher_client.send_bundle(Request::new(bundle_request)).await {
            Ok(response) => {
                info!("Bundle submission successful");
                let response = response.into_inner();
                info!("Bundle UUID: {}", response.uuid);
                
                // Monitor bundle status using get_bundle_results
                info!("Checking bundle status...");
                let mut bundle_results = searcher_client
                    .subscribe_bundle_results(Request::new(SubscribeBundleResultsRequest {}))
                    .await?
                    .into_inner();

                // Wait for a few seconds to get bundle results
                use tokio::time::{sleep, Duration};
                sleep(Duration::from_secs(5)).await;
                
                while let Some(result) = bundle_results.message().await? {
                    info!("Bundle result: {:?}", result);
                }
            },
            Err(e) => {
                error!("Bundle submission failed: {}", e);
                return Err(e.into());
            }
        }

        Ok(())
    }
}
