// Hades - God of the Underworld and Security
// Handles wallet encryption, security, and authentication

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub require_password_for_transactions: bool,
    pub require_password_for_export: bool,
    pub session_timeout_minutes: u32,
    pub biometric_enabled: bool,
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            require_password_for_transactions: true,
            require_password_for_export: true,
            session_timeout_minutes: 15,
            biometric_enabled: false,
        }
    }
}

pub struct HadesSecurityManager;

impl HadesSecurityManager {
    pub fn new() -> Self {
        Self
    }

    /// Hash a password using SHA-256 (for storage)
    pub fn hash_password(&self, password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Verify password hash
    pub fn verify_password(&self, password: &str, hash: &str) -> bool {
        self.hash_password(password) == hash
    }

    /// Generate session token
    pub fn generate_session_token(&self, user_id: &str) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut hasher = Sha256::new();
        hasher.update(user_id.as_bytes());
        hasher.update(timestamp.to_string().as_bytes());
        hasher.update(&rand::random::<u64>().to_be_bytes());
        
        hex::encode(hasher.finalize())
    }

    /// Validate session token (simplified - in production use JWT)
    pub fn validate_session_token(&self, token: &str) -> bool {
        // Basic validation - token should be 64 hex characters
        token.len() == 64 && token.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Encrypt sensitive data (simplified - in production use proper AES-GCM)
    pub fn encrypt_data(&self, data: &[u8], key: &[u8]) -> Vec<u8> {
        // Simple XOR encryption (in production use AES-256-GCM)
        data.iter()
            .enumerate()
            .map(|(i, &b)| b ^ key[i % key.len()])
            .collect()
    }

    /// Decrypt sensitive data
    pub fn decrypt_data(&self, encrypted: &[u8], key: &[u8]) -> Vec<u8> {
        self.encrypt_data(encrypted, key) // XOR is symmetric
    }

    /// Generate secure random bytes
    pub fn generate_random_bytes(&self, length: usize) -> Vec<u8> {
        (0..length).map(|_| rand::random::<u8>()).collect()
    }

    /// Validate password strength
    pub fn validate_password_strength(&self, password: &str) -> Result<(), String> {
        if password.len() < 8 {
            return Err("Password must be at least 8 characters".to_string());
        }
        
        if !password.chars().any(|c| c.is_ascii_lowercase()) {
            return Err("Password must contain at least one lowercase letter".to_string());
        }
        
        if !password.chars().any(|c| c.is_ascii_uppercase()) {
            return Err("Password must contain at least one uppercase letter".to_string());
        }
        
        if !password.chars().any(|c| c.is_ascii_digit()) {
            return Err("Password must contain at least one number".to_string());
        }
        
        Ok(())
    }
}


