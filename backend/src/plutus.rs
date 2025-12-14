// Plutus - God of Wealth and Portfolio
// Handles portfolio tracking, balance aggregation, and transaction history

use mongodb::{Collection, Database};
use mongodb::bson::{doc, DateTime};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Portfolio {
    pub sol_balance: u64, // SOL in lamports
    pub sol_value_usd: f64, // Estimated USD value
    pub token_count: usize,
    pub nft_count: usize,
    pub total_value_usd: f64, // Total portfolio value
    pub tokens: Vec<TokenBalance>,
    pub nfts: Vec<NFT>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenBalance {
    pub mint: String,
    pub amount: u64,
    pub decimals: u8,
    pub ui_amount: f64,
    pub symbol: Option<String>,
    pub value_usd: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NFT {
    pub mint: String,
    pub name: String,
    pub image_uri: Option<String>,
    pub collection: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionHistory {
    pub signature: String,
    pub timestamp: DateTime,
    pub type_: TransactionType,
    pub amount: Option<u64>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub status: TransactionStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Transfer,
    TokenTransfer,
    NftTransfer,
    Swap,
    Stake,
    Unstake,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

pub struct PlutusPortfolioManager {
    db: Arc<Database>,
    solana_rpc_url: String,
}

impl PlutusPortfolioManager {
    pub fn new(db: Arc<Database>, solana_rpc_url: String) -> Self {
        Self { db, solana_rpc_url }
    }

    /// Get complete portfolio for a wallet
    pub async fn get_portfolio(
        &self,
        wallet_pubkey: &str,
    ) -> Result<Portfolio, String> {
        use crate::solana::SolanaClient;
        use crate::dionysus::DionysusTokenManager;
        use crate::aphrodite::AphroditeNFTManager;

        let client = SolanaClient::new(self.solana_rpc_url.clone());
        let token_manager = DionysusTokenManager::new(
            Arc::clone(&self.db),
            self.solana_rpc_url.clone(),
        );
        let nft_manager = AphroditeNFTManager::new(
            Arc::clone(&self.db),
            self.solana_rpc_url.clone(),
        );

        // Get SOL balance
        let sol_balance = client
            .get_balance(wallet_pubkey)
            .await
            .map_err(|e| format!("Failed to get balance: {}", e))?;

        // Get token balances
        let tokens = token_manager
            .get_token_balances(wallet_pubkey)
            .await
            .map_err(|e| format!("Failed to get tokens: {}", e))?;

        // Get NFTs
        let nfts = nft_manager
            .get_nfts(wallet_pubkey)
            .await
            .map_err(|e| format!("Failed to get NFTs: {}", e))?;

        // Calculate USD values (simplified - in production use price APIs)
        let sol_price_usd = self.get_sol_price().await.unwrap_or(100.0);
        let sol_value_usd = (sol_balance as f64 / 1_000_000_000.0) * sol_price_usd;

        // Calculate total value
        let total_value_usd = sol_value_usd;
        // Token USD values would be fetched from price API in production

        Ok(Portfolio {
            sol_balance,
            sol_value_usd,
            token_count: tokens.len(),
            nft_count: nfts.len(),
            total_value_usd: total_value_usd,
            tokens: tokens.into_iter().map(|t| TokenBalance {
                mint: t.mint,
                amount: t.amount,
                decimals: t.decimals,
                ui_amount: t.ui_amount,
                symbol: t.symbol,
                value_usd: None, // Would fetch from price API
            }).collect(),
            nfts: nfts.into_iter().map(|n| NFT {
                mint: n.mint,
                name: n.name,
                image_uri: n.image_uri,
                collection: n.collection,
            }).collect(),
        })
    }

    /// Get transaction history for a wallet
    pub async fn get_transaction_history(
        &self,
        wallet_pubkey: &str,
        limit: Option<u32>,
    ) -> Result<Vec<TransactionHistory>, String> {
        use crate::solana::SolanaClient;
        let client = SolanaClient::new(self.solana_rpc_url.clone());

        let pubkey = Pubkey::from_str(wallet_pubkey)
            .map_err(|_| "Invalid pubkey".to_string())?;

        // Get signatures for address
        let signatures = client
            .get_signatures_for_address(&pubkey, limit.unwrap_or(50))
            .await
            .map_err(|e| format!("Failed to get signatures: {}", e))?;

        let mut history = Vec::new();

        for sig in signatures {
            // Parse transaction to determine type
            let tx_type = TransactionType::Other; // Would parse transaction to determine
            let status = TransactionStatus::Confirmed; // Would check confirmation status

            // Convert Unix timestamp to DateTime
            let timestamp = if let Some(block_time) = sig.block_time {
                // MongoDB DateTime uses milliseconds since epoch
                DateTime::from_millis(block_time * 1000)
            } else {
                DateTime::now()
            };
            
            history.push(TransactionHistory {
                signature: sig.signature,
                timestamp,
                type_: tx_type,
                amount: None,
                from: None,
                to: None,
                status,
            });
        }

        Ok(history)
    }

    /// Get SOL price in USD (cached)
    async fn get_sol_price(&self) -> Result<f64, String> {
        let collection: Collection<PriceCache> = self.db.collection("price_cache");

        // Check cache (5 minute TTL)
        if let Some(cached) = collection
            .find_one(doc! { "symbol": "SOL" }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))? {
            let age = (DateTime::now().timestamp_millis() - cached.updated_at.timestamp_millis()) / 1000;
            if age < 300 { // 5 minutes
                return Ok(cached.price);
            }
        }

        // Fetch from API (simplified - in production use CoinGecko or similar)
        let price = 100.0; // Placeholder

        // Cache it
        let cache = PriceCache {
            symbol: "SOL".to_string(),
            price,
            updated_at: DateTime::now(),
        };

        collection
            .replace_one(
                doc! { "symbol": "SOL" },
                &cache,
                mongodb::options::ReplaceOptions::builder()
                    .upsert(true)
                    .build(),
            )
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(price)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PriceCache {
    symbol: String,
    price: f64,
    updated_at: DateTime,
}


