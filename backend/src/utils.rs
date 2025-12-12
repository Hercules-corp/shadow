// Utility functions for Shadow backend
use std::time::{SystemTime, UNIX_EPOCH};

/// Get current timestamp in milliseconds
pub fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

/// Format duration as human-readable string
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    }
}

/// Calculate percentage change
pub fn percentage_change(old: f64, new: f64) -> f64 {
    if old == 0.0 {
        if new == 0.0 {
            0.0
        } else {
            100.0
        }
    } else {
        ((new - old) / old) * 100.0
    }
}

/// Truncate string to max length with ellipsis
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Parse Solana address from string
pub fn parse_solana_address(addr: &str) -> Result<[u8; 32], String> {
    use bs58;
    let decoded = bs58::decode(addr)
        .into_vec()
        .map_err(|e| format!("Invalid base58: {}", e))?;
    
    if decoded.len() != 32 {
        return Err("Address must be 32 bytes".to_string());
    }
    
    let mut result = [0u8; 32];
    result.copy_from_slice(&decoded);
    Ok(result)
}

/// Validate and normalize domain
pub fn normalize_domain(domain: &str) -> Result<String, String> {
    let normalized = domain.to_lowercase().trim().to_string();
    
    if normalized.is_empty() {
        return Err("Domain cannot be empty".to_string());
    }
    
    if normalized.len() > 253 {
        return Err("Domain too long".to_string());
    }
    
    // Basic validation
    if normalized.contains("..") {
        return Err("Domain cannot contain consecutive dots".to_string());
    }
    
    if normalized.starts_with('.') || normalized.ends_with('.') {
        return Err("Domain cannot start or end with dot".to_string());
    }
    
    Ok(normalized)
}

/// Generate a unique ID
pub fn generate_id() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
}

/// Calculate hash of content
pub fn hash_content(content: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}

/// Format bytes as human-readable size
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Check if string is valid base58
pub fn is_base58(s: &str) -> bool {
    s.chars().all(|c| {
        matches!(c, '1'..='9' | 'A'..='H' | 'J'..='N' | 'P'..='Z' | 'a'..='k' | 'm'..='z')
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3665), "1h 1m");
    }
    
    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("short", 10), "short");
        assert_eq!(truncate_string("very long string", 10), "very lo...");
    }
    
    #[test]
    fn test_normalize_domain() {
        assert!(normalize_domain("example.shadow").is_ok());
        assert!(normalize_domain("EXAMPLE.SHADOW").is_ok());
        assert!(normalize_domain("invalid..domain").is_err());
        assert!(normalize_domain("").is_err());
    }
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(512), "512 B");
    }
    
    #[test]
    fn test_is_base58() {
        assert!(is_base58("11111111111111111111111111111111"));
        assert!(!is_base58("invalid-base58-0OIl"));
    }
}
