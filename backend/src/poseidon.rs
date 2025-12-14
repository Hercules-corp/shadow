// Poseidon - God of Transactions
// Handles transaction signing, approval, and management

use mongodb::{Collection, Database};
use mongodb::bson::{doc, DateTime};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    signature::Keypair,
    transaction::Transaction,
};
use std::sync::Arc;
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PendingTransaction {
    #[serde(rename = "_id")]
    pub id: String,
    pub user_id: String,
    pub wallet_id: String,
    pub dapp_origin: String, // Origin of the dApp requesting signature
    pub transaction_data: String, // Base64 encoded transaction
    pub message: Option<String>, // Human-readable message
    pub status: TransactionStatus,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Approved,
    Rejected,
    Signed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignTransactionRequest {
    pub transaction_id: String,
    pub password: String, // To decrypt wallet
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub wallet_id: String,
    pub dapp_origin: String,
    pub transaction_data: String, // Base64 encoded
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub id: String,
    pub status: TransactionStatus,
    pub signed_transaction: Option<String>, // Base64 encoded signed transaction
    pub message: Option<String>,
}

pub struct PoseidonTransactionManager {
    db: Arc<Database>,
}

impl PoseidonTransactionManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn get_collection(&self) -> Collection<PendingTransaction> {
        self.db.collection::<PendingTransaction>("pending_transactions")
    }

    /// Create a pending transaction request
    pub async fn create_transaction(
        &self,
        user_id: &str,
        wallet_id: &str,
        dapp_origin: &str,
        transaction_data: &str,
        message: Option<&str>,
    ) -> Result<TransactionResponse, String> {
        // Validate transaction data
        let tx_bytes = general_purpose::STANDARD.decode(transaction_data)
            .map_err(|_| "Invalid base64 transaction data".to_string())?;

        let _transaction: Transaction = bincode::deserialize(&tx_bytes)
            .map_err(|_| "Invalid transaction format".to_string())?;

        let pending = PendingTransaction {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            wallet_id: wallet_id.to_string(),
            dapp_origin: dapp_origin.to_string(),
            transaction_data: transaction_data.to_string(),
            message: message.map(|s| s.to_string()),
            status: TransactionStatus::Pending,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };

        let collection = self.get_collection();
        collection
            .insert_one(&pending, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(TransactionResponse {
            id: pending.id,
            status: pending.status,
            signed_transaction: None,
            message: pending.message.clone(),
        })
    }

    /// Get pending transactions for a user
    pub async fn get_pending_transactions(
        &self,
        user_id: &str,
    ) -> Result<Vec<TransactionResponse>, String> {
        let collection = self.get_collection();
        let filter = doc! {
            "user_id": user_id,
            "status": "pending"
        };

        let mut transactions = Vec::new();
        let mut cursor = collection
            .find(filter, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        while let Some(tx) = cursor.try_next().await
            .map_err(|e| format!("Database error: {}", e))? {
            transactions.push(TransactionResponse {
                id: tx.id,
                status: tx.status,
                signed_transaction: None,
                message: tx.message.clone(),
            });
        }

        Ok(transactions)
    }

    /// Sign a transaction
    pub async fn sign_transaction(
        &self,
        transaction_id: &str,
        user_id: &str,
        private_key: &[u8],
    ) -> Result<TransactionResponse, String> {
        let collection = self.get_collection();

        // Get transaction
        let mut tx = collection
            .find_one(doc! { "_id": transaction_id, "user_id": user_id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| "Transaction not found".to_string())?;

        if tx.status != TransactionStatus::Pending {
            return Err("Transaction already processed".to_string());
        }

        // Decode transaction
        let tx_bytes = general_purpose::STANDARD.decode(&tx.transaction_data)
            .map_err(|_| "Invalid transaction data".to_string())?;
        let mut transaction: Transaction = bincode::deserialize(&tx_bytes)
            .map_err(|_| "Invalid transaction format".to_string())?;

        // Create keypair from private key
        let keypair = Keypair::from_bytes(private_key)
            .map_err(|_| "Invalid private key".to_string())?;

        // Sign transaction
        transaction.sign(&[&keypair], transaction.message.recent_blockhash);

        // Encode signed transaction
        let signed_data = bincode::serialize(&transaction)
            .map_err(|_| "Failed to serialize transaction".to_string())?;
        let signed_base64 = general_purpose::STANDARD.encode(&signed_data);

        // Update status
        tx.status = TransactionStatus::Signed;
        tx.updated_at = DateTime::now();

        collection
            .update_one(
                doc! { "_id": transaction_id },
                doc! {
                    "$set": {
                        "status": "signed",
                        "updated_at": DateTime::now()
                    }
                },
                None,
            )
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(TransactionResponse {
            id: tx.id,
            status: TransactionStatus::Signed,
            signed_transaction: Some(signed_base64),
            message: tx.message.clone(),
        })
    }

    /// Reject a transaction
    pub async fn reject_transaction(
        &self,
        transaction_id: &str,
        user_id: &str,
    ) -> Result<(), String> {
        let collection = self.get_collection();

        collection
            .update_one(
                doc! { "_id": transaction_id, "user_id": user_id },
                doc! {
                    "$set": {
                        "status": "rejected",
                        "updated_at": DateTime::now()
                    }
                },
                None,
            )
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(())
    }

    /// Get transaction by ID
    pub async fn get_transaction(
        &self,
        transaction_id: &str,
        user_id: &str,
    ) -> Result<Option<TransactionResponse>, String> {
        let collection = self.get_collection();

        if let Some(tx) = collection
            .find_one(doc! { "_id": transaction_id, "user_id": user_id }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))? {
            Ok(Some(TransactionResponse {
                id: tx.id.clone(),
                status: tx.status.clone(),
                signed_transaction: if tx.status == TransactionStatus::Signed {
                    Some(tx.transaction_data.clone())
                } else {
                    None
                },
                message: tx.message.clone(),
            }))
        } else {
            Ok(None)
        }
    }
}

use futures_util::TryStreamExt;


