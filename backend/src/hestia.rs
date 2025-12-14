// Hestia - Goddess of Home and Connections
// Handles dApp connections, permissions, and session management

use mongodb::{Collection, Database};
use mongodb::bson::{doc, DateTime};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DAppConnection {
    #[serde(rename = "_id")]
    pub id: String,
    pub user_id: String,
    pub wallet_id: String,
    pub dapp_origin: String, // e.g., "https://example.com"
    pub dapp_name: String,
    pub dapp_icon: Option<String>,
    pub permissions: Vec<Permission>,
    pub connected_at: DateTime,
    pub last_used: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Permission {
    ViewBalance,
    RequestTransaction,
    SignMessage,
    ViewPublicKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectDAppRequest {
    pub wallet_id: String,
    pub dapp_origin: String,
    pub dapp_name: String,
    pub dapp_icon: Option<String>,
    pub requested_permissions: Vec<Permission>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DAppConnectionResponse {
    pub id: String,
    pub dapp_origin: String,
    pub dapp_name: String,
    pub dapp_icon: Option<String>,
    pub permissions: Vec<Permission>,
    pub connected_at: DateTime,
}

pub struct HestiaConnectionManager {
    db: Arc<Database>,
}

impl HestiaConnectionManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn get_collection(&self) -> Collection<DAppConnection> {
        self.db.collection::<DAppConnection>("dapp_connections")
    }

    /// Connect a dApp to a wallet
    pub async fn connect_dapp(
        &self,
        user_id: &str,
        wallet_id: &str,
        dapp_origin: &str,
        dapp_name: &str,
        dapp_icon: Option<&str>,
        requested_permissions: Vec<Permission>,
    ) -> Result<DAppConnectionResponse, String> {
        // Check if already connected
        let collection = self.get_collection();
        let existing = collection
            .find_one(
                doc! {
                    "user_id": user_id,
                    "wallet_id": wallet_id,
                    "dapp_origin": dapp_origin
                },
                None,
            )
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if let Some(mut conn) = existing {
            // Update existing connection
            conn.permissions = requested_permissions.clone();
            conn.last_used = DateTime::now();

            // Convert permissions to BSON
            let permissions_bson: Vec<mongodb::bson::Bson> = conn.permissions.iter()
                .map(|p| mongodb::bson::to_bson(p).unwrap_or(mongodb::bson::Bson::String(format!("{:?}", p))))
                .collect();
            
            collection
                .update_one(
                    doc! { "_id": &conn.id },
                    doc! {
                        "$set": {
                            "permissions": permissions_bson,
                            "last_used": conn.last_used
                        }
                    },
                    None,
                )
                .await
                .map_err(|e| format!("Database error: {}", e))?;

            Ok(DAppConnectionResponse {
                id: conn.id,
                dapp_origin: conn.dapp_origin,
                dapp_name: conn.dapp_name,
                dapp_icon: conn.dapp_icon,
                permissions: conn.permissions,
                connected_at: conn.connected_at,
            })
        } else {
            // Create new connection
            let connection = DAppConnection {
                id: uuid::Uuid::new_v4().to_string(),
                user_id: user_id.to_string(),
                wallet_id: wallet_id.to_string(),
                dapp_origin: dapp_origin.to_string(),
                dapp_name: dapp_name.to_string(),
                dapp_icon: dapp_icon.map(|s| s.to_string()),
                permissions: requested_permissions.clone(),
                connected_at: DateTime::now(),
                last_used: DateTime::now(),
            };

            collection
                .insert_one(&connection, None)
                .await
                .map_err(|e| format!("Database error: {}", e))?;

            Ok(DAppConnectionResponse {
                id: connection.id.clone(),
                dapp_origin: connection.dapp_origin,
                dapp_name: connection.dapp_name,
                dapp_icon: connection.dapp_icon,
                permissions: connection.permissions,
                connected_at: connection.connected_at,
            })
        }
    }

    /// Disconnect a dApp
    pub async fn disconnect_dapp(
        &self,
        user_id: &str,
        connection_id: &str,
    ) -> Result<(), String> {
        let collection = self.get_collection();

        collection
            .delete_one(
                doc! {
                    "_id": connection_id,
                    "user_id": user_id
                },
                None,
            )
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(())
    }

    /// Get all connected dApps for a user
    pub async fn get_connections(
        &self,
        user_id: &str,
    ) -> Result<Vec<DAppConnectionResponse>, String> {
        let collection = self.get_collection();
        let filter = doc! { "user_id": user_id };

        let mut connections = Vec::new();
        let mut cursor = collection
            .find(filter, None)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        while let Some(conn) = cursor.try_next().await
            .map_err(|e| format!("Database error: {}", e))? {
            connections.push(DAppConnectionResponse {
                id: conn.id,
                dapp_origin: conn.dapp_origin,
                dapp_name: conn.dapp_name,
                dapp_icon: conn.dapp_icon,
                permissions: conn.permissions,
                connected_at: conn.connected_at,
            });
        }

        Ok(connections)
    }

    /// Check if dApp has permission
    pub async fn has_permission(
        &self,
        user_id: &str,
        wallet_id: &str,
        dapp_origin: &str,
        permission: &Permission,
    ) -> Result<bool, String> {
        let collection = self.get_collection();

        if let Some(conn) = collection
            .find_one(
                doc! {
                    "user_id": user_id,
                    "wallet_id": wallet_id,
                    "dapp_origin": dapp_origin
                },
                None,
            )
            .await
            .map_err(|e| format!("Database error: {}", e))? {
            Ok(conn.permissions.contains(permission))
        } else {
            Ok(false)
        }
    }

    /// Update last used timestamp
    pub async fn update_last_used(
        &self,
        connection_id: &str,
    ) -> Result<(), String> {
        let collection = self.get_collection();

        collection
            .update_one(
                doc! { "_id": connection_id },
                doc! { "$set": { "last_used": DateTime::now() } },
                None,
            )
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(())
    }
}

use futures_util::TryStreamExt;


