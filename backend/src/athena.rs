// Athena: Wisdom and knowledge - Search indexing and content analysis
use mongodb::{Collection, Database};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use futures_util::TryStreamExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchIndex {
    #[serde(rename = "_id")]
    pub id: String,
    pub domain: String,
    pub program_address: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub content_hash: String,
    pub indexed_at: DateTime<Utc>,
    pub popularity_score: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentAnalysis {
    pub domain: String,
    pub word_count: usize,
    pub language: Option<String>,
    pub categories: Vec<String>,
    pub trust_score: f64,
    pub last_analyzed: DateTime<Utc>,
}

pub struct AthenaIndexer {
    db: Database,
}

impl AthenaIndexer {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn get_index_collection(&self) -> Collection<SearchIndex> {
        self.db.collection::<SearchIndex>("search_index")
    }

    pub fn get_analysis_collection(&self) -> Collection<ContentAnalysis> {
        self.db.collection::<ContentAnalysis>("content_analysis")
    }

    pub async fn index_site(
        &self,
        domain: &str,
        program_address: &str,
        title: Option<&str>,
        description: Option<&str>,
        content: &str,
    ) -> Result<(), mongodb::error::Error> {
        let collection = self.get_index_collection();
        
        // Extract keywords from content
        let keywords = Self::extract_keywords(content, title, description);
        
        // Calculate popularity score (simplified)
        let popularity_score = self.calculate_popularity(domain).await.unwrap_or(0.0);
        
        let now = Utc::now();
        let content_hash = Self::hash_content(content);
        
        let index = SearchIndex {
            id: format!("{}:{}", domain, program_address),
            domain: domain.to_string(),
            program_address: program_address.to_string(),
            title: title.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            keywords,
            content_hash,
            indexed_at: now,
            popularity_score,
        };
        
        let filter = doc! { "_id": &index.id };
        let update = doc! {
            "$set": mongodb::bson::to_bson(&index).unwrap()
        };
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();
        
        collection.update_one(filter, update, options).await?;
        Ok(())
    }

    pub async fn search(
        &self,
        query: &str,
        limit: i64,
    ) -> Result<Vec<SearchIndex>, mongodb::error::Error> {
        let collection = self.get_index_collection();
        
        let filter = doc! {
            "$or": [
                { "domain": { "$regex": query, "$options": "i" } },
                { "title": { "$regex": query, "$options": "i" } },
                { "description": { "$regex": query, "$options": "i" } },
                { "keywords": { "$in": [query] } }
            ]
        };
        
        let options = mongodb::options::FindOptions::builder()
            .limit(limit)
            .sort(doc! { "popularity_score": -1, "indexed_at": -1 })
            .build();
        
        let mut cursor = collection.find(filter, options).await?;
        let mut results = Vec::new();
        
        while let Some(doc) = cursor.try_next().await? {
            results.push(doc);
        }
        
        Ok(results)
    }

    pub async fn analyze_content(
        &self,
        domain: &str,
        content: &str,
    ) -> Result<ContentAnalysis, mongodb::error::Error> {
        let collection = self.get_analysis_collection();
        
        let word_count = content.split_whitespace().count();
        let language = Self::detect_language(content);
        let categories = Self::categorize_content(content);
        let trust_score = self.calculate_trust_score(domain).await.unwrap_or(0.5);
        
        let analysis = ContentAnalysis {
            domain: domain.to_string(),
            word_count,
            language,
            categories,
            trust_score,
            last_analyzed: Utc::now(),
        };
        
        let filter = doc! { "domain": domain };
        let update = doc! {
            "$set": mongodb::bson::to_bson(&analysis).unwrap()
        };
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();
        
        collection.update_one(filter, update, options).await?;
        Ok(analysis)
    }

    fn extract_keywords(content: &str, title: Option<&str>, description: Option<&str>) -> Vec<String> {
        let mut text = String::new();
        if let Some(t) = title {
            text.push_str(t);
            text.push(' ');
        }
        if let Some(d) = description {
            text.push_str(d);
            text.push(' ');
        }
        text.push_str(content);
        
        // Simple keyword extraction (stop words removed)
        let stop_words: Vec<&str> = vec!["the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by"];
        let words: Vec<String> = text
            .to_lowercase()
            .split_whitespace()
            .filter(|w| w.len() > 3 && !stop_words.contains(w))
            .map(|s| s.to_string())
            .collect();
        
        // Count frequency and take top keywords
        let mut word_count: HashMap<String, usize> = HashMap::new();
        for word in words {
            *word_count.entry(word).or_insert(0) += 1;
        }
        
        let mut keywords: Vec<(String, usize)> = word_count.into_iter().collect();
        keywords.sort_by(|a, b| b.1.cmp(&a.1));
        keywords.into_iter().take(10).map(|(word, _)| word).collect()
    }

    fn hash_content(content: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn detect_language(_content: &str) -> Option<String> {
        // Simplified language detection
        Some("en".to_string())
    }

    fn categorize_content(content: &str) -> Vec<String> {
        let content_lower = content.to_lowercase();
        let mut categories = Vec::new();
        
        if content_lower.contains("nft") || content_lower.contains("token") {
            categories.push("crypto".to_string());
        }
        if content_lower.contains("defi") || content_lower.contains("swap") {
            categories.push("defi".to_string());
        }
        if content_lower.contains("game") || content_lower.contains("play") {
            categories.push("gaming".to_string());
        }
        if content_lower.contains("art") || content_lower.contains("music") {
            categories.push("creative".to_string());
        }
        
        if categories.is_empty() {
            categories.push("general".to_string());
        }
        
        categories
    }

    async fn calculate_popularity(&self, _domain: &str) -> Result<f64, mongodb::error::Error> {
        // Calculate based on visits, bookmarks, etc.
        // Simplified for now
        Ok(1.0)
    }

    async fn calculate_trust_score(&self, _domain: &str) -> Result<f64, mongodb::error::Error> {
        // Calculate trust based on verification, age, etc.
        // Simplified for now
        Ok(0.8)
    }
}

