import { Keypair } from '@solana/web3.js'
import bs58 from 'bs58'
import {
  encrypt,
  decrypt,
  uint8ArrayToBase64,
  base64ToUint8Array,
  arrayBufferToBase64,
  base64ToArrayBuffer,
} from './crypto'

const WALLET_STORAGE_KEY = 'shadow_wallet_encrypted'
const WALLET_ADDRESS_KEY = 'shadow_wallet_address'
const WALLET_SALT_KEY = 'shadow_wallet_salt'
const WALLET_IV_KEY = 'shadow_wallet_iv'

/**
 * Generate a new Solana keypair
 */
export function generateWallet(): Keypair {
  return Keypair.generate()
}

/**
 * Encrypt wallet private key with password
 */
async function encryptKeypair(keypair: Keypair, password: string): Promise<{
  encrypted: string
  salt: string
  iv: string
}> {
  const secretKey = keypair.secretKey
  
  // Encrypt using Web Crypto API
  const { encrypted, salt, iv } = await encrypt(secretKey, password)
  
  // Convert to base64 for storage
  return {
    encrypted: arrayBufferToBase64(encrypted),
    salt: uint8ArrayToBase64(salt),
    iv: uint8ArrayToBase64(iv),
  }
}

/**
 * Decrypt wallet private key with password
 */
async function decryptKeypair(
  encrypted: string,
  password: string,
  salt: string,
  iv: string
): Promise<Keypair> {
  try {
    // Convert from base64
    const encryptedBuffer = base64ToArrayBuffer(encrypted)
    const saltArray = base64ToUint8Array(salt)
    const ivArray = base64ToUint8Array(iv)
    
    // Decrypt using Web Crypto API
    const decrypted = await decrypt(encryptedBuffer, password, saltArray, ivArray)
    
    // Create keypair from decrypted secret key
    return Keypair.fromSecretKey(decrypted)
  } catch (error) {
    throw new Error('Failed to decrypt wallet. Wrong password?')
  }
}

/**
 * Store wallet securely in localStorage with encryption
 * 
 * SECURITY:
 * - Uses AES-GCM encryption (256-bit)
 * - PBKDF2 key derivation (100,000 iterations)
 * - Random salt and IV for each encryption
 * - Password required to decrypt
 */
export async function storeWallet(keypair: Keypair, password: string): Promise<void> {
  try {
    // Encrypt the wallet with password
    const { encrypted, salt, iv } = await encryptKeypair(keypair, password)
    const address = keypair.publicKey.toBase58()
    
    // Store encrypted data in localStorage
    localStorage.setItem(WALLET_STORAGE_KEY, encrypted)
    localStorage.setItem(WALLET_ADDRESS_KEY, address)
    localStorage.setItem(WALLET_SALT_KEY, salt)
    localStorage.setItem(WALLET_IV_KEY, iv)
    
    // Log for verification (remove in production)
    if (import.meta.env.DEV) {
      console.log('✅ Wallet encrypted and stored in localStorage')
      console.log('   Address:', address)
      console.log('   Encryption: AES-GCM (256-bit)')
      console.log('   Key Derivation: PBKDF2 (100k iterations)')
    }
  } catch (error) {
    console.error('Failed to store wallet:', error)
    throw error
  }
}

/**
 * Load wallet from localStorage with password decryption
 */
export async function loadWallet(password: string): Promise<Keypair | null> {
  try {
    const encrypted = localStorage.getItem(WALLET_STORAGE_KEY)
    const salt = localStorage.getItem(WALLET_SALT_KEY)
    const iv = localStorage.getItem(WALLET_IV_KEY)
    
    if (!encrypted || !salt || !iv) {
      return null
    }
    
    // Decrypt wallet with password
    return await decryptKeypair(encrypted, password, salt, iv)
  } catch (error) {
    console.error('Failed to load wallet:', error)
    throw error // Re-throw to allow caller to handle (e.g., wrong password)
  }
}

/**
 * Get wallet address from storage (without loading full keypair)
 */
export function getStoredWalletAddress(): string | null {
  return localStorage.getItem(WALLET_ADDRESS_KEY)
}

/**
 * Check if wallet exists in storage
 */
export function hasStoredWallet(): boolean {
  return localStorage.getItem(WALLET_STORAGE_KEY) !== null
}

/**
 * Delete wallet from storage
 */
export function deleteWallet(): void {
  localStorage.removeItem(WALLET_STORAGE_KEY)
  localStorage.removeItem(WALLET_ADDRESS_KEY)
  localStorage.removeItem(WALLET_SALT_KEY)
  localStorage.removeItem(WALLET_IV_KEY)
}

/**
 * Export wallet as base58 secret key (for backup)
 * NOTE: This exports the UNENCRYPTED key - use with caution
 */
export function exportWallet(keypair: Keypair): string {
  return bs58.encode(keypair.secretKey)
}

/**
 * Import wallet from base58 secret key
 */
export function importWallet(secretKeyBase58: string): Keypair {
  try {
    const secretKey = bs58.decode(secretKeyBase58)
    return Keypair.fromSecretKey(secretKey)
  } catch (error) {
    throw new Error('Invalid wallet secret key')
  }
}

/**
 * Verify password is correct for stored wallet
 */
export async function verifyPassword(password: string): Promise<boolean> {
  try {
    await loadWallet(password)
    return true
  } catch (error) {
    return false
  }
}
