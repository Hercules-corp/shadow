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

    fn detect_language(content: &str) -> Option<String> {
        // Simple language detection based on common words
        let content_lower = content.to_lowercase();
        
        // English common words
        let english_words = ["the", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by"];
        let english_count = english_words.iter()
            .filter(|&&word| content_lower.contains(word))
            .count();
        
        // Spanish common words
        let spanish_words = ["el", "la", "de", "que", "y", "a", "en", "un", "ser", "se", "no", "haber"];
        let spanish_count = spanish_words.iter()
            .filter(|&&word| content_lower.contains(word))
            .count();
        
        // French common words
        let french_words = ["le", "de", "et", "à", "un", "il", "être", "et", "en", "avoir", "que", "pour"];
        let french_count = french_words.iter()
            .filter(|&&word| content_lower.contains(word))
            .count();
        
        // Determine language based on word frequency
        if english_count >= spanish_count && english_count >= french_count && english_count > 0 {
            Some("en".to_string())
        } else if spanish_count >= french_count && spanish_count > 0 {
            Some("es".to_string())
        } else if french_count > 0 {
            Some("fr".to_string())
        } else {
            // Default to English if no matches
            Some("en".to_string())
        }
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

    async fn calculate_trust_score(&self, domain: &str) -> Result<f64, mongodb::error::Error> {
        use mongodb::bson::doc;
        use chrono::Utc;
        
        // Get domain verification status
        let domains_col = self.db.collection::<mongodb::bson::Document>("domains");
        let domain_filter = doc! { "_id": domain };
        let domain_doc = domains_col.find_one(domain_filter, None).await?;
        
        let mut trust_score = 0.5; // Base score
        
        if let Some(domain_data) = domain_doc {
            // Factor 1: Verification status (+0.3 if verified)
            if domain_data.get_bool("verified").unwrap_or(false) {
                trust_score += 0.3;
            }
            
            // Factor 2: Domain age (older = more trusted, max +0.2)
            if let Ok(created_at) = domain_data.get_datetime("created_at") {
                let age_seconds = Utc::now().timestamp() - (created_at.timestamp_millis() / 1000);
                let age_days = age_seconds / 86400;
                let age_bonus = (age_days as f64 / 365.0).min(0.2_f64); // Max 0.2 for 1+ year
                trust_score += age_bonus;
            }
        }
        
        // Factor 3: Visit patterns (from analytics)
        let analytics_col = self.db.collection::<mongodb::bson::Document>("analytics");
        let analytics_filter = doc! { "_id": domain };
        if let Ok(Some(analytics)) = analytics_col.find_one(analytics_filter, None).await {
            // More unique visitors = more trusted (max +0.1)
            if let Ok(visitors) = analytics.get_i64("unique_visitors") {
                let visitor_bonus = (visitors as f64 / 1000.0).min(0.1_f64);
                trust_score += visitor_bonus;
            }
            
            // Lower bounce rate = more trusted (max +0.1)
            if let Ok(bounce_rate) = analytics.get_f64("bounce_rate") {
                let bounce_bonus = (1.0_f64 - bounce_rate).min(0.1_f64);
                trust_score += bounce_bonus;
            }
        }
        
        // Clamp between 0.0 and 1.0
        Ok(trust_score.min(1.0).max(0.0))
    }
}

