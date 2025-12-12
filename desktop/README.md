# Shadow Desktop (Vue)

Desktop application for Shadow Browser built with Vue 3 + TypeScript + Vite.

## Features

- **Hermes Wallet Service**: Secure Solana wallet generation and encryption
- **AES-GCM Encryption**: 256-bit encryption with PBKDF2 key derivation
- **Password-Protected Storage**: Wallets stored securely in localStorage
- **Vue 3 Composition API**: Modern reactive framework
- **Pinia State Management**: Centralized wallet state
- **Vue Router**: Client-side routing

## Setup

1. Install dependencies:
```bash
npm install
```

2. Run development server:
```bash
npm run dev
```

3. Build for production:
```bash
npm run build
```

## Architecture

- `src/lib/wallet.ts`: Wallet encryption/decryption (Hermes)
- `src/lib/crypto.ts`: Web Crypto API utilities
- `src/stores/wallet.ts`: Pinia store for wallet state
- `src/views/`: UI views (Home, Profile, Site)
- `src/router/`: Vue Router configuration

## Security

- Wallets encrypted with AES-GCM (256-bit)
- PBKDF2 key derivation (100,000 iterations)
- Random salt and IV for each encryption
- Secure storage via localStorage

