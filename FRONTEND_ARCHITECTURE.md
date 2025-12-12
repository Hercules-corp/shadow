# Frontend Architecture - Shadow Browser

## Overview

Shadow Browser has **three separate frontend applications** for different platforms:

1. **Web** - React + Vite (`app/`)
2. **Mobile** - Flutter (`mobile/`)
3. **Desktop** - Vue 3 (`desktop/`)

## Web App (React) - `app/`

**Status**: ✅ Cleaned and ready for designer

### What Was Removed
- ❌ Complex search component (`apple-spotlight.tsx`)
- ❌ Test files (`verify-storage.ts`, `wallet-storage-test.ts`)
- ❌ Complex animations (framer-motion)
- ❌ Tauri desktop integration (using Vue instead)
- ❌ Capacitor mobile integration (using Flutter instead)
- ❌ Unused dependencies (zustand, react-use, etc.)

### What Remains (Core)
- ✅ Wallet provider & password dialog
- ✅ Theme provider & toggle
- ✅ Basic UI components (button, dialog, input)
- ✅ Three pages: Home, Profile, Site
- ✅ Wallet encryption logic (AES-GCM)

### For Designer
See `app/DESIGNER_NOTES.md` for detailed guidelines.

## Mobile App (Flutter) - `mobile/`

**Status**: ✅ Complete structure

### Features
- Flutter + Dart
- Solana wallet integration
- AES-GCM encryption (Dart implementation)
- Material Design 3
- Provider state management

### Setup
```bash
cd mobile
flutter pub get
flutter run
```

## Desktop App (Vue) - `desktop/`

**Status**: ✅ Complete structure

### Features
- Vue 3 + TypeScript + Vite
- Solana wallet integration
- AES-GCM encryption (TypeScript)
- Pinia state management
- Vue Router

### Setup
```bash
cd desktop
npm install
npm run dev
```

## Shared Features

All three frontends share:
- ✅ Same wallet encryption (AES-GCM 256-bit)
- ✅ Same password protection
- ✅ Same backend API integration
- ✅ Same wallet address format

## Backend Integration

All frontends connect to the same Rust backend:
- Base URL: `VITE_BACKEND_URL` or `http://localhost:8080`
- API endpoints documented in `backend/BACKEND_OVERVIEW.md`

## Next Steps for Designer

1. **Web App** (`app/`): 
   - Start with `DESIGNER_NOTES.md`
   - Focus on Home, Profile, and Site pages
   - Keep wallet flow intact

2. **Mobile App** (`mobile/`):
   - Flutter Material Design 3
   - Follow Flutter design patterns

3. **Desktop App** (`desktop/`):
   - Vue 3 composition API
   - Desktop-optimized UI

## Notes

- All wallet logic is **encrypted** and **secure**
- Backend handles all Solana interactions
- Frontends are **presentation-only** (no direct blockchain calls)
- Designer can freely modify UI/UX but should keep wallet flow intact

