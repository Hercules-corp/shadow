use serde_json::Value;
use std::env;

pub struct PinataStorage {
    api_key: Option<String>,
    secret: Option<String>,
}

impl PinataStorage {
    pub fn new() -> Self {
        Self {
            api_key: env::var("PINATA_API_KEY").ok(),
            secret: env::var("PINATA_SECRET").ok(),
        }
    }

    pub async fn upload(&self, data: &[u8], name: &str) -> Result<String, String> {
        if self.api_key.is_none() || self.secret.is_none() {
            return Err("Pinata credentials not configured".to_string());
        }

        let client = reqwest::Client::new();
        let form = reqwest::multipart::Form::new()
            .text("pinataOptions", r#"{"cidVersion":1}"#)
            .text("pinataMetadata", format!(r#"{{"name":"{}"}}"#, name))
            .part("file", reqwest::multipart::Part::bytes(data.to_vec()).file_name(name.to_string()));

        let response = client
            .post("https://api.pinata.cloud/pinning/pinFileToIPFS")
            .header("pinata_api_key", self.api_key.as_ref().unwrap())
            .header("pinata_secret_api_key", self.secret.as_ref().unwrap())
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Pinata upload error: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Pinata error: {}", response.status()));
        }

        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse Pinata response: {}", e))?;
        
        let ipfs_hash = json["IpfsHash"].as_str()
            .ok_or_else(|| "Missing IpfsHash in response".to_string())?;

        Ok(format!("ipfs://{}", ipfs_hash))
    }

    pub async fn get(&self, cid: &str) -> Result<Vec<u8>, String> {
        let cid = cid.strip_prefix("ipfs://").unwrap_or(cid);
        let url = format!("https://gateway.pinata.cloud/ipfs/{}", cid);
        
        let client = reqwest::Client::new();
        let response = client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch from IPFS: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("IPFS fetch error: {}", response.status()));
        }

        let bytes = response.bytes().await
            .map_err(|e| format!("Failed to read IPFS data: {}", e))?;

        Ok(bytes.to_vec())
    }
}

pub struct BundlrStorage {
    node_url: Option<String>,
    private_key: Option<String>,
}

impl BundlrStorage {
    pub fn new() -> Self {
        Self {
            node_url: env::var("BUNDLR_NODE_URL")
                .unwrap_or_else(|_| "https://devnet.bundlr.network".to_string())
                .into(),
            private_key: env::var("BUNDLR_PRIVATE_KEY").ok(),
        }
    }

    pub async fn upload(&self, data: &[u8], _tags: Vec<(&str, &str)>) -> Result<String, String> {
        if self.private_key.is_none() {
            return Err("Bundlr private key not configured".to_string());
        }

        let client = reqwest::Client::new();
        let url = format!("{}/tx", self.node_url.as_ref().unwrap());

        // Simplified upload - in production, use bundlr-sdk
        let response = client
            .post(&url)
            .header("Content-Type", "application/octet-stream")
            .body(data.to_vec())
            .send()
            .await
            .map_err(|e| format!("Bundlr upload error: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Bundlr error: {}", response.status()));
        }

        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse Bundlr response: {}", e))?;

        let tx_id = json["id"].as_str()
            .ok_or_else(|| "Missing tx id in response".to_string())?;

        Ok(format!("arweave://{}", tx_id))
    }

    pub async fn get(&self, tx_id: &str) -> Result<Vec<u8>, String> {
        let tx_id = tx_id.strip_prefix("arweave://").unwrap_or(tx_id);
        let url = format!("https://arweave.net/{}", tx_id);
        
        let client = reqwest::Client::new();
        let response = client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch from Arweave: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Arweave fetch error: {}", response.status()));
        }

        let bytes = response.bytes().await
            .map_err(|e| format!("Failed to read Arweave data: {}", e))?;

        Ok(bytes.to_vec())
    }
}

