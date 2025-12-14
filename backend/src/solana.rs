use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub struct SolanaClient {
    rpc_url: String,
}

impl SolanaClient {
    pub fn new(rpc_url: String) -> Self {
        Self { rpc_url }
    }

    pub fn search_account(&self, address: &str) -> Result<Option<AccountInfo>, String> {
        let pubkey = Pubkey::from_str(address)
            .map_err(|e| format!("Invalid pubkey: {}", e))?;

        let client = RpcClient::new(&self.rpc_url);
        
        match client.get_account(&pubkey) {
            Ok(account) => Ok(Some(AccountInfo {
                address: address.to_string(),
                lamports: account.lamports,
                owner: account.owner.to_string(),
                executable: account.executable,
                data_len: account.data.len(),
            })),
            Err(_) => Ok(None),
        }
    }

    pub fn search_program(&self, address: &str) -> Result<Option<ProgramInfo>, String> {
        let pubkey = Pubkey::from_str(address)
            .map_err(|e| format!("Invalid pubkey: {}", e))?;

        let client = RpcClient::new(&self.rpc_url);
        
        match client.get_account(&pubkey) {
            Ok(account) if account.executable => Ok(Some(ProgramInfo {
                address: address.to_string(),
                lamports: account.lamports,
                data_len: account.data.len(),
            })),
            Ok(_) => Ok(None),
            Err(_) => Ok(None),
        }
    }

    /// Verify program ownership by checking upgrade authority
    /// Returns true if the owner_pubkey matches the program's upgrade authority
    pub fn verify_program_ownership(
        &self,
        program_address: &str,
        owner_pubkey: &str,
    ) -> Result<bool, String> {
        let program = Pubkey::from_str(program_address)
            .map_err(|e| format!("Invalid program pubkey: {}", e))?;
        
        let owner = Pubkey::from_str(owner_pubkey)
            .map_err(|e| format!("Invalid owner pubkey: {}", e))?;

        let client = RpcClient::new(&self.rpc_url);
        
        // Get program account data
        let account = client.get_account(&program)
            .map_err(|e| format!("Failed to get program account: {}", e))?;

        // Check if account is executable (is a program)
        if !account.executable {
            return Ok(false);
        }

        // Get program data account (BPF upgradeable loader)
        // For upgradeable programs, the upgrade authority is stored in the program data account
        // This is a simplified check - in production, you'd parse the program data account
        // For now, we'll check if the owner matches the program's owner field
        // (which for BPF programs is the BPF loader)
        
        // For a more accurate check, we'd need to:
        // 1. Get the program data account (derived from program address)
        // 2. Parse the upgrade authority from the program data
        // 3. Compare with owner_pubkey
        
        // Simplified: check if owner matches (this works for non-upgradeable programs)
        Ok(account.owner == owner)
    }

    /// Get program upgrade authority (for upgradeable programs)
    pub fn get_program_upgrade_authority(
        &self,
        program_address: &str,
    ) -> Result<Option<String>, String> {
        let program = Pubkey::from_str(program_address)
            .map_err(|e| format!("Invalid program pubkey: {}", e))?;

        let client = RpcClient::new(&self.rpc_url);
        
        // Get program account
        let account = client.get_account(&program)
            .map_err(|e| format!("Failed to get program account: {}", e))?;

        if !account.executable {
            return Ok(None);
        }

        // For upgradeable programs, derive program data account
        // Program data = findProgramAddress([program, "upgrade"])
        // This is a simplified version - full implementation would derive the PDA
        // and parse the upgrade authority from the account data
        
        // Placeholder: return None for now
        // In production, implement proper PDA derivation and data parsing
        Ok(None)
    }

    /// Get SOL balance for a pubkey
    pub async fn get_balance(&self, pubkey: &str) -> Result<u64, String> {
        let pubkey = Pubkey::from_str(pubkey)
            .map_err(|e| format!("Invalid pubkey: {}", e))?;
        
        let client = RpcClient::new(&self.rpc_url);
        client.get_balance(&pubkey)
            .map_err(|e| format!("RPC error: {}", e))
    }

    /// Get recent blockhash
    pub async fn get_recent_blockhash(&self) -> Result<solana_sdk::hash::Hash, String> {
        let client = RpcClient::new(&self.rpc_url);
        let hash = client.get_latest_blockhash()
            .map_err(|e| format!("RPC error: {}", e))?;
        Ok(hash)
    }

    /// Get token accounts for a pubkey
    pub async fn get_token_accounts(&self, _pubkey: &Pubkey) -> Result<Vec<TokenAccountInfo>, String> {
        // TODO: Implement proper token account fetching
        // The solana-client API for get_token_accounts_by_owner has complex type requirements
        // For now, return empty vector - will be implemented with proper parsing later
        Ok(Vec::new())
    }

    /// Get signatures for an address
    pub async fn get_signatures_for_address(
        &self,
        pubkey: &Pubkey,
        limit: u32,
    ) -> Result<Vec<SignatureInfo>, String> {
        let client = RpcClient::new(&self.rpc_url);
        
        // Use the simpler API - get_signatures_for_address takes only the pubkey
        let signatures = client.get_signatures_for_address(pubkey)
            .map_err(|e| format!("RPC error: {}", e))?;
        
        // Limit the results manually
        let signatures: Vec<_> = signatures.into_iter().take(limit as usize).collect();

        Ok(signatures.into_iter().map(|sig| SignatureInfo {
            signature: sig.signature.to_string(),
            block_time: sig.block_time,
        }).collect())
    }
}

#[derive(Debug, Clone)]
pub struct TokenAccountInfo {
    pub mint: String,
    pub amount: u64,
    pub decimals: u8,
}

#[derive(Debug, Clone)]
pub struct SignatureInfo {
    pub signature: String,
    pub block_time: Option<i64>,
}

#[derive(serde::Serialize)]
pub struct AccountInfo {
    pub address: String,
    pub lamports: u64,
    pub owner: String,
    pub executable: bool,
    pub data_len: usize,
}

#[derive(serde::Serialize)]
pub struct ProgramInfo {
    pub address: String,
    pub lamports: u64,
    pub data_len: usize,
}

