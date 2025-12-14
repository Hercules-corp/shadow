// Aphrodite - Goddess of Beauty and Collectibles
// Handles NFT operations, metadata, and transfers

use mongodb::{Collection, Database};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NFT {
    pub mint: String, // NFT mint address
    pub name: String,
    pub symbol: String,
    pub uri: Option<String>, // Metadata URI
    pub image_uri: Option<String>,
    pub collection: Option<String>,
    pub owner: String, // Current owner pubkey
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NFTTransferRequest {
    pub wallet_id: String,
    pub mint: String,
    pub destination: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NFTMetadata {
    pub mint: String,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub attributes: Vec<Attribute>,
    pub collection: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

pub struct AphroditeNFTManager {
    db: Arc<Database>,
    solana_rpc_url: String,
}

impl AphroditeNFTManager {
    pub fn new(db: Arc<Database>, solana_rpc_url: String) -> Self {
        Self { db, solana_rpc_url }
    }

    /// Get all NFTs owned by a wallet
    pub async fn get_nfts(&self, wallet_pubkey: &str) -> Result<Vec<NFT>, String> {
        use crate::solana::SolanaClient;
        let client = SolanaClient::new(self.solana_rpc_url.clone());

        let pubkey = Pubkey::from_str(wallet_pubkey)
            .map_err(|_| "Invalid pubkey".to_string())?;

        // Get token accounts (NFTs are SPL tokens)
        let token_accounts = client
            .get_token_accounts(&pubkey)
            .await
            .map_err(|e| format!("Failed to get token accounts: {}", e))?;

        let mut nfts = Vec::new();

        for account in token_accounts {
            // Check if it's an NFT (amount == 1 and decimals == 0)
            if account.amount == 1 && account.decimals == 0 {
                // Fetch NFT metadata
                let metadata = self.get_nft_metadata(&account.mint).await.ok();

                nfts.push(NFT {
                    mint: account.mint.clone(),
                    name: metadata.as_ref()
                        .map(|m| m.name.clone())
                        .unwrap_or_else(|| "Unknown NFT".to_string()),
                    symbol: metadata.as_ref()
                        .and_then(|m| m.collection.clone())
                        .unwrap_or_else(|| "NFT".to_string()),
                    uri: metadata.as_ref().and_then(|m| m.image.clone()),
                    image_uri: metadata.as_ref().and_then(|m| m.image.clone()),
                    collection: metadata.as_ref().and_then(|m| m.collection.clone()),
                    owner: wallet_pubkey.to_string(),
                });
            }
        }

        Ok(nfts)
    }

    /// Get NFT metadata (cached in database)
    pub async fn get_nft_metadata(&self, mint: &str) -> Result<NFTMetadata, String> {
        let collection: Collection<NFTMetadata> = self.db.collection("nft_metadata");

        // Check cache first
        if let Some(cached) = collection
            .find_one(doc! { "mint": mint }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))? {
            return Ok(cached);
        }

        // Fetch from on-chain (Metaplex Token Metadata)
        // For now, return basic metadata
        let metadata = NFTMetadata {
            mint: mint.to_string(),
            name: "Unknown NFT".to_string(),
            description: None,
            image: None,
            attributes: Vec::new(),
            collection: None,
        };

        // Cache it
        collection
            .insert_one(&metadata, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(metadata)
    }

    /// Create NFT transfer transaction
    pub async fn create_transfer_transaction(
        &self,
        wallet_pubkey: &str,
        mint: &str,
        destination: &str,
    ) -> Result<String, String> {
        // NFTs are SPL tokens, so use token transfer
        use crate::dionysus::DionysusTokenManager;
        let token_manager = DionysusTokenManager::new(
            Arc::clone(&self.db),
            self.solana_rpc_url.clone(),
        );

        // Transfer 1 NFT (amount = 1)
        token_manager
            .create_transfer_transaction(wallet_pubkey, mint, destination, 1)
            .await
    }
}


