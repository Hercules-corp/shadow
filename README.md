# Shadow Browser

A decentralized browser built on Solana, where sites are Solana programs and domains are managed by the Olympus CA system.

## Architecture

### Backend (Rust)
- **Actix-Web** server with MongoDB
- **Greek Mythology Modules**:
  - **Ares** (God of War): Authentication & security
  - **Apollo** (God of Truth): Input validation
  - **Artemis** (Goddess of the Hunt): Rate limiting
  - **Olympus** (Pantheon): CA domain system
  - **Hermes** (Messenger God): WebSocket real-time events

### Frontend Applications

#### Mobile (Flutter)
- **Platform**: Android & iOS
- **Location**: `mobile/`
- **Features**: 
  - Solana wallet generation & encryption
  - AES-GCM encryption with PBKDF2
  - Password-protected wallet storage
  - Material Design 3 UI

#### Desktop (Vue)
- **Platform**: Windows, macOS, Linux
- **Location**: `desktop/`
- **Features**:
  - Solana wallet generation & encryption
  - AES-GCM encryption with PBKDF2
  - Password-protected wallet storage
  - Vue 3 + TypeScript + Vite

#### Legacy Web (React)
- **Platform**: Web browser
- **Location**: `app/` (React/Vite) and `frontend/` (Next.js)
- **Status**: Maintained for web compatibility

### SDK (TypeScript)
- **Hermes SDK**: Developer tools for building Shadow sites
- **Location**: `sdk/`
- **Commands**:
  - `npx hermes-sdk init` - Initialize new site
  - `npx hermes-sdk deploy` - Deploy site to Shadow
  - `npx hermes-sdk convert` - Convert existing site

### Rust Crates
- **hermes-cli**: Rust CLI for Shadow operations
- **hermes-client**: Rust client SDK
- **Location**: `crates/`

## Quick Start

### Mobile (Flutter)
```bash
cd mobile
flutter pub get
flutter run
```

### Desktop (Vue)
```bash
cd desktop
npm install
npm run dev
```

### Backend
```bash
cd backend
cargo run
```

### SDK
```bash
cd sdk
npm install
npm run build
```

## Security

All wallet implementations use:
- **AES-GCM encryption** (256-bit)
- **PBKDF2 key derivation** (100,000 iterations)
- **Random salt and IV** for each encryption
- **Password-protected storage**

## Domain System

Shadow uses the **Olympus CA** system where:
- `.shadow` domains map to Solana program addresses
- Custom domains can be registered
- Domains are verified on-chain
- Token addresses can act as domains

## Development

See individual README files in each directory:
- `mobile/README.md` - Flutter mobile app
- `desktop/README.md` - Vue desktop app
- `backend/BACKEND_OVERVIEW.md` - Backend API
- `sdk/README.md` - Hermes SDK

## License

MIT
