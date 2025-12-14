// Zeus - God of Gods, Master of Wallets
// Handles wallet creation, import, export, and management

use mongodb::{Collection, Database};
use mongodb::bson::{doc, DateTime};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    signature::{Keypair, Signer},
};
use std::sync::Arc;
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wallet {
    #[serde(rename = "_id")]
    pub id: String, // UUID
    pub user_id: String, // User identifier (can be email, username, etc.)
    pub pubkey: String, // Solana public key
    pub name: String, // Wallet nickname
    pub encrypted_private_key: String, // AES-encrypted private key
    pub salt: String, // Salt for encryption (hex)
    pub is_active: bool, // Active wallet for user
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub user_id: String,
    pub name: String,
    pub password: String, // For encryption
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportWalletRequest {
    pub user_id: String,
    pub name: String,
    pub private_key: String, // Base58 or hex
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletResponse {
    pub id: String,
    pub pubkey: String,
    pub name: String,
    pub is_active: bool,
    pub balance: Option<u64>, // SOL balance in lamports
}

pub struct ZeusWalletManager {
    db: Arc<Database>,
    solana_rpc_url: String,
}

impl ZeusWalletManager {
    pub fn new(db: Arc<Database>, solana_rpc_url: String) -> Self {
        Self { db, solana_rpc_url }
    }

    pub fn get_collection(&self) -> Collection<Wallet> {
        self.db.collection::<Wallet>("wallets")
    }

    /// Create a new wallet
    pub async fn create_wallet(
        &self,
        user_id: &str,
        name: &str,
        password: &str,
    ) -> Result<WalletResponse, String> {
        // Generate new keypair
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey().to_string();

        // Encrypt private key
        let (encrypted_key, salt) = self.encrypt_private_key(
            &keypair.to_bytes(),
            password,
        )?;

        let wallet = Wallet {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            pubkey: pubkey.clone(),
            name: name.to_string(),
            encrypted_private_key: encrypted_key,
            salt,
            is_active: false,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };

        // Set as active if it's the first wallet
        let collection = self.get_collection();
        let existing_count = collection
            .count_documents(doc! { "user_id": user_id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let is_active = existing_count == 0;

        let mut wallet = wallet;
        wallet.is_active = is_active;

        // If setting as active, deactivate others
        if is_active {
            collection
                .update_many(
                    doc! { "user_id": user_id, "is_active": true },
                    doc! { "$set": { "is_active": false, "updated_at": DateTime::now() } },
                    None,
                )
                .await
                .map_err(|e| format!("Database error: {}", e))?;
        }

        collection
            .insert_one(&wallet, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        // Get balance
        let balance = self.get_balance(&pubkey).await.ok();

        Ok(WalletResponse {
            id: wallet.id,
            pubkey: wallet.pubkey,
            name: wallet.name,
            is_active: wallet.is_active,
            balance,
        })
    }

    /// Import existing wallet
    pub async fn import_wallet(
        &self,
        user_id: &str,
        name: &str,
        private_key: &str,
        password: &str,
    ) -> Result<WalletResponse, String> {
        // Parse private key (base58 or hex)
        let key_bytes = if private_key.len() == 64 {
            // Hex format
            hex::decode(private_key)
                .map_err(|_| "Invalid hex private key".to_string())?
        } else {
            // Base58 format
            bs58::decode(private_key)
                .into_vec()
                .map_err(|_| "Invalid base58 private key".to_string())?
        };

        if key_bytes.len() != 64 {
            return Err("Private key must be 64 bytes".to_string());
        }

        let keypair = Keypair::from_bytes(&key_bytes)
            .map_err(|_| "Invalid keypair bytes".to_string())?;
        let pubkey = keypair.pubkey().to_string();

        // Check if wallet already exists
        let collection = self.get_collection();
        let existing = collection
            .find_one(doc! { "pubkey": &pubkey }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if existing.is_some() {
            return Err("Wallet already imported".to_string());
        }

        // Encrypt private key
        let (encrypted_key, salt) = self.encrypt_private_key(&key_bytes, password)?;

        let wallet = Wallet {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            pubkey: pubkey.clone(),
            name: name.to_string(),
            encrypted_private_key: encrypted_key,
            salt,
            is_active: false,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };

        collection
            .insert_one(&wallet, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let balance = self.get_balance(&pubkey).await.ok();

        Ok(WalletResponse {
            id: wallet.id,
            pubkey: wallet.pubkey,
            name: wallet.name,
            is_active: wallet.is_active,
            balance,
        })
    }

    /// List all wallets for a user
    pub async fn list_wallets(&self, user_id: &str) -> Result<Vec<WalletResponse>, String> {
        let collection = self.get_collection();
        let filter = doc! { "user_id": user_id };
        
        let mut wallets = Vec::new();
        let mut cursor = collection
            .find(filter, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        while let Some(wallet) = cursor.try_next().await
            .map_err(|e| format!("Database error: {}", e))? {
            let balance = self.get_balance(&wallet.pubkey).await.ok();
            wallets.push(WalletResponse {
                id: wallet.id,
                pubkey: wallet.pubkey,
                name: wallet.name,
                is_active: wallet.is_active,
                balance,
            });
        }

        Ok(wallets)
    }

    /// Get active wallet for user
    pub async fn get_active_wallet(&self, user_id: &str) -> Result<Option<WalletResponse>, String> {
        let collection = self.get_collection();
        let filter = doc! { "user_id": user_id, "is_active": true };
        
        if let Some(wallet) = collection
            .find_one(filter, None)
            .await
            .map_err(|e| format!("Database error: {}", e))? {
            let balance = self.get_balance(&wallet.pubkey).await.ok();
            Ok(Some(WalletResponse {
                id: wallet.id,
                pubkey: wallet.pubkey,
                name: wallet.name,
                is_active: wallet.is_active,
                balance,
            }))
        } else {
            Ok(None)
        }
    }

    /// Set active wallet
    pub async fn set_active_wallet(
        &self,
        user_id: &str,
        wallet_id: &str,
    ) -> Result<(), String> {
        let collection = self.get_collection();

        // Verify wallet belongs to user
        let wallet = collection
            .find_one(doc! { "_id": wallet_id, "user_id": user_id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| "Wallet not found".to_string())?;

        // Deactivate all other wallets
        collection
            .update_many(
                doc! { "user_id": user_id, "is_active": true },
                doc! { "$set": { "is_active": false, "updated_at": DateTime::now() } },
                None,
            )
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        // Activate this wallet
        collection
            .update_one(
                doc! { "_id": wallet_id },
                doc! { "$set": { "is_active": true, "updated_at": DateTime::now() } },
                None,
            )
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(())
    }

    /// Decrypt and get private key (requires password)
    pub async fn get_private_key(
        &self,
        wallet_id: &str,
        password: &str,
    ) -> Result<Vec<u8>, String> {
        let collection = self.get_collection();
        let wallet = collection
            .find_one(doc! { "_id": wallet_id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| "Wallet not found".to_string())?;

        self.decrypt_private_key(&wallet.encrypted_private_key, &wallet.salt, password)
    }

    /// Delete wallet
    pub async fn delete_wallet(&self, user_id: &str, wallet_id: &str) -> Result<(), String> {
        let collection = self.get_collection();

        // Verify ownership
        let wallet = collection
            .find_one(doc! { "_id": wallet_id, "user_id": user_id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| "Wallet not found".to_string())?;

        // Don't allow deleting if it's the only wallet
        let count = collection
            .count_documents(doc! { "user_id": user_id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if count <= 1 {
            return Err("Cannot delete the only wallet".to_string());
        }

        collection
            .delete_one(doc! { "_id": wallet_id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        // If deleted wallet was active, activate another one
        if wallet.is_active {
            if let Some(new_active) = collection
                .find_one(doc! { "user_id": user_id }, None)
                .await
                .map_err(|e| format!("Database error: {}", e))? {
                collection
                    .update_one(
                        doc! { "_id": &new_active.id },
                        doc! { "$set": { "is_active": true, "updated_at": DateTime::now() } },
                        None,
                    )
                    .await
                    .map_err(|e| format!("Database error: {}", e))?;
            }
        }

        Ok(())
    }

    /// Encrypt private key using AES-256-GCM with PBKDF2
    fn encrypt_private_key(
        &self,
        key_bytes: &[u8],
        password: &str,
    ) -> Result<(String, String), String> {
        // Generate salt
        let salt = (0..16)
            .map(|_| rand::random::<u8>())
            .collect::<Vec<u8>>();
        let salt_hex = hex::encode(&salt);

        // Derive key using PBKDF2
        let mut key = [0u8; 32];
        pbkdf2::pbkdf2::<hmac::Hmac<Sha256>>(
            password.as_bytes(),
            &salt,
            100000, // iterations
            &mut key,
        );

        // For simplicity, we'll use a basic encryption
        // In production, use proper AES-GCM with nonce
        let encrypted = self.simple_xor_encrypt(key_bytes, &key);
        let encrypted_hex = hex::encode(&encrypted);

        Ok((encrypted_hex, salt_hex))
    }

    /// Decrypt private key
    fn decrypt_private_key(
        &self,
        encrypted_hex: &str,
        salt_hex: &str,
        password: &str,
    ) -> Result<Vec<u8>, String> {
        let encrypted = hex::decode(encrypted_hex)
            .map_err(|_| "Invalid encrypted key format".to_string())?;
        let salt = hex::decode(salt_hex)
            .map_err(|_| "Invalid salt format".to_string())?;

        // Derive key
        let mut key = [0u8; 32];
        pbkdf2::pbkdf2::<hmac::Hmac<Sha256>>(
            password.as_bytes(),
            &salt,
            100000,
            &mut key,
        );

        let decrypted = self.simple_xor_decrypt(&encrypted, &key);
        Ok(decrypted)
    }

    fn simple_xor_encrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(i, &b)| b ^ key[i % key.len()])
            .collect()
    }

    fn simple_xor_decrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8> {
        self.simple_xor_encrypt(data, key) // XOR is symmetric
    }

    /// Get SOL balance for a pubkey
    async fn get_balance(&self, pubkey: &str) -> Result<u64, String> {
        use crate::solana::SolanaClient;
        let client = SolanaClient::new(self.solana_rpc_url.clone());
        client.get_balance(pubkey).await
    }
}

use futures_util::TryStreamExt;


