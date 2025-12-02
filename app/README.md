# Shadow - Standalone Application

Beautiful standalone app for iOS, Android, and Desktop (Windows/Mac/Linux).

## 🚀 Quick Start

### 1. Install Dependencies

```bash
npm install
```

### 2. Set Up Environment

Create `.env` file:

```bash
cp .env.example .env
```

Edit `.env` with your values:
- `VITE_BACKEND_URL` - Your backend URL (default: http://localhost:8080)
- `VITE_PRIVY_APP_ID` - Your Privy App ID

### 3. Development

```bash
# Web development (for testing)
npm run dev

# Desktop app (Tauri)
npm run tauri:dev

# Mobile (after Capacitor setup)
npm run cap:sync
npm run cap:ios      # iOS
npm run cap:android  # Android
```

## 📱 Building for Platforms

### Desktop (Tauri)

**Windows:**
```bash
npm run tauri:build
# Output: src-tauri/target/release/shadow-app.exe
```

**macOS:**
```bash
npm run tauri:build
# Output: src-tauri/target/release/bundle/macos/Shadow.app
```

**Linux:**
```bash
npm run tauri:build
# Output: src-tauri/target/release/bundle/appimage/shadow_*.AppImage
```

### Mobile (Capacitor)

**iOS:**
```bash
npm run cap:sync
npm run cap:ios
# Opens Xcode - build from there
```

**Android:**
```bash
npm run cap:sync
npm run cap:android
# Opens Android Studio - build from there
```

## 🎨 Mobile UI Features

- ✅ Safe area support (iOS notch, Android navigation)
- ✅ Touch-friendly buttons (44px minimum)
- ✅ Responsive design (mobile-first)
- ✅ Native feel animations
- ✅ Dark mode support

## 📦 Project Structure

```
app/
├── src/
│   ├── components/    # UI components
│   ├── pages/         # App pages
│   ├── lib/           # Utilities
│   └── main.tsx       # Entry point
├── src-tauri/         # Tauri desktop config
├── capacitor.config.ts # Capacitor mobile config
└── package.json
```

## 🔧 Configuration

### Tauri (Desktop)
- Config: `src-tauri/tauri.conf.json`
- Window size: 1200x800 (min: 800x600)
- Bundle ID: `com.shadow.app`

### Capacitor (Mobile)
- Config: `capacitor.config.ts`
- App ID: `com.shadow.app`
- Web dir: `dist`

## 📝 Notes

- Uses Vite for fast builds
- React Router for navigation
- Tailwind CSS for styling
- Framer Motion for animations
- All your existing components work!

## 🐛 Troubleshooting

**Tauri build fails:**
- Ensure Rust is installed
- Check `src-tauri/Cargo.toml` is correct

**Capacitor sync fails:**
- Run `npm run build` first
- Check `capacitor.config.ts` paths

**Mobile UI issues:**
- Check safe area CSS classes
- Verify viewport meta tag in `index.html`

