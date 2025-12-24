// Link Converter - Converts URLs to SPL tokens and manages sublinks
// Part of the Shadow token-only domain system

use mongodb::{Collection, Database};
use mongodb::bson::{doc, DateTime};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use sha2::{Sha256, Digest};
use std::str::FromStr;
use std::sync::Arc;
use hex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinkMapping {
    #[serde(rename = "_id")]
    pub url_hash: String, // SHA256 hash of original URL
    pub token_mint: String, // SPL token mint address
    pub original_url: String, // Original URL (for reference)
    pub subpaths: Vec<String>, // Array of subpaths (e.g., ["/page1", "/page2"])
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConvertLinkRequest {
    pub url: String,
    pub sublink: Option<String>, // Optional subpath to add
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConvertLinkResponse {
    pub token_mint: String,
    pub url_hash: String,
    pub is_new: bool, // Whether a new token was minted
    pub subpath: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralTokenRequest {
    pub platform: String, // e.g., "twitter"
    pub token_name: String,
    pub token_symbol: String,
}

pub struct LinkConverter {
    db: Arc<Database>,
    solana_rpc_url: String,
}

impl LinkConverter {
    pub fn new(db: Arc<Database>, solana_rpc_url: String) -> Self {
        Self { db, solana_rpc_url }
    }

    pub fn get_collection(&self) -> Collection<LinkMapping> {
        self.db.collection::<LinkMapping>("link_mappings")
    }

    /// Hash a URL to get a unique identifier
    pub fn hash_url(url: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(url.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Check if URL already has a token mapping
    pub async fn get_existing_mapping(
        &self,
        url: &str,
    ) -> Result<Option<LinkMapping>, String> {
        let url_hash = Self::hash_url(url);
        let collection = self.get_collection();

        let mapping = collection
            .find_one(doc! { "url_hash": &url_hash }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(mapping)
    }

    /// Convert URL to token (create mapping or return existing)
    pub async fn convert_link(
        &self,
        url: &str,
        sublink: Option<&str>,
    ) -> Result<ConvertLinkResponse, String> {
        // Normalize URL
        let normalized_url = url.trim().to_lowercase();
        
        // Check if mapping already exists
        if let Some(existing) = self.get_existing_mapping(&normalized_url).await? {
            // If sublink provided and doesn't exist, add it
            if let Some(subpath) = sublink {
                if !existing.subpaths.contains(&subpath.to_string()) {
                    let mut updated = existing.clone();
                    updated.subpaths.push(subpath.to_string());
                    updated.updated_at = DateTime::now();

                    let collection = self.get_collection();
                    collection
                        .update_one(
                            doc! { "url_hash": &updated.url_hash },
                            doc! {
                                "$set": {
                                    "subpaths": &updated.subpaths,
                                    "updated_at": updated.updated_at
                                }
                            },
                            None,
                        )
                        .await
                        .map_err(|e| format!("Database error: {}", e))?;

                    return Ok(ConvertLinkResponse {
                        token_mint: updated.token_mint,
                        url_hash: updated.url_hash,
                        is_new: false,
                        subpath: Some(subpath.to_string()),
                    });
                }
            }

            return Ok(ConvertLinkResponse {
                token_mint: existing.token_mint,
                url_hash: existing.url_hash,
                is_new: false,
                subpath: sublink.map(|s| s.to_string()),
            });
        }

        // Create new token for URL
        // In production, this would mint an SPL token
        // For now, generate a deterministic pubkey from URL hash
        let url_hash = Self::hash_url(&normalized_url);
        let token_mint = self.derive_token_mint_from_hash(&url_hash)?;

        // Create mapping
        let mapping = LinkMapping {
            url_hash: url_hash.clone(),
            token_mint: token_mint.clone(),
            original_url: normalized_url,
            subpaths: sublink.map(|s| vec![s.to_string()]).unwrap_or_default(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };

        let collection = self.get_collection();
        collection
            .insert_one(&mapping, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(ConvertLinkResponse {
            token_mint,
            url_hash,
            is_new: true,
            subpath: sublink.map(|s| s.to_string()),
        })
    }

    /// Create a general platform token (e.g., Twitter)
    pub async fn create_general_token(
        &self,
        platform: &str,
        token_name: &str,
        token_symbol: &str,
    ) -> Result<String, String> {
        // Check if platform token already exists
        let platform_key = format!("platform:{}", platform.to_lowercase());
        
        if let Some(existing) = self.get_existing_mapping(&platform_key).await? {
            return Ok(existing.token_mint);
        }

        // Create new token for platform
        let url_hash = Self::hash_url(&platform_key);
        let token_mint = self.derive_token_mint_from_hash(&url_hash)?;

        let mapping = LinkMapping {
            url_hash: url_hash.clone(),
            token_mint: token_mint.clone(),
            original_url: platform_key,
            subpaths: Vec::new(), // Subpaths added later via sublinks
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };

        let collection = self.get_collection();
        collection
            .insert_one(&mapping, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        // TODO: Actually mint the SPL token with name and symbol
        // For now, return the derived mint address

        Ok(token_mint)
    }

    /// Get token mint from URL (if exists)
    pub async fn get_token_from_url(&self, url: &str) -> Result<Option<String>, String> {
        let normalized_url = url.trim().to_lowercase();
        
        if let Some(mapping) = self.get_existing_mapping(&normalized_url).await? {
            Ok(Some(mapping.token_mint))
        } else {
            Ok(None)
        }
    }

    /// Get URL from token mint
    pub async fn get_url_from_token(&self, token_mint: &str) -> Result<Option<String>, String> {
        let collection = self.get_collection();
        
        let mapping = collection
            .find_one(doc! { "token_mint": token_mint }, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(mapping.map(|m| m.original_url))
    }

    /// Add sublink to existing token
    pub async fn add_sublink(
        &self,
        token_mint: &str,
        subpath: &str,
    ) -> Result<(), String> {
        let collection = self.get_collection();
        
        let filter = doc! { "token_mint": token_mint };
        let update = doc! {
            "$addToSet": { "subpaths": subpath },
            "$set": { "updated_at": DateTime::now() }
        };

        collection
            .update_one(filter, update, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(())
    }

    /// Derive a deterministic token mint from URL hash
    /// In production, this would actually mint an SPL token
    fn derive_token_mint_from_hash(&self, hash: &str) -> Result<String, String> {
        // Use hash to derive a deterministic pubkey
        // This is a simplified version - in production, mint actual SPL token
        let hash_bytes = hex::decode(hash)
            .map_err(|_| "Invalid hash".to_string())?;
        
        // Take first 32 bytes for pubkey
        if hash_bytes.len() < 32 {
            return Err("Hash too short".to_string());
        }

        let pubkey_bytes: [u8; 32] = hash_bytes[..32]
            .try_into()
            .map_err(|_| "Failed to convert hash to pubkey".to_string())?;

        // Create pubkey (this would be the token mint)
        let pubkey = Pubkey::try_from(pubkey_bytes)
            .map_err(|_| "Invalid pubkey bytes".to_string())?;

        Ok(pubkey.to_string())
    }

    /// Validate that an address is a valid SPL token mint
    pub fn validate_token_address(address: &str) -> Result<bool, String> {
        Pubkey::from_str(address)
            .map(|_| true)
            .map_err(|e| format!("Invalid token address: {}", e))
    }
}

