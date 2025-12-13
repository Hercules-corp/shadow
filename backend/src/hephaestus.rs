// Hephaestus: Forge and cache - Content caching and optimization
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedContent {
    pub content: Vec<u8>,
    pub content_type: String,
    pub cached_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub etag: String,
    pub size_bytes: usize,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    content: CachedContent,
    last_accessed: Instant,
    access_count: u64,
}

pub struct HephaestusCache {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    max_size_mb: usize,
    default_ttl: Duration,
    hits: Arc<std::sync::atomic::AtomicU64>,
    misses: Arc<std::sync::atomic::AtomicU64>,
}

impl HephaestusCache {
    pub fn new(max_size_mb: usize, default_ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_size_mb,
            default_ttl: Duration::from_secs(default_ttl_seconds),
            hits: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            misses: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    pub async fn get(&self, key: &str) -> Option<CachedContent> {
        let mut cache = self.cache.write().await;
        
        if let Some(entry) = cache.get_mut(key) {
            // Check if expired
            if Utc::now() > entry.content.expires_at {
                cache.remove(key);
                self.misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                return None;
            }
            
            entry.last_accessed = Instant::now();
            entry.access_count += 1;
            self.hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return Some(entry.content.clone());
        }
        
        self.misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        None
    }

    pub async fn set(
        &self,
        key: String,
        content: Vec<u8>,
        content_type: String,
        ttl: Option<Duration>,
    ) -> Result<(), String> {
        let mut cache = self.cache.write().await;
        
        // Check cache size and evict if needed
        self.evict_if_needed(&mut cache, content.len()).await;
        
        let now = Utc::now();
        let ttl_duration = ttl.unwrap_or(self.default_ttl);
        let expires_at = now + chrono::Duration::from_std(ttl_duration)
            .map_err(|e| format!("Invalid TTL: {}", e))?;
        
        let etag = self.generate_etag(&content);
        
        let cached_content = CachedContent {
            content: content.clone(),
            content_type,
            cached_at: now,
            expires_at,
            etag,
            size_bytes: content.len(),
        };
        
        let entry = CacheEntry {
            content: cached_content,
            last_accessed: Instant::now(),
            access_count: 0,
        };
        
        cache.insert(key, entry);
        Ok(())
    }

    pub async fn invalidate(&self, key: &str) {
        let mut cache = self.cache.write().await;
        cache.remove(key);
    }

    pub async fn invalidate_pattern(&self, pattern: &str) {
        let mut cache = self.cache.write().await;
        cache.retain(|k, _| !k.contains(pattern));
    }

    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    pub async fn get_stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        let mut total_size = 0;
        let mut total_entries = 0;
        let mut total_accesses = 0;
        
        for entry in cache.values() {
            total_size += entry.content.size_bytes;
            total_entries += 1;
            total_accesses += entry.access_count;
        }
        
        let hits = self.hits.load(std::sync::atomic::Ordering::Relaxed);
        let misses = self.misses.load(std::sync::atomic::Ordering::Relaxed);
        let total_requests = hits + misses;
        let hit_rate = if total_requests > 0 {
            hits as f64 / total_requests as f64
        } else {
            0.0
        };
        
        CacheStats {
            total_entries,
            total_size_mb: total_size as f64 / 1_048_576.0,
            total_accesses,
            hit_rate,
        }
    }

    async fn evict_if_needed(&self, cache: &mut HashMap<String, CacheEntry>, new_size: usize) {
        let max_size_bytes = self.max_size_mb * 1_048_576;
        let current_size: usize = cache.values().map(|e| e.content.size_bytes).sum();
        
        if current_size + new_size > max_size_bytes {
            // Evict least recently used entries
            let mut entries: Vec<(String, Instant, u64)> = cache
                .iter()
                .map(|(k, v)| (k.clone(), v.last_accessed, v.access_count))
                .collect();
            
            entries.sort_by(|a, b| a.1.cmp(&b.1));
            
            let mut freed = 0;
            for (key, _, _) in entries {
                if let Some(entry) = cache.remove(&key) {
                    freed += entry.content.size_bytes;
                    if current_size - freed + new_size <= max_size_bytes {
                        break;
                    }
                }
            }
        }
    }

    fn generate_etag(&self, content: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("\"{:x}\"", hasher.finalize())
    }
}

#[derive(Debug, Serialize)]
pub struct CacheStats {
    pub total_entries: usize,
    pub total_size_mb: f64,
    pub total_accesses: u64,
    pub hit_rate: f64,
}

