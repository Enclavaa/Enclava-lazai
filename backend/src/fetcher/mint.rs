use std::str::FromStr;

use alloy::{
    primitives::{Address, FixedBytes, U256, keccak256},
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
    sol,
    sol_types::SolEvent,
};

use color_eyre::{Result, eyre::Context};
use futures_util::StreamExt;

use crate::{
    config::{APP_CONFIG, ENCLAVA_CONTRACT_ADDRESS},
    helpers::nft::handle_new_nft_mint,
    types::WebAppState,
};

#[derive(Debug, Clone)]
pub struct DatasetNFTMint {
    pub to: Address,
    pub token_id: U256,
    pub dataset_id: String,
    pub tx_hash: Option<FixedBytes<32>>,
}

// Generate strongly typed bindings for your contract events
sol! {
    event DatasetNFTMinted(address indexed to, uint256 indexed tokenId, string datasetId);
    event DatasetUsed(uint256 indexed tokenId, address indexed user, uint256 amount);
    event AmountClaimed(uint256 indexed tokenId, address indexed owner, uint256 amount);
}

pub async fn mint_nft_fetcher(app_state: &WebAppState) -> Result<()> {
    tracing::info!("Starting mint nft fetcher...");

    // Create the provider.
    let rpc_url = &APP_CONFIG.alchemy_rpc_url;

    let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);

    let contract_address = Address::from_str(ENCLAVA_CONTRACT_ADDRESS)?;
    let event_sig = "DatasetNFTMinted(address,uint256,string)";

    let mut last_block = provider.get_block_number().await?;

    loop {
        let current_block = provider.get_block_number().await?;

        if current_block >= last_block {
            let filter = Filter::new()
                .address(contract_address)
                .event(event_sig)
                .from_block(last_block);

            let filtered_logs = provider.get_logs(&filter).await?;

            for log in filtered_logs {
                tracing::info!("New DatasetNFTMinted event: {:?}", log);

                let log_data = log.data();

                let dataset_nft: DatasetNFTMint;

                // Try decode as DatasetNFTMinted
                if let Ok(event) = DatasetNFTMinted::decode_log_data(&log_data) {
                    tracing::trace!(
                        "New NFT Minted! to: {:?}, tokenId: {:?}, datasetId: {}",
                        event.to,
                        event.tokenId,
                        event.datasetId
                    );

                    dataset_nft = DatasetNFTMint {
                        to: event.to,
                        token_id: event.tokenId,
                        dataset_id: event.datasetId,
                        tx_hash: log.transaction_hash,
                    };

                    tracing::info!("DatasetNFTMinted: {:?}", dataset_nft);

                    // Open new thraed that will handle the event(by inserting teh payment details in the database)
                    let app_state = app_state.clone();

                    tokio::spawn(async move {
                        if let Err(e) = handle_new_nft_mint(&app_state, &dataset_nft).await {
                            tracing::error!("Failed to handle new nft mint: {}", e);
                        }
                    });
                }
            }

            last_block = current_block;
        }

        // Sleep for a while before polling again
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

pub async fn _mint_nft_fetcher_ws(app_state: &WebAppState) -> Result<()> {
    tracing::info!("Starting mint nft fetcher...");

    // Create the provider.
    let rpc_url = APP_CONFIG.alchemy_ws_url.clone();
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    let contract_address = Address::from_str(ENCLAVA_CONTRACT_ADDRESS)?;
    let event_sig = "DatasetNFTMinted(address,uint256,string)";

    let filter = Filter::new().address(contract_address).event(event_sig);

    tracing::info!("Starting event monitoring for DatasetNFTMinted...");

    // Subscribe to new Logs matching the filter.
    let sub = provider.subscribe_logs(&filter).await?;

    // Wait and take the next 4 blocks.
    let mut stream = sub.into_stream().take(4);

    while let Some(log) = stream.next().await {
        tracing::info!("New DatasetNFTMinted event: {:?}", log);

        let log_data = log.data();

        let dataset_nft: DatasetNFTMint;

        // Try decode as DatasetNFTMinted
        if let Ok(event) = DatasetNFTMinted::decode_log_data(&log_data) {
            tracing::trace!(
                "New NFT Minted! to: {:?}, tokenId: {:?}, datasetId: {}",
                event.to,
                event.tokenId,
                event.datasetId
            );

            dataset_nft = DatasetNFTMint {
                to: event.to,
                token_id: event.tokenId,
                dataset_id: event.datasetId,
                tx_hash: log.transaction_hash,
            };

            tracing::info!("DatasetNFTMinted: {:?}", dataset_nft);

            // Open new thraed that will handle the event(by inserting teh payment details in the database)
            let app_state = app_state.clone();

            tokio::spawn(async move {
                if let Err(e) = handle_new_nft_mint(&app_state, &dataset_nft).await {
                    tracing::error!("Failed to handle new nft mint: {}", e);
                }
            });
        }
    }

    Ok(())
}
