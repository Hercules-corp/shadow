use mongodb::{Collection, Database};
use mongodb::bson::doc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use futures_util::TryStreamExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub wallet_pubkey: String,
    pub profile_cid: Option<String>,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Site {
    #[serde(rename = "_id")]
    pub program_address: String,
    pub owner_pubkey: String,
    pub storage_cid: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn get_users_collection(db: &Database) -> Collection<User> {
    db.collection::<User>("users")
}

pub fn get_sites_collection(db: &Database) -> Collection<Site> {
    db.collection::<Site>("sites")
}

pub async fn get_user(db: &Database, wallet: &str) -> Result<Option<User>, mongodb::error::Error> {
    let collection = get_users_collection(db);
    let filter = doc! { "_id": wallet };
    let user = collection.find_one(filter, None).await?;
    Ok(user)
}

pub async fn search_users(
    db: &Database,
    query: &str,
    limit: i64,
) -> Result<Vec<User>, mongodb::error::Error> {
    let collection = get_users_collection(db);
    let filter = doc! {
        "is_public": true,
        "_id": { "$regex": query, "$options": "i" }
    };
    let options = mongodb::options::FindOptions::builder()
        .limit(limit)
        .build();
    
    let mut cursor = collection.find(filter, options).await?;
    let mut users = Vec::new();
    
    while let Some(user) = cursor.try_next().await? {
        users.push(user);
    }
    
    Ok(users)
}

pub async fn create_or_update_user(
    db: &Database,
    wallet: &str,
    profile_cid: Option<&str>,
    is_public: bool,
) -> Result<(), mongodb::error::Error> {
    let collection = get_users_collection(db);
    let now = Utc::now();
    
    let filter = doc! { "_id": wallet };
    // Convert chrono::DateTime to mongodb::bson::DateTime
    // Using timestamp_millis() for accurate millisecond conversion
    let bson_now = mongodb::bson::DateTime::from_millis(now.timestamp_millis());
    let update = doc! {
        "$set": {
            "profile_cid": profile_cid,
            "is_public": is_public,
            "updated_at": bson_now
        },
        "$setOnInsert": {
            "created_at": bson_now
        }
    };
    let options = mongodb::options::UpdateOptions::builder()
        .upsert(true)
        .build();
    
    collection.update_one(filter, update, options).await?;
    Ok(())
}

pub async fn get_site(db: &Database, program_address: &str) -> Result<Option<Site>, mongodb::error::Error> {
    let collection = get_sites_collection(db);
    let filter = doc! { "_id": program_address };
    let site = collection.find_one(filter, None).await?;
    Ok(site)
}

pub async fn search_sites(
    db: &Database,
    query: &str,
    limit: i64,
) -> Result<Vec<Site>, mongodb::error::Error> {
    let collection = get_sites_collection(db);
    let filter = doc! {
        "$or": [
            { "name": { "$regex": query, "$options": "i" } },
            { "description": { "$regex": query, "$options": "i" } },
            { "_id": { "$regex": query, "$options": "i" } }
        ]
    };
    let options = mongodb::options::FindOptions::builder()
        .limit(limit)
        .sort(doc! { "created_at": -1 })
        .build();
    
    let mut cursor = collection.find(filter, options).await?;
    let mut sites = Vec::new();
    
    while let Some(site) = cursor.try_next().await? {
        sites.push(site);
    }
    
    Ok(sites)
}

pub async fn create_or_update_site(
    db: &Database,
    program_address: &str,
    owner_pubkey: &str,
    storage_cid: &str,
    name: Option<&str>,
    description: Option<&str>,
) -> Result<(), mongodb::error::Error> {
    let collection = get_sites_collection(db);
    let now = Utc::now();
    
    let filter = doc! { "_id": program_address };
    // Convert chrono::DateTime to mongodb::bson::DateTime
    // Using timestamp_millis() for accurate millisecond conversion
    let bson_now = mongodb::bson::DateTime::from_millis(now.timestamp_millis());
    let update = doc! {
        "$set": {
            "owner_pubkey": owner_pubkey,
            "storage_cid": storage_cid,
            "name": name,
            "description": description,
            "updated_at": bson_now
        },
        "$setOnInsert": {
            "created_at": bson_now
        }
    };
    let options = mongodb::options::UpdateOptions::builder()
        .upsert(true)
        .build();
    
    collection.update_one(filter, update, options).await?;
    Ok(())
}
