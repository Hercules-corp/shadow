// Link Converter Handlers

use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
use crate::error::ShadowError;
use crate::link_converter::{LinkConverter, ConvertLinkRequest, GeneralTokenRequest};
use crate::ares::AresAuth;
use mongodb::Database;
use std::sync::Arc;

pub async fn convert_link(
    db: web::Data<Database>,
    body: web::Json<ConvertLinkRequest>,
    solana_rpc: web::Data<String>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    // Verify authentication
    let _user_id = verify_auth(&req, &ares)?;

    let converter = LinkConverter::new(
        Arc::new(db.as_ref().clone()),
        solana_rpc.to_string(),
    );

    let result = converter
        .convert_link(&body.url, body.sublink.as_deref())
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn create_general_token(
    db: web::Data<Database>,
    body: web::Json<GeneralTokenRequest>,
    solana_rpc: web::Data<String>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    let _user_id = verify_auth(&req, &ares)?;

    let converter = LinkConverter::new(
        Arc::new(db.as_ref().clone()),
        solana_rpc.to_string(),
    );

    let token_mint = converter
        .create_general_token(&body.platform, &body.token_name, &body.token_symbol)
        .await
        .map_err(|e| ShadowError::BadRequest(e))?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "token_mint": token_mint,
        "platform": body.platform
    })))
}

pub async fn get_url_from_token(
    db: web::Data<Database>,
    path: web::Path<String>,
    solana_rpc: web::Data<String>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    verify_auth(&req, &ares)?;
    let token_mint = path.into_inner();

    let converter = LinkConverter::new(
        Arc::new(db.as_ref().clone()),
        solana_rpc.to_string(),
    );

    match converter.get_url_from_token(&token_mint).await
        .map_err(|e| ShadowError::BadRequest(e))? {
        Some(url) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "token_mint": token_mint,
            "original_url": url
        }))),
        None => Err(ShadowError::NotFound(format!("Token {} not found", token_mint)).into()),
    }
}

#[derive(serde::Deserialize)]
pub struct GetTokenFromUrlRequest {
    pub url: String,
}

pub async fn get_token_from_url(
    db: web::Data<Database>,
    body: web::Json<GetTokenFromUrlRequest>,
    solana_rpc: web::Data<String>,
    ares: web::Data<AresAuth>,
    req: HttpRequest,
) -> ActixResult<HttpResponse, ShadowError> {
    verify_auth(&req, &ares)?;

    let converter = LinkConverter::new(
        Arc::new(db.as_ref().clone()),
        solana_rpc.to_string(),
    );

    match converter.get_token_from_url(&body.url).await
        .map_err(|e| ShadowError::BadRequest(e))? {
        Some(token_mint) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "url": body.url,
            "token_mint": token_mint
        }))),
        None => Err(ShadowError::NotFound(format!("URL {} not found", body.url)).into()),
    }
}

fn verify_auth(req: &HttpRequest, ares: &AresAuth) -> Result<String, ShadowError> {
    use crate::ares::AuthHeader;
    
    let auth_header = req.headers().get("X-Shadow-Auth")
        .ok_or_else(|| ShadowError::Unauthorized)?
        .to_str()
        .map_err(|_| ShadowError::Unauthorized)?;
    
    let auth = AuthHeader::from_header(auth_header)
        .map_err(|_| ShadowError::Unauthorized)?;
    auth.verify(ares)
        .map_err(|_| ShadowError::Unauthorized)?;
    
    Ok(auth.wallet)
}

