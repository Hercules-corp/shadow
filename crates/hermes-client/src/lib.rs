use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientConfig {
    pub backend: String,
    pub network: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConvertResponse {
    pub message: String,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeployResponse {
    pub program: String,
    pub storage: String,
    pub domain: Option<String>,
    pub minted_token: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterDomainResponse {
    pub domain: String,
    pub program: String,
    pub storage: Option<String>,
    pub owner: Option<String>,
}

pub async fn convert_site(config: &ClientConfig, path: &str) -> Result<ConvertResponse> {
    let client = Client::new();
    let url = format!("{}/api/sdk/convert", config.backend);
    let body = serde_json::json!({ "path": path, "network": config.network });
    let resp = client.post(url).json(&body).send().await?;
    if resp.status().is_success() {
        Ok(resp.json().await?)
    } else {
        Err(anyhow!("convert failed: {}", resp.text().await?))
    }
}

pub async fn deploy_site(
    config: &ClientConfig,
    path: &str,
    domain: Option<&str>,
    mint_token: bool,
) -> Result<DeployResponse> {
    let client = Client::new();
    let url = format!("{}/api/sdk/deploy", config.backend);
    let body = serde_json::json!({
        "path": path,
        "network": config.network,
        "domain": domain,
        "mintToken": mint_token
    });
    let resp = client.post(url).json(&body).send().await?;
    if resp.status().is_success() {
        Ok(resp.json().await?)
    } else {
        Err(anyhow!("deploy failed: {}", resp.text().await?))
    }
}

pub async fn register_domain(
    config: &ClientConfig,
    domain: &str,
    program: &str,
) -> Result<RegisterDomainResponse> {
    let client = Client::new();
    let url = format!("{}/api/domains", config.backend);
    let body = serde_json::json!({
        "domain": domain,
        "program": program,
        "network": config.network
    });
    let resp = client.post(url).json(&body).send().await?;
    if resp.status().is_success() {
        Ok(resp.json().await?)
    } else {
        Err(anyhow!("register domain failed: {}", resp.text().await?))
    }
}


