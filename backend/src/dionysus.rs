// Dionysus - God of Tokens and Wealth
// Handles SPL token operations, balances, and transfers

use mongodb::{Collection, Database};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenBalance {
    pub mint: String, // Token mint address
    pub amount: u64, // Raw amount (with decimals)
    pub decimals: u8,
    pub ui_amount: f64, // Human-readable amount
    pub symbol: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenTransferRequest {
    pub wallet_id: String,
    pub mint: String, // Token mint address
    pub destination: String, // Destination pubkey
    pub amount: u64, // Amount in smallest unit
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenMetadata {
    pub mint: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub logo_uri: Option<String>,
}

pub struct DionysusTokenManager {
    db: Arc<Database>,
    solana_rpc_url: String,
}

impl DionysusTokenManager {
    pub fn new(db: Arc<Database>, solana_rpc_url: String) -> Self {
        Self { db, solana_rpc_url }
    }

    /// Get all token balances for a wallet
    pub async fn get_token_balances(
        &self,
        wallet_pubkey: &str,
    ) -> Result<Vec<TokenBalance>, String> {
        use crate::solana::SolanaClient;
        let client = SolanaClient::new(self.solana_rpc_url.clone());

        let pubkey = Pubkey::from_str(wallet_pubkey)
            .map_err(|_| "Invalid pubkey".to_string())?;

        // Get token accounts
        let token_accounts = client
            .get_token_accounts(&pubkey)
            .await
            .map_err(|e| format!("Failed to get token accounts: {}", e))?;

        let mut balances = Vec::new();

        for account in token_accounts {
            // Get token metadata
            let metadata = self.get_token_metadata(&account.mint).await.ok();

            balances.push(TokenBalance {
                mint: account.mint,
                amount: account.amount,
                decimals: account.decimals,
                ui_amount: account.amount as f64 / 10_f64.powi(account.decimals as i32),
                symbol: metadata.as_ref().map(|m| m.symbol.clone()),
                name: metadata.as_ref().map(|m| m.name.clone()),
            });
        }

        Ok(balances)
    }

    /// Get token metadata (cached in database)
    pub async fn get_token_metadata(&self, mint: &str) -> Result<TokenMetadata, String> {
        let collection: Collection<TokenMetadata> = self.db.collection("token_metadata");

        // Check cache first
        if let Some(cached) = collection
            .find_one(doc! { "mint": mint }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))? {
            return Ok(cached);
        }

        // Fetch from on-chain (simplified - in production use Metaplex Token Metadata)
        // For now, return basic metadata
        let metadata = TokenMetadata {
            mint: mint.to_string(),
            symbol: "UNKNOWN".to_string(),
            name: "Unknown Token".to_string(),
            decimals: 9, // Default
            logo_uri: None,
        };

        // Cache it
        collection
            .insert_one(&metadata, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(metadata)
    }

    /// Create token transfer transaction
    pub async fn create_transfer_transaction(
        &self,
        wallet_pubkey: &str,
        mint: &str,
        destination: &str,
        amount: u64,
    ) -> Result<String, String> {
        use crate::solana::SolanaClient;
        let client = SolanaClient::new(self.solana_rpc_url.clone());

        let from_pubkey = Pubkey::from_str(wallet_pubkey)
            .map_err(|_| "Invalid source pubkey".to_string())?;
        let to_pubkey = Pubkey::from_str(destination)
            .map_err(|_| "Invalid destination pubkey".to_string())?;
        let _mint_pubkey = Pubkey::from_str(mint)
            .map_err(|_| "Invalid mint pubkey".to_string())?;

        // Create transfer instruction
        // In production, use spl-token crate properly
        let instruction = spl_token::instruction::transfer(
            &spl_token::id(),
            &from_pubkey, // source
            &to_pubkey,   // destination
            &from_pubkey, // authority
            &[],
            amount,
        )
        .map_err(|e| format!("Failed to create transfer instruction: {}", e))?;

        // Get recent blockhash
        let _blockhash = client
            .get_recent_blockhash()
            .await
            .map_err(|e| format!("Failed to get blockhash: {}", e))?;

        // Create message
        let message = solana_sdk::message::Message::new(
            &[instruction],
            Some(&from_pubkey),
        );

        // Serialize transaction (unsigned)
        let transaction = solana_sdk::transaction::Transaction::new_unsigned(message);
        use base64::{Engine as _, engine::general_purpose};
        let tx_bytes = bincode::serialize(&transaction)
            .map_err(|_| "Failed to serialize transaction".to_string())?;

        Ok(general_purpose::STANDARD.encode(&tx_bytes))
    }
}


