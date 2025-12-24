// Sign-In With Solana (SIWS) implementation
// For local wallet authentication without Phantom/extensions

import { Keypair, PublicKey } from "@solana/web3.js";
import bs58 from "bs58";

export interface SIWSMessage {
  domain: string;
  address: string;
  statement?: string;
  uri: string;
  version: string;
  chainId: number;
  nonce: string;
  issuedAt: string;
  expirationTime?: string;
  notBefore?: string;
  requestId?: string;
  resources?: string[];
}

/**
 * Create a SIWS message for signing
 */
export function createSIWSMessage(params: {
  domain: string;
  address: string;
  statement?: string;
  uri?: string;
  nonce?: string;
  expirationTime?: string;
}): SIWSMessage {
  const now = new Date();
  
  return {
    domain: params.domain,
    address: params.address,
    statement: params.statement || "Sign in with Solana to the app.",
    uri: params.uri || window.location.origin,
    version: "1",
    chainId: 103, // Solana Mainnet (use 103 for devnet)
    nonce: params.nonce || generateNonce(),
    issuedAt: now.toISOString(),
    expirationTime: params.expirationTime || new Date(now.getTime() + 1000 * 60 * 60).toISOString(), // 1 hour
  };
}

/**
 * Format SIWS message as string for signing
 */
export function formatSIWSMessage(message: SIWSMessage): string {
  let msg = `${message.domain} wants you to sign in with your Solana account:\n${message.address}\n\n`;
  
  if (message.statement) {
    msg += `${message.statement}\n\n`;
  }
  
  msg += `URI: ${message.uri}\n`;
  msg += `Version: ${message.version}\n`;
  msg += `Chain ID: ${message.chainId}\n`;
  msg += `Nonce: ${message.nonce}\n`;
  msg += `Issued At: ${message.issuedAt}`;
  
  if (message.expirationTime) {
    msg += `\nExpiration Time: ${message.expirationTime}`;
  }
  
  if (message.notBefore) {
    msg += `\nNot Before: ${message.notBefore}`;
  }
  
  if (message.requestId) {
    msg += `\nRequest ID: ${message.requestId}`;
  }
  
  if (message.resources && message.resources.length > 0) {
    msg += `\nResources:\n`;
    message.resources.forEach((resource) => {
      msg += `- ${resource}\n`;
    });
  }
  
  return msg;
}

/**
 * Sign SIWS message with local wallet keypair
 */
export async function signSIWSMessage(
  keypair: Keypair,
  message: SIWSMessage
): Promise<{ message: string; signature: string }> {
  const formattedMessage = formatSIWSMessage(message);
  const messageBytes = new TextEncoder().encode(formattedMessage);
  
  // Sign with keypair using nacl
  // Solana uses ed25519 signatures
  const nacl = await import("tweetnacl");
  const signature = nacl.sign.detached(messageBytes, keypair.secretKey);
  const signatureBase58 = bs58.encode(signature);
  
  return {
    message: formattedMessage,
    signature: signatureBase58,
  };
}

/**
 * Verify SIWS signature
 */
export async function verifySIWSMessage(
  message: SIWSMessage,
  signature: string,
  publicKey: string
): Promise<boolean> {
  const formattedMessage = formatSIWSMessage(message);
  const messageBytes = new TextEncoder().encode(formattedMessage);
  
  try {
    const pubkey = new PublicKey(publicKey);
    const sigBytes = bs58.decode(signature);
    
    // Verify signature using nacl
    // Note: In production, this should be done on the backend
    const nacl = await import("tweetnacl");
    return nacl.sign.detached.verify(messageBytes, sigBytes, pubkey.toBytes());
  } catch {
    return false;
  }
}

/**
 * Generate random nonce
 */
function generateNonce(): string {
  const array = new Uint8Array(16);
  crypto.getRandomValues(array);
  return Array.from(array, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/**
 * Create authentication header for Shadow API
 */
export async function createAuthHeader(
  keypair: Keypair,
  challenge: string
): Promise<string> {
  const timestamp = Date.now();
  const message = `${challenge}:${timestamp}`;
  const messageBytes = new TextEncoder().encode(message);
  
  const nacl = await import("tweetnacl");
  const signature = nacl.sign.detached(messageBytes, keypair.secretKey);
  const signatureBase58 = bs58.encode(signature);
  
  return JSON.stringify({
    wallet: keypair.publicKey.toBase58(),
    signature: signatureBase58,
    timestamp,
  });
}

