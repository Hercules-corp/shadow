// Prometheus: Forethought and analytics - Site analytics and performance monitoring
use mongodb::{Collection, Database};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use futures_util::TryStreamExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SiteAnalytics {
    #[serde(rename = "_id")]
    pub domain: String,
    pub program_address: String,
    pub total_visits: i64,
    pub unique_visitors: i64,
    pub average_time_spent: f64,
    pub bounce_rate: f64,
    pub last_updated: DateTime<Utc>,
    pub daily_stats: Vec<DailyStats>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyStats {
    pub date: String,
    pub visits: i64,
    pub unique_visitors: i64,
    pub avg_time_spent: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceMetrics {
    #[serde(rename = "_id")]
    pub id: String,
    pub domain: String,
    pub load_time_ms: f64,
    pub render_time_ms: f64,
    pub total_size_bytes: i64,
    pub request_count: i32,
    pub measured_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEngagement {
    pub domain: String,
    pub wallet_pubkey: String,
    pub visit_count: i32,
    pub total_time_spent: i64,
    pub last_visit: DateTime<Utc>,
    pub favorite: bool,
}

pub struct PrometheusAnalytics {
    db: Database,
}

impl PrometheusAnalytics {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn get_analytics_collection(&self) -> Collection<SiteAnalytics> {
        self.db.collection::<SiteAnalytics>("site_analytics")
    }

    pub fn get_performance_collection(&self) -> Collection<PerformanceMetrics> {
        self.db.collection::<PerformanceMetrics>("performance_metrics")
    }

    pub fn get_engagement_collection(&self) -> Collection<UserEngagement> {
        self.db.collection::<UserEngagement>("user_engagement")
    }

    pub async fn record_visit(
        &self,
        domain: &str,
        program_address: &str,
        wallet: &str,
        time_spent_seconds: f64,
    ) -> Result<(), mongodb::error::Error> {
        // Update site analytics
        let analytics_col = self.get_analytics_collection();
        
        let filter = doc! { "_id": domain };
        let update = doc! {
            "$inc": {
                "total_visits": 1,
            },
            "$set": {
                "program_address": program_address,
                "last_updated": mongodb::bson::DateTime::from_millis(Utc::now().timestamp_millis()),
            },
            "$setOnInsert": {
                "domain": domain,
                "unique_visitors": 0,
                "average_time_spent": 0.0,
                "bounce_rate": 0.0,
                "daily_stats": mongodb::bson::Bson::Array(vec![]),
            }
        };
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();
        analytics_col.update_one(filter, update, options).await?;

        // Update user engagement
        let engagement_col = self.get_engagement_collection();
        let engagement_id = format!("{}:{}", domain, wallet);
        let filter = doc! { "_id": &engagement_id };
        let update = doc! {
            "$inc": {
                "visit_count": 1,
                "total_time_spent": time_spent_seconds as i64,
            },
            "$set": {
                "domain": domain,
                "wallet_pubkey": wallet,
                "last_visit": mongodb::bson::DateTime::from_millis(Utc::now().timestamp_millis()),
            },
            "$setOnInsert": {
                "favorite": false,
            }
        };
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();
        engagement_col.update_one(filter, update, options).await?;

        Ok(())
    }

    pub async fn record_performance(
        &self,
        domain: &str,
        load_time_ms: f64,
        render_time_ms: f64,
        total_size_bytes: i64,
        request_count: i32,
    ) -> Result<(), mongodb::error::Error> {
        let collection = self.get_performance_collection();
        let id = format!("{}:{}", domain, Utc::now().format("%Y-%m-%d"));
        let now = Utc::now();
        
        let metrics = PerformanceMetrics {
            id,
            domain: domain.to_string(),
            load_time_ms,
            render_time_ms,
            total_size_bytes,
            request_count,
            measured_at: now,
        };
        
        let filter = doc! { "_id": &metrics.id };
        let update = doc! {
            "$set": mongodb::bson::to_bson(&metrics).unwrap()
        };
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();
        
        collection.update_one(filter, update, options).await?;
        Ok(())
    }

    pub async fn get_analytics(
        &self,
        domain: &str,
    ) -> Result<Option<SiteAnalytics>, mongodb::error::Error> {
        let collection = self.get_analytics_collection();
        let filter = doc! { "_id": domain };
        let analytics = collection.find_one(filter, None).await?;
        Ok(analytics)
    }

    pub async fn get_top_sites(
        &self,
        limit: i64,
    ) -> Result<Vec<SiteAnalytics>, mongodb::error::Error> {
        let collection = self.get_analytics_collection();
        let options = mongodb::options::FindOptions::builder()
            .limit(limit)
            .sort(doc! { "total_visits": -1 })
            .build();
        
        let mut cursor = collection.find(doc! {}, options).await?;
        let mut sites = Vec::new();
        
        while let Some(site) = cursor.try_next().await? {
            sites.push(site);
        }
        
        Ok(sites)
    }

    pub async fn calculate_bounce_rate(
        &self,
        domain: &str,
    ) -> Result<f64, mongodb::error::Error> {
        let engagement_col = self.get_engagement_collection();
        let filter = doc! { "domain": domain };
        
        let total_visits = engagement_col.count_documents(filter.clone(), None).await? as f64;
        let single_visit = engagement_col.count_documents(
            doc! {
                "domain": domain,
                "visit_count": 1
            },
            None,
        ).await? as f64;
        
        if total_visits == 0.0 {
            return Ok(0.0);
        }
        
        Ok(single_visit / total_visits)
    }

    pub async fn update_analytics_summary(
        &self,
        domain: &str,
    ) -> Result<(), mongodb::error::Error> {
        let engagement_col = self.get_engagement_collection();
        let filter = doc! { "domain": domain };
        
        let mut cursor = engagement_col.find(filter, None).await?;
        let mut unique_visitors = 0;
        let mut total_time = 0.0;
        let mut visit_count = 0;
        
        while let Some(engagement) = cursor.try_next().await? {
            unique_visitors += 1;
            total_time += engagement.total_time_spent as f64;
            visit_count += engagement.visit_count as i64;
        }
        
        let avg_time = if unique_visitors > 0 {
            total_time / unique_visitors as f64
        } else {
            0.0
        };
        
        let bounce_rate = self.calculate_bounce_rate(domain).await.unwrap_or(0.0);
        
        let analytics_col = self.get_analytics_collection();
        let filter = doc! { "_id": domain };
        let update = doc! {
            "$set": {
                "unique_visitors": unique_visitors as i64,
                "average_time_spent": avg_time,
                "bounce_rate": bounce_rate,
                "total_visits": visit_count,
            }
        };
        
        analytics_col.update_one(filter, update, None).await?;
        Ok(())
    }
}

