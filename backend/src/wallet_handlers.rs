// Wallet dApp API Handlers
// All endpoints for Phantom-like wallet functionality

use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use crate::error::ShadowError;
use crate::zeus::{ZeusWalletManager, CreateWalletRequest, ImportWalletRequest};
use crate::poseidon::{PoseidonTransactionManager, SignTransactionRequest, CreateTransactionRequest};
use crate::dionysus::DionysusTokenManager;
use crate::aphrodite::AphroditeNFTManager;
use crate::hestia::{HestiaConnectionManager, ConnectDAppRequest};
use crate::plutus::PlutusPortfolioManager;
use crate::ares::AresAuth;
use mongodb::Database;
use serde::Deserialize;

// ========== Zeus (Wallet Management) ==========

pub async fn create_wallet(
    db: web::Data<Database>,
    body: web::Json<CreateWalletRequest>,
    solana_rpc: web::Data<String>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    // Verify authentication
    let user_id = verify_auth(&req, &ares)?;

    let manager = ZeusWalletManager::new(
        Arc::new(db.as_ref().clone()),
        solana_rpc.to_string(),
    );

    let wallet = manager
        .create_wallet(&user_id, &body.name, &body.password)
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Created().json(wallet))
}

pub async fn import_wallet(
    db: web::Data<Database>,
    body: web::Json<ImportWalletRequest>,
    solana_rpc: web::Data<String>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    let user_id = verify_auth(&req, &ares)?;

    let manager = ZeusWalletManager::new(
        Arc::new(db.as_ref().clone()),
        solana_rpc.to_string(),
    );

    let wallet = manager
        .import_wallet(&user_id, &body.name, &body.private_key, &body.password)
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Created().json(wallet))
}

pub async fn list_wallets(
    db: web::Data<Database>,
    solana_rpc: web::Data<String>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    let user_id = verify_auth(&req, &ares)?;

    let manager = ZeusWalletManager::new(
        Arc::new(db.as_ref().clone()),
        solana_rpc.to_string(),
    );

    let wallets = manager
        .list_wallets(&user_id)
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Ok().json(wallets))
}

pub async fn get_active_wallet(
    db: web::Data<Database>,
    solana_rpc: web::Data<String>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    let user_id = verify_auth(&req, &ares)?;

    let manager = ZeusWalletManager::new(
        Arc::new(db.as_ref().clone()),
        solana_rpc.to_string(),
    );

    match manager.get_active_wallet(&user_id).await
        .map_err(|e| ShadowError::BadRequest(e))? {
        Some(wallet) => Ok(HttpResponse::Ok().json(wallet)),
        None => Err(ShadowError::NotFound("No active wallet".to_string()).into()),
    }
}

#[derive(Deserialize)]
pub struct SetActiveWalletRequest {
    pub wallet_id: String,
}

pub async fn set_active_wallet(
    db: web::Data<Database>,
    body: web::Json<SetActiveWalletRequest>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    let user_id = verify_auth(&req, &ares)?;

    let manager = ZeusWalletManager::new(
        Arc::new(db.as_ref().clone()),
        "".to_string(), // Not needed for this operation
    );

    manager
        .set_active_wallet(&user_id, &body.wallet_id)
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "success": true })))
}

// ========== Poseidon (Transaction Signing) ==========

pub async fn create_transaction(
    db: web::Data<Database>,
    body: web::Json<CreateTransactionRequest>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    let user_id = verify_auth(&req, &ares)?;

    let manager = PoseidonTransactionManager::new(Arc::new(db.as_ref().clone()));

    let tx = manager
        .create_transaction(
            &user_id,
            &body.wallet_id,
            &body.dapp_origin,
            &body.transaction_data,
            body.message.as_deref(),
        )
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Created().json(tx))
}

pub async fn sign_transaction(
    db: web::Data<Database>,
    body: web::Json<SignTransactionRequest>,
    _solana_rpc: web::Data<String>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    let user_id = verify_auth(&req, &ares)?;

    let poseidon = PoseidonTransactionManager::new(Arc::new(db.as_ref().clone()));

    // Get transaction to find wallet_id
    let _tx = poseidon
        .get_transaction(&body.transaction_id, &user_id)
        .await
        .map_err(|e| ShadowError::BadRequest(e))?
        .ok_or_else(|| ShadowError::NotFound("Transaction not found".to_string()))?;

    // Get private key (requires password)
    // In production, get wallet_id from transaction and decrypt using password
    // For now, return error - this needs proper implementation
    return Err(ShadowError::BadRequest("Transaction signing requires wallet decryption - not yet implemented".to_string()).into());
}

pub async fn get_pending_transactions(
    db: web::Data<Database>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    let user_id = verify_auth(&req, &ares)?;

    let manager = PoseidonTransactionManager::new(Arc::new(db.as_ref().clone()));

    let transactions = manager
        .get_pending_transactions(&user_id)
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Ok().json(transactions))
}

// ========== Dionysus (Tokens) ==========

pub async fn get_token_balances(
    path: web::Path<String>,
    db: web::Data<Database>,
    solana_rpc: web::Data<String>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    verify_auth(&req, &ares)?;
    let wallet_pubkey = path.into_inner();

    let manager = DionysusTokenManager::new(
        Arc::new(db.as_ref().clone()),
        solana_rpc.to_string(),
    );

    let balances = manager
        .get_token_balances(&wallet_pubkey)
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Ok().json(balances))
}

// ========== Aphrodite (NFTs) ==========

pub async fn get_nfts(
    path: web::Path<String>,
    db: web::Data<Database>,
    solana_rpc: web::Data<String>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    verify_auth(&req, &ares)?;
    let wallet_pubkey = path.into_inner();

    let manager = AphroditeNFTManager::new(
        Arc::new(db.as_ref().clone()),
        solana_rpc.to_string(),
    );

    let nfts = manager
        .get_nfts(&wallet_pubkey)
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Ok().json(nfts))
}

// ========== Hestia (dApp Connections) ==========

pub async fn connect_dapp(
    db: web::Data<Database>,
    body: web::Json<ConnectDAppRequest>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    let user_id = verify_auth(&req, &ares)?;

    let manager = HestiaConnectionManager::new(Arc::new(db.as_ref().clone()));

    let connection = manager
        .connect_dapp(
            &user_id,
            &body.wallet_id,
            &body.dapp_origin,
            &body.dapp_name,
            body.dapp_icon.as_deref(),
            body.requested_permissions.clone(),
        )
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Created().json(connection))
}

pub async fn get_connections(
    db: web::Data<Database>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    let user_id = verify_auth(&req, &ares)?;

    let manager = HestiaConnectionManager::new(Arc::new(db.as_ref().clone()));

    let connections = manager
        .get_connections(&user_id)
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Ok().json(connections))
}

#[derive(Deserialize)]
pub struct DisconnectDAppRequest {
    pub connection_id: String,
}

pub async fn disconnect_dapp(
    db: web::Data<Database>,
    body: web::Json<DisconnectDAppRequest>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    let user_id = verify_auth(&req, &ares)?;

    let manager = HestiaConnectionManager::new(Arc::new(db.as_ref().clone()));

    manager
        .disconnect_dapp(&user_id, &body.connection_id)
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "success": true })))
}

// ========== Plutus (Portfolio) ==========

pub async fn get_portfolio(
    path: web::Path<String>,
    db: web::Data<Database>,
    solana_rpc: web::Data<String>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    verify_auth(&req, &ares)?;
    let wallet_pubkey = path.into_inner();

    let manager = PlutusPortfolioManager::new(
        Arc::new(db.as_ref().clone()),
        solana_rpc.to_string(),
    );

    let portfolio = manager
        .get_portfolio(&wallet_pubkey)
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Ok().json(portfolio))
}

pub async fn get_transaction_history(
    path: web::Path<String>,
    query: web::Query<std::collections::HashMap<String, String>>,
    db: web::Data<Database>,
    solana_rpc: web::Data<String>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    verify_auth(&req, &ares)?;
    let wallet_pubkey = path.into_inner();
    let limit = query.get("limit")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(50);

    let manager = PlutusPortfolioManager::new(
        Arc::new(db.as_ref().clone()),
        solana_rpc.to_string(),
    );

    let history = manager
        .get_transaction_history(&wallet_pubkey, Some(limit))
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Ok().json(history))
}

// Helper function to verify authentication
fn verify_auth(req: &HttpRequest, ares: &AresAuth) -> Result<String, ShadowError> {
    use crate::ares::AuthHeader;
    
    let auth_header = req.headers().get("X-Shadow-Auth")
        .ok_or_else(|| ShadowError::Unauthorized)?
        .to_str()
        .map_err(|_| ShadowError::Unauthorized)?;
    
    let auth = AuthHeader::from_header(auth_header)?;
    auth.verify(ares)
        .map_err(|_| ShadowError::Unauthorized)?;
    
    Ok(auth.wallet) // Using wallet as user_id for now
}

use std::sync::Arc;


