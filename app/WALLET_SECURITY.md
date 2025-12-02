# Wallet Security

## Current Implementation

The Shadow app now uses **local device wallets** instead of Google authentication. Each user gets a Solana wallet that is:

- ✅ **Generated on-device** - No server involvement
- ✅ **Stored locally** - In browser localStorage
- ✅ **Auto-created** - On first visit, no signup needed
- ✅ **Private** - Never sent to any server

## How It Works

1. **First Visit**: A new Solana keypair is automatically generated
2. **Storage**: The private key is encoded (base58) and stored in localStorage
3. **Subsequent Visits**: The wallet is automatically loaded from storage
4. **No Accounts**: No email, no username, no tracking

## Security Considerations

### Current Storage Method
- Uses `localStorage` with base58 encoding
- **Pros**: Simple, works everywhere, no dependencies
- **Cons**: Not encrypted (but encoded), vulnerable to XSS attacks

### For Production (Recommended Improvements)

1. **Use Web Crypto API** for encryption:
   ```typescript
   // Encrypt with user's device key
   const encrypted = await crypto.subtle.encrypt(...)
   ```

2. **Use IndexedDB** instead of localStorage:
   - Better for larger data
   - More secure storage option

3. **Add password protection**:
   - Encrypt wallet with user-chosen password
   - Use PBKDF2 or similar for key derivation

4. **Consider hardware wallets**:
   - Support Ledger/Trezor for advanced users
   - Keep private keys in hardware

## Wallet Management

Users can:
- **Create new wallet**: Generates a fresh keypair
- **Delete wallet**: Removes wallet from device (destructive)
- **Export wallet**: Get secret key for backup (future feature)
- **Import wallet**: Restore from backup (future feature)

## Privacy Benefits

- ✅ No email collection
- ✅ No Google OAuth
- ✅ No third-party authentication
- ✅ No user accounts
- ✅ Wallet is device-specific
- ✅ Can't be tracked across devices

## Backup Recommendation

**Important**: Users should export and backup their wallet secret key. If they clear browser data or switch devices, the wallet will be lost.

Future feature: Add export/import functionality with clear warnings.

