// Athena - Local Solana Wallet Provider
// Device-local wallet generation with password-based encryption

import { Keypair, PublicKey } from "@solana/web3.js"
import bs58 from "bs58"
import {
  encrypt,
  decrypt,
  uint8ArrayToBase64,
  base64ToUint8Array,
  arrayBufferToBase64,
  base64ToArrayBuffer,
} from "./crypto"

const WALLET_STORAGE_KEY = "athena_wallet_encrypted"
const WALLET_ADDRESS_KEY = "athena_wallet_address"
const WALLET_SALT_KEY = "athena_wallet_salt"
const WALLET_IV_KEY = "athena_wallet_iv"

/**
 * Generate a new Solana keypair
 */
export function generateWallet(): Keypair {
  return Keypair.generate()
}

/**
 * Encrypt wallet private key with password (AES-256-GCM)
 */
async function encryptKeypair(
  keypair: Keypair,
  password: string
): Promise<{
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
  const encryptedBuffer = base64ToArrayBuffer(encrypted)
  const saltArray = base64ToUint8Array(salt)
  const ivArray = base64ToUint8Array(iv)

  const decrypted = await decrypt(encryptedBuffer, password, saltArray, ivArray)

  return Keypair.fromSecretKey(decrypted)
}

/**
 * Save encrypted wallet to localStorage
 */
export async function saveWallet(
  keypair: Keypair,
  password: string
): Promise<void> {
  const { encrypted, salt, iv } = await encryptKeypair(keypair, password)
  const address = keypair.publicKey.toBase58()

  if (typeof window === "undefined") {
    throw new Error("localStorage is not available")
  }

  localStorage.setItem(WALLET_STORAGE_KEY, encrypted)
  localStorage.setItem(WALLET_ADDRESS_KEY, address)
  localStorage.setItem(WALLET_SALT_KEY, salt)
  localStorage.setItem(WALLET_IV_KEY, iv)
}

/**
 * Load and decrypt wallet from localStorage
 */
export async function loadWallet(password: string): Promise<Keypair | null> {
  if (typeof window === "undefined") {
    return null
  }

  const encrypted = localStorage.getItem(WALLET_STORAGE_KEY)
  const salt = localStorage.getItem(WALLET_SALT_KEY)
  const iv = localStorage.getItem(WALLET_IV_KEY)

  if (!encrypted || !salt || !iv) {
    return null
  }

  try {
    return await decryptKeypair(encrypted, password, salt, iv)
  } catch (error) {
    console.error("Failed to decrypt wallet:", error)
    return null
  }
}

/**
 * Get wallet address from localStorage (without password)
 */
export function getWalletAddress(): string | null {
  if (typeof window === "undefined") {
    return null
  }

  return localStorage.getItem(WALLET_ADDRESS_KEY)
}

/**
 * Check if wallet exists in localStorage
 */
export function hasWallet(): boolean {
  if (typeof window === "undefined") {
    return false
  }

  return localStorage.getItem(WALLET_STORAGE_KEY) !== null
}

/**
 * Delete wallet from localStorage
 */
export function deleteWallet(): void {
  if (typeof window === "undefined") {
    return
  }

  localStorage.removeItem(WALLET_STORAGE_KEY)
  localStorage.removeItem(WALLET_ADDRESS_KEY)
  localStorage.removeItem(WALLET_SALT_KEY)
  localStorage.removeItem(WALLET_IV_KEY)
}

/**
 * Export wallet as base58-encoded secret key (for backup)
 * Requires password to decrypt first
 */
export async function exportWallet(password: string): Promise<string | null> {
  const keypair = await loadWallet(password)
  if (!keypair) {
    return null
  }

  return bs58.encode(keypair.secretKey)
}

/**
 * Import wallet from base58-encoded secret key
 */
export async function importWallet(
  secretKeyBase58: string,
  password: string
): Promise<Keypair> {
  try {
    const secretKey = bs58.decode(secretKeyBase58)
    const keypair = Keypair.fromSecretKey(secretKey)

    // Save encrypted
    await saveWallet(keypair, password)

    return keypair
  } catch (error) {
    throw new Error(`Invalid secret key: ${error}`)
  }
}


