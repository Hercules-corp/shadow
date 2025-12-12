// Integration tests for Shadow backend
#[cfg(test)]
mod tests {
    use shadow_backend::apollo::ApolloValidator;
    use shadow_backend::artemis::ArtemisRateLimiter;
    use shadow_backend::hephaestus::HephaestusCache;
    use std::time::Duration;
    
    #[test]
    fn test_apollo_validation() {
        // Test pubkey validation
        assert!(ApolloValidator::validate_pubkey("11111111111111111111111111111111").is_ok());
        assert!(ApolloValidator::validate_pubkey("invalid").is_err());
        
        // Test domain validation
        assert!(ApolloValidator::validate_domain("example.shadow").is_ok());
        assert!(ApolloValidator::validate_domain("invalid..domain").is_err());
        
        // Test IPFS CID validation
        assert!(ApolloValidator::validate_ipfs_cid("QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG").is_ok());
        assert!(ApolloValidator::validate_ipfs_cid("invalid").is_err());
        
        // Test search query validation
        assert!(ApolloValidator::validate_search_query("test query").is_ok());
        assert!(ApolloValidator::validate_search_query("").is_err());
    }
    
    #[test]
    fn test_artemis_rate_limiter() {
        let limiter = ArtemisRateLimiter::new(10);
        
        let key = "test_client";
        for _ in 0..10 {
            assert!(limiter.check_rate_limit(key).is_ok());
        }
        assert!(limiter.check_rate_limit(key).is_err());
        
        // Test different keys don't interfere
        assert!(limiter.check_rate_limit("other_client").is_ok());
    }
    
    #[tokio::test]
    async fn test_hephaestus_cache() {
        let cache = HephaestusCache::new(10, 3600); // 10MB, 1hr TTL
        
        // Test set and get
        cache.set(
            "test_key".to_string(),
            b"test content".to_vec(),
            "text/plain".to_string(),
            None,
        ).await.unwrap();
        
        let cached = cache.get("test_key").await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().content, b"test content");
        
        // Test cache stats
        let stats = cache.get_stats().await;
        assert_eq!(stats.total_entries, 1);
        
        // Test clear
        cache.clear().await;
        assert!(cache.get("test_key").await.is_none());
    }
    
    #[test]
    fn test_apollo_limit_validation() {
        // Test limit validation
        assert!(ApolloValidator::validate_limit(Some(10)).is_ok());
        assert!(ApolloValidator::validate_limit(Some(0)).is_err());
        assert!(ApolloValidator::validate_limit(Some(1000)).is_err()); // Too high
        assert!(ApolloValidator::validate_limit(None).is_ok());
    }
}

