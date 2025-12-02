use actix_web::{web, HttpResponse, Result as ActixResult};
use crate::db;
use crate::error::ShadowError;
use crate::storage::{PinataStorage, BundlrStorage};
use crate::solana::SolanaClient;
use serde::{Deserialize, Serialize};
use mongodb::Database;

#[derive(Deserialize)]
pub struct CreateProfileRequest {
    pub wallet: String,
    pub profile_cid: String,
    pub is_public: bool,
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub profile_cid: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Serialize)]
pub struct ProfileResponse {
    pub wallet_pubkey: String,
    pub profile_cid: Option<String>,
    pub is_public: bool,
    pub exists: bool,
}

pub async fn search_profiles(
    db: web::Data<Database>,
    query: web::Query<SearchQuery>,
) -> ActixResult<HttpResponse, ShadowError> {
    let users = db::search_users(&db, &query.q, query.limit.unwrap_or(10))
        .await?;

    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_profile(
    db: web::Data<Database>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse, ShadowError> {
    let wallet = path.into_inner();
    
    match db::get_user(&db, &wallet).await? {
        Some(user) => {
            Ok(HttpResponse::Ok().json(ProfileResponse {
                wallet_pubkey: user.wallet_pubkey.clone(),
                profile_cid: user.profile_cid.clone(),
                is_public: user.is_public,
                exists: true,
            }))
        }
        None => {
            // Return exists: false if not found, but don't error
            Ok(HttpResponse::Ok().json(ProfileResponse {
                wallet_pubkey: wallet,
                profile_cid: None,
                is_public: false,
                exists: false,
            }))
        }
    }
}

pub async fn create_profile_route(
    db: web::Data<Database>,
    body: web::Json<CreateProfileRequest>,
) -> ActixResult<HttpResponse, ShadowError> {
    db::create_or_update_user(
        &db,
        &body.wallet,
        Some(&body.profile_cid),
        body.is_public,
    ).await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "success": true,
        "wallet": body.wallet
    })))
}

pub async fn update_profile(
    db: web::Data<Database>,
    path: web::Path<String>,
    body: web::Json<UpdateProfileRequest>,
) -> ActixResult<HttpResponse, ShadowError> {
    let wallet = path.into_inner();
    
    let user = db::get_user(&db, &wallet).await?
        .ok_or_else(|| ShadowError::NotFound("Profile not found".to_string()))?;

    let profile_cid = body.profile_cid.as_ref().map(|s| s.as_str()).or(user.profile_cid.as_deref());
    let is_public = body.is_public.unwrap_or(user.is_public);

    db::create_or_update_user(&db, &wallet, profile_cid, is_public).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true
    })))
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default)]
    pub limit: Option<i64>,
}

#[derive(Deserialize)]
pub struct RegisterSiteRequest {
    pub owner_pubkey: String,
    pub storage_cid: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn search_sites(
    db: web::Data<Database>,
    query: web::Query<SearchQuery>,
) -> ActixResult<HttpResponse, ShadowError> {
    let sites = db::search_sites(&db, &query.q, query.limit.unwrap_or(10))
        .await?;

    Ok(HttpResponse::Ok().json(sites))
}

pub async fn get_site(
    db: web::Data<Database>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse, ShadowError> {
    let program_address = path.into_inner();
    
    let site = db::get_site(&db, &program_address).await?
        .ok_or_else(|| ShadowError::NotFound("Site not found".to_string()))?;

    Ok(HttpResponse::Ok().json(site))
}

pub async fn register_site(
    db: web::Data<Database>,
    body: web::Json<RegisterSiteRequest>,
) -> ActixResult<HttpResponse, ShadowError> {
    // In production, verify program_address exists on-chain
    let program_address = format!("{}", uuid::Uuid::new_v4()); // Placeholder
    
    db::create_or_update_site(
        &db,
        &program_address,
        &body.owner_pubkey,
        &body.storage_cid,
        body.name.as_deref(),
        body.description.as_deref(),
    ).await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "success": true,
        "program_address": program_address
    })))
}

pub async fn update_site(
    db: web::Data<Database>,
    path: web::Path<String>,
    body: web::Json<RegisterSiteRequest>,
) -> ActixResult<HttpResponse, ShadowError> {
    let program_address = path.into_inner();
    
    db::create_or_update_site(
        &db,
        &program_address,
        &body.owner_pubkey,
        &body.storage_cid,
        body.name.as_deref(),
        body.description.as_deref(),
    ).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true
    })))
}

pub async fn get_site_content(
    db: web::Data<Database>,
    pinata: web::Data<PinataStorage>,
    bundlr: web::Data<BundlrStorage>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse, ShadowError> {
    let program_address = path.into_inner();
    
    let site = db::get_site(&db, &program_address).await?
        .ok_or_else(|| ShadowError::NotFound("Site not found".to_string()))?;

    let content = if site.storage_cid.starts_with("ipfs://") {
        pinata.get(&site.storage_cid).await
            .map_err(|e| ShadowError::Storage(e))?
    } else if site.storage_cid.starts_with("arweave://") {
        bundlr.get(&site.storage_cid).await
            .map_err(|e| ShadowError::Storage(e))?
    } else {
        return Err(ShadowError::BadRequest("Invalid storage CID".to_string()));
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(content))
}

pub async fn upload_ipfs(
    pinata: web::Data<PinataStorage>,
    body: web::Bytes,
) -> ActixResult<HttpResponse, ShadowError> {
    let cid = pinata.upload(&body, "upload").await
        .map_err(|e| ShadowError::Storage(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "cid": cid
    })))
}

pub async fn upload_arweave(
    bundlr: web::Data<BundlrStorage>,
    body: web::Bytes,
) -> ActixResult<HttpResponse, ShadowError> {
    let tx_id = bundlr.upload(&body, vec![]).await
        .map_err(|e| ShadowError::Storage(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "tx_id": tx_id
    })))
}

pub async fn search_solana(
    solana_rpc_url: web::Data<String>,
    query: web::Query<SearchQuery>,
) -> ActixResult<HttpResponse, ShadowError> {
    let client = SolanaClient::new(solana_rpc_url.to_string());
    
    // Try to parse as pubkey first
    if let Ok(account) = client.search_account(&query.q) {
        if let Some(acc) = account {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "type": "account",
                "data": acc
            })));
        }
    }

    // Try as program
    if let Ok(program) = client.search_program(&query.q) {
        if let Some(prog) = program {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "type": "program",
                "data": prog
            })));
        }
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "type": "none",
        "data": null
    })))
}
