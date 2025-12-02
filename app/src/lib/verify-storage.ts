/**
 * Verification utility to prove wallet is stored in localStorage
 * 
 * Run this in browser console:
 *   import { verifyWalletStorage } from './lib/verify-storage'
 *   verifyWalletStorage()
 * 
 * Or access directly:
 *   window.verifyWalletStorage()
 */

const WALLET_STORAGE_KEY = 'shadow_wallet_encrypted'
const WALLET_ADDRESS_KEY = 'shadow_wallet_address'

export function verifyWalletStorage() {
  console.log('='.repeat(60))
  console.log('🔍 WALLET STORAGE VERIFICATION')
  console.log('='.repeat(60))
  console.log('')
  
  // Check localStorage
  const privateKey = localStorage.getItem(WALLET_STORAGE_KEY)
  const address = localStorage.getItem(WALLET_ADDRESS_KEY)
  
  console.log('📍 Storage Location: localStorage (browser)')
  console.log('')
  console.log('📦 Storage Keys:')
  console.log(`   ${WALLET_STORAGE_KEY}:`, privateKey ? '✅ EXISTS' : '❌ NOT FOUND')
  console.log(`   ${WALLET_ADDRESS_KEY}:`, address ? '✅ EXISTS' : '❌ NOT FOUND')
  console.log('')
  
  if (privateKey) {
    console.log('🔑 Private Key Details:')
    console.log('   Format: base58 (Solana standard)')
    console.log('   Length:', privateKey.length, 'characters')
    console.log('   Preview:', privateKey.substring(0, 30) + '...')
    console.log('   Full Key:', privateKey)
    console.log('')
  }
  
  if (address) {
    console.log('📍 Public Address:')
    console.log('   Address:', address)
    console.log('')
  }
  
  console.log('🔒 Security Level:')
  console.log('   Encoding: base58 (readable)')
  console.log('   Encryption: ❌ NONE (not encrypted)')
  console.log('   Storage: localStorage (accessible to JavaScript)')
  console.log('   XSS Risk: ⚠️  HIGH (any script can read it)')
  console.log('')
  
  console.log('📋 How to Verify Manually:')
  console.log('   1. Open DevTools (F12)')
  console.log('   2. Go to Application tab > Local Storage')
  console.log('   3. Find key: "shadow_wallet_encrypted"')
  console.log('   4. The value is your base58-encoded private key')
  console.log('')
  
  console.log('⚠️  IMPORTANT:')
  console.log('   - This is ENCODED, not ENCRYPTED')
  console.log('   - Anyone with access to localStorage can read it')
  console.log('   - XSS attacks can steal this key')
  console.log('   - For production, use Web Crypto API encryption')
  console.log('')
  
  return {
    hasWallet: !!privateKey,
    address,
    privateKeyLength: privateKey?.length || 0,
    storageLocation: 'localStorage',
    securityLevel: 'encoded (not encrypted)',
    xssRisk: 'high',
  }
}

// Make available globally for easy testing
if (typeof window !== 'undefined') {
  (window as any).verifyWalletStorage = verifyWalletStorage
  console.log('💡 Tip: Run verifyWalletStorage() in console to inspect storage')
}

