mod api;
mod db;
mod error;
mod handlers;
mod storage;
mod websocket;
mod solana;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use mongodb::{Client as MongoClient, options::ClientOptions, IndexModel};
use std::env;
use std::sync::Arc;
use tracing_subscriber;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let client_options = ClientOptions::parse(&database_url).await?;
    let client = MongoClient::with_options(client_options)?;
    let db = Arc::new(client.database("shadow"));
    
    // Create indexes for better performance
    let users_collection = db.collection::<db::User>("users");
    let users_index = IndexModel::builder()
        .keys(mongodb::bson::doc! { "is_public": 1, "_id": 1 })
        .build();
    users_collection.create_index(users_index, None).await?;
    
    let sites_collection = db.collection::<db::Site>("sites");
    let sites_index = IndexModel::builder()
        .keys(mongodb::bson::doc! { "created_at": -1 })
        .build();
    sites_collection.create_index(sites_index, None).await?;

    let solana_rpc_url = env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    
    let solana_ws_url = env::var("SOLANA_WS_URL")
        .unwrap_or_else(|_| "wss://api.devnet.solana.com".to_string());

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()?;

    println!("🚀 Shadow backend starting on port {}", port);

    let db_clone = Arc::clone(&db);
    let solana_rpc_clone = solana_rpc_url.clone();
    let solana_ws_clone = solana_ws_url.clone();
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::from(Arc::clone(&db_clone)))
            .app_data(web::Data::new(solana_rpc_clone.clone()))
            .app_data(web::Data::new(solana_ws_clone.clone()))
            .app_data(web::Data::new(storage::PinataStorage::new()))
            .app_data(web::Data::new(storage::BundlrStorage::new()))
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(api::health))
                    .route("/profiles/search", web::get().to(handlers::search_profiles))
                    .route("/profiles/{wallet}", web::get().to(handlers::get_profile))
                    .route("/profiles", web::post().to(handlers::create_profile_route))
                    .route("/profiles/{wallet}", web::put().to(handlers::update_profile))
                    .route("/sites/search", web::get().to(handlers::search_sites))
                    .route("/sites/{program_address}", web::get().to(handlers::get_site))
                    .route("/sites", web::post().to(handlers::register_site))
                    .route("/sites/{program_address}", web::put().to(handlers::update_site))
                    .route("/sites/{program_address}/content", web::get().to(handlers::get_site_content))
                    .route("/upload/ipfs", web::post().to(handlers::upload_ipfs))
                    .route("/upload/arweave", web::post().to(handlers::upload_arweave))
                    .route("/solana/search", web::get().to(handlers::search_solana))
                    .route("/ws", web::get().to(websocket::ws_handler))
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
