// Chronos: Time and history - Browser history and session management
use mongodb::{Collection, Database};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::time::Duration;
use futures_util::TryStreamExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrowserHistory {
    #[serde(rename = "_id")]
    pub id: String,
    pub wallet_pubkey: String,
    pub domain: String,
    pub program_address: String,
    pub title: Option<String>,
    pub visited_at: DateTime<Utc>,
    pub visit_count: i32,
    pub last_visit: DateTime<Utc>,
    pub time_spent_seconds: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bookmark {
    #[serde(rename = "_id")]
    pub id: String,
    pub wallet_pubkey: String,
    pub domain: String,
    pub program_address: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub folder: Option<String>,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrowserSession {
    #[serde(rename = "_id")]
    pub session_id: String,
    pub wallet_pubkey: String,
    pub started_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub active_tabs: Vec<String>,
    pub total_visits: i32,
}

pub struct ChronosManager {
    db: Database,
}

impl ChronosManager {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn get_history_collection(&self) -> Collection<BrowserHistory> {
        self.db.collection::<BrowserHistory>("browser_history")
    }

    pub fn get_bookmarks_collection(&self) -> Collection<Bookmark> {
        self.db.collection::<Bookmark>("bookmarks")
    }

    pub fn get_sessions_collection(&self) -> Collection<BrowserSession> {
        self.db.collection::<BrowserSession>("browser_sessions")
    }

    pub async fn record_visit(
        &self,
        wallet: &str,
        domain: &str,
        program_address: &str,
        title: Option<&str>,
        time_spent: Duration,
    ) -> Result<(), mongodb::error::Error> {
        let collection = self.get_history_collection();
        let id = format!("{}:{}", wallet, domain);
        let now = Utc::now();
        let bson_now = mongodb::bson::DateTime::from_millis(now.timestamp_millis());
        
        let filter = doc! { "_id": &id };
        let update = doc! {
            "$set": {
                "wallet_pubkey": wallet,
                "domain": domain,
                "program_address": program_address,
                "title": title,
                "last_visit": bson_now,
            },
            "$inc": {
                "visit_count": 1,
                "time_spent_seconds": time_spent.as_secs() as i64,
            },
            "$setOnInsert": {
                "visited_at": bson_now,
            }
        };
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();
        
        collection.update_one(filter, update, options).await?;
        Ok(())
    }

    pub async fn get_history(
        &self,
        wallet: &str,
        limit: i64,
    ) -> Result<Vec<BrowserHistory>, mongodb::error::Error> {
        let collection = self.get_history_collection();
        let filter = doc! { "wallet_pubkey": wallet };
        let options = mongodb::options::FindOptions::builder()
            .limit(limit)
            .sort(doc! { "last_visit": -1 })
            .build();
        
        let mut cursor = collection.find(filter, options).await?;
        let mut history = Vec::new();
        
        while let Some(item) = cursor.try_next().await? {
            history.push(item);
        }
        
        Ok(history)
    }

    pub async fn clear_history(&self, wallet: &str) -> Result<(), mongodb::error::Error> {
        let collection = self.get_history_collection();
        let filter = doc! { "wallet_pubkey": wallet };
        collection.delete_many(filter, None).await?;
        Ok(())
    }

    pub async fn add_bookmark(
        &self,
        wallet: &str,
        domain: &str,
        program_address: &str,
        title: Option<&str>,
        description: Option<&str>,
        folder: Option<&str>,
        tags: Vec<String>,
    ) -> Result<(), mongodb::error::Error> {
        let collection = self.get_bookmarks_collection();
        let id = format!("{}:{}", wallet, domain);
        let now = Utc::now();
        
        let bookmark = Bookmark {
            id,
            wallet_pubkey: wallet.to_string(),
            domain: domain.to_string(),
            program_address: program_address.to_string(),
            title: title.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            folder: folder.map(|s| s.to_string()),
            created_at: now,
            tags,
        };
        
        let filter = doc! { "_id": &bookmark.id };
        let update = doc! {
            "$set": mongodb::bson::to_bson(&bookmark).unwrap()
        };
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();
        
        collection.update_one(filter, update, options).await?;
        Ok(())
    }

    pub async fn get_bookmarks(
        &self,
        wallet: &str,
        folder: Option<&str>,
    ) -> Result<Vec<Bookmark>, mongodb::error::Error> {
        let collection = self.get_bookmarks_collection();
        let mut filter = doc! { "wallet_pubkey": wallet };
        if let Some(f) = folder {
            filter.insert("folder", f);
        }
        
        let options = mongodb::options::FindOptions::builder()
            .sort(doc! { "created_at": -1 })
            .build();
        
        let mut cursor = collection.find(filter, options).await?;
        let mut bookmarks = Vec::new();
        
        while let Some(bookmark) = cursor.try_next().await? {
            bookmarks.push(bookmark);
        }
        
        Ok(bookmarks)
    }

    pub async fn remove_bookmark(&self, wallet: &str, domain: &str) -> Result<(), mongodb::error::Error> {
        let collection = self.get_bookmarks_collection();
        let id = format!("{}:{}", wallet, domain);
        let filter = doc! { "_id": id };
        collection.delete_one(filter, None).await?;
        Ok(())
    }

    pub async fn create_session(
        &self,
        wallet: &str,
    ) -> Result<String, mongodb::error::Error> {
        let collection = self.get_sessions_collection();
        let session_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let session = BrowserSession {
            session_id: session_id.clone(),
            wallet_pubkey: wallet.to_string(),
            started_at: now,
            last_activity: now,
            active_tabs: Vec::new(),
            total_visits: 0,
        };
        
        let filter = doc! { "_id": &session_id };
        let update = doc! {
            "$set": mongodb::bson::to_bson(&session).unwrap()
        };
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();
        
        collection.update_one(filter, update, options).await?;
        Ok(session_id)
    }

    pub async fn update_session_activity(
        &self,
        session_id: &str,
    ) -> Result<(), mongodb::error::Error> {
        let collection = self.get_sessions_collection();
        let now = Utc::now();
        let bson_now = mongodb::bson::DateTime::from_millis(now.timestamp_millis());
        
        let filter = doc! { "_id": session_id };
        let update = doc! {
            "$set": { "last_activity": bson_now },
            "$inc": { "total_visits": 1 }
        };
        
        collection.update_one(filter, update, None).await?;
        Ok(())
    }

    pub async fn get_active_sessions(
        &self,
        wallet: &str,
    ) -> Result<Vec<BrowserSession>, mongodb::error::Error> {
        let collection = self.get_sessions_collection();
        let filter = doc! {
            "wallet_pubkey": wallet,
            "last_activity": {
                "$gte": mongodb::bson::DateTime::from_millis(
                    (Utc::now() - chrono::Duration::hours(1)).timestamp_millis()
                )
            }
        };
        
        let mut cursor = collection.find(filter, None).await?;
        let mut sessions = Vec::new();
        
        while let Some(session) = cursor.try_next().await? {
            sessions.push(session);
        }
        
        Ok(sessions)
    }
}

