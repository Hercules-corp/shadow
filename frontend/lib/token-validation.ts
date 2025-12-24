// Token validation utilities for Shadow browser
// Only accepts SPL token addresses as domains

import { PublicKey } from "@solana/web3.js";

/**
 * Validates if a string is a valid Solana Pubkey (token address)
 */
export function isValidTokenAddress(address: string): boolean {
  try {
    new PublicKey(address);
    return true;
  } catch {
    return false;
  }
}

/**
 * Validates token address and returns error message if invalid
 */
export function validateTokenAddress(address: string): { valid: boolean; error?: string } {
  const trimmed = address.trim();
  
  if (!trimmed) {
    return { valid: false, error: "Enter a token address" };
  }

  // Check if it's a normal URL/domain (reject these)
  if (trimmed.includes("://") || trimmed.includes("www.") || trimmed.includes("http")) {
    return { valid: false, error: "Invalid: Use token address only (not URLs/domains)" };
  }

  // Check if it contains a dot (likely a domain)
  if (trimmed.includes(".") && !trimmed.match(/^[1-9A-HJ-NP-Za-km-z]{32,44}$/)) {
    return { valid: false, error: "Invalid: Use token address only (not domains)" };
  }

  // Validate as Solana Pubkey
  if (!isValidTokenAddress(trimmed)) {
    return { valid: false, error: "Invalid token address format" };
  }

  return { valid: true };
}

/**
 * Parse token address with optional sublink
 * Returns { tokenAddress, sublink } or null if invalid
 */
export function parseTokenAddress(input: string): { tokenAddress: string; sublink?: string } | null {
  const trimmed = input.trim();
  
  // Check for sublink (token_addr/sublink)
  const parts = trimmed.split("/");
  const tokenAddress = parts[0];
  const sublink = parts.slice(1).join("/") || undefined;

  const validation = validateTokenAddress(tokenAddress);
  if (!validation.valid) {
    return null;
  }

  return {
    tokenAddress,
    sublink: sublink || undefined,
  };
}

/**
 * Normalize token address input
 */
export function normalizeTokenInput(input: string): string {
  return input.trim().toLowerCase();
}

