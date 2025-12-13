// Zeus - God of Thunder and Power (On-Chain Operations Circuit)
// Handle Solana on-chain operations and program interactions

import { Connection, PublicKey, Keypair, Transaction } from "@solana/web3.js"
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor"

const DEFAULT_RPC_URL = "https://api.devnet.solana.com"

/**
 * Verify a program address exists on-chain
 */
export async function verifyProgram(
  programAddress: string,
  rpcUrl: string = DEFAULT_RPC_URL
): Promise<boolean> {
  try {
    const connection = new Connection(rpcUrl, "confirmed")
    const pubkey = new PublicKey(programAddress)
    const accountInfo = await connection.getAccountInfo(pubkey)
    return accountInfo !== null && accountInfo.executable
  } catch {
    return false
  }
}

/**
 * Get program account data
 */
export async function getProgramData(
  programAddress: string,
  rpcUrl: string = DEFAULT_RPC_URL
): Promise<{ lamports: number; owner: string; dataLength: number } | null> {
  try {
    const connection = new Connection(rpcUrl, "confirmed")
    const pubkey = new PublicKey(programAddress)
    const accountInfo = await connection.getAccountInfo(pubkey)
    
    if (!accountInfo) return null
    
    return {
      lamports: accountInfo.lamports,
      owner: accountInfo.owner.toBase58(),
      dataLength: accountInfo.data.length,
    }
  } catch {
    return null
  }
}

/**
 * Verify wallet owns a program by checking upgrade authority
 */
export async function verifyOwnership(
  programAddress: string,
  ownerPubkey: string,
  rpcUrl: string = DEFAULT_RPC_URL
): Promise<boolean> {
  try {
    const connection = new Connection(rpcUrl, "confirmed")
    const programPubkey = new PublicKey(programAddress)
    const ownerPubkeyObj = new PublicKey(ownerPubkey)
    
    // Get program account info
    const accountInfo = await connection.getAccountInfo(programPubkey)
    if (!accountInfo || !accountInfo.executable) {
      return false
    }
    
    // For Solana programs, check the upgrade authority
    // The program's upgrade authority is stored in the program data account
    // We need to derive the program data address
    const [programDataAddress] = PublicKey.findProgramAddressSync(
      [programPubkey.toBuffer()],
      new PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
    )
    
    try {
      const programDataInfo = await connection.getAccountInfo(programDataAddress)
      if (!programDataInfo) {
        // If no program data account, check if owner is the program itself (immutable)
        return accountInfo.owner.equals(programPubkey)
      }
      
      // Parse upgrade authority from program data (offset 12, 32 bytes)
      if (programDataInfo.data.length < 44) {
        return false
      }
      
      const upgradeAuthorityBytes = programDataInfo.data.slice(12, 44)
      const upgradeAuthority = new PublicKey(upgradeAuthorityBytes)
      
      // Check if the provided owner is the upgrade authority
      return upgradeAuthority.equals(ownerPubkeyObj)
    } catch {
      // Fallback: check if owner matches program owner
      return accountInfo.owner.equals(ownerPubkeyObj)
    }
  } catch {
    return false
  }
}

/**
 * Create a transaction for on-chain operations
 */
export function createTransaction(
  instructions: any[],
  payer: PublicKey
): Transaction {
  const transaction = new Transaction()
  transaction.add(...instructions)
  transaction.feePayer = payer
  return transaction
}


