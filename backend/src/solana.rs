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

