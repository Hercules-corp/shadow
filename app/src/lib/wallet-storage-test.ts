/**
 * Test utility to verify wallet storage
 * Run this in browser console to check what's stored
 */

export function inspectWalletStorage() {
  const storageKey = 'shadow_wallet_encrypted'
  const addressKey = 'shadow_wallet_address'
  
  const encrypted = localStorage.getItem(storageKey)
  const address = localStorage.getItem(addressKey)
  
  console.log('=== Wallet Storage Inspection ===')
  console.log('Storage Location:', 'localStorage')
  console.log('Private Key Key:', storageKey)
  console.log('Address Key:', addressKey)
  console.log('')
  console.log('Private Key (base58):', encrypted ? `${encrypted.substring(0, 20)}...` : 'NOT FOUND')
  console.log('Address:', address || 'NOT FOUND')
  console.log('')
  console.log('Full Private Key Length:', encrypted?.length || 0, 'characters')
  console.log('Storage Method:', 'localStorage (browser)')
  console.log('Encoding:', 'base58 (Solana standard)')
  console.log('Encryption:', 'None (encoded only)')
  console.log('')
  console.log('⚠️  Security Note:')
  console.log('   - Private key is base58 encoded (not encrypted)')
  console.log('   - Stored in localStorage (accessible to JavaScript)')
  console.log('   - Vulnerable to XSS attacks')
  console.log('   - For production, consider Web Crypto API encryption')
  
  return {
    hasWallet: !!encrypted,
    address,
    privateKeyLength: encrypted?.length || 0,
    storageLocation: 'localStorage',
  }
}

// Make it available globally for easy testing
if (typeof window !== 'undefined') {
  (window as any).inspectWalletStorage = inspectWalletStorage
}

