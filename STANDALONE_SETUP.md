# Shadow - Standalone Application Setup

This guide will help you convert Shadow into a beautiful standalone application for iOS, Android, and Desktop (Windows/Mac/Linux).

## Architecture

### Desktop (Windows/Mac/Linux)
- **Tauri** - Rust-based framework
  - Uses your existing Rust backend
  - Embeds React frontend
  - Native performance, small bundle size
  - Secure by default

### Mobile (iOS/Android)
- **Capacitor** - Native runtime
  - Wraps React app as native
  - Access to device features
  - App Store ready

## Setup Steps

### 1. Convert Next.js to Vite React App

Next.js is optimized for web, but for standalone apps, Vite is better:
- Faster builds
- Better for desktop/mobile
- Smaller bundle size
- Easier to integrate with Tauri/Capacitor

### 2. Install Tauri (Desktop)

```bash
cd frontend
npm install --save-dev @tauri-apps/cli
npm install @tauri-apps/api
```

### 3. Install Capacitor (Mobile)

```bash
npm install @capacitor/core @capacitor/cli
npm install @capacitor/ios @capacitor/android
npx cap init
```

### 4. Mobile UI Enhancements

- Add responsive breakpoints
- Touch-friendly components
- Native navigation
- Safe area handling for iOS
- Material Design for Android

## Project Structure

```
shadow/
├── backend/          # Rust backend (Tauri will use this)
├── frontend/         # React app (Vite)
│   ├── src/
│   ├── tauri/        # Tauri config
│   └── capacitor/    # Capacitor config
└── programs/         # Solana programs
```

## Next Steps

1. Convert Next.js → Vite React
2. Set up Tauri
3. Set up Capacitor
4. Add mobile-responsive UI
5. Integrate native features

