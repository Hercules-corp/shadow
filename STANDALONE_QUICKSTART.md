# Shadow Standalone App - Quick Start Guide

## What We've Built

✅ **Vite React App** - Fast, modern React setup  
✅ **Tauri Desktop** - Native Windows/Mac/Linux apps  
✅ **Capacitor Mobile** - Native iOS/Android apps  
✅ **Mobile-Responsive UI** - Beautiful on all devices  
✅ **All Your Components** - Migrated from Next.js  

## Setup Instructions

### Step 1: Install Dependencies

```bash
cd app
npm install
```

### Step 2: Environment Setup

Create `.env` file in the `app` directory:

```env
VITE_BACKEND_URL=http://localhost:8080
VITE_PRIVY_APP_ID=cmi8s0ci700mal80e4hzrdyc4
```

### Step 3: Test Web Version

```bash
npm run dev
```

Visit http://localhost:3000

### Step 4: Build Desktop App (Tauri)

```bash
# Development
npm run tauri:dev

# Production build
npm run tauri:build
```

**Output locations:**
- Windows: `src-tauri/target/release/shadow-app.exe`
- macOS: `src-tauri/target/release/bundle/macos/Shadow.app`
- Linux: `src-tauri/target/release/bundle/appimage/`

### Step 5: Build Mobile Apps (Capacitor)

**First time setup:**
```bash
npm run build
npx cap sync
```

**iOS:**
```bash
npm run cap:ios
# Opens Xcode - build and run from there
```

**Android:**
```bash
npm run cap:android
# Opens Android Studio - build and run from there
```

## Mobile UI Features

✨ **Safe Area Support** - Works with iPhone notch, Android navigation  
✨ **Touch-Friendly** - All buttons are 44px+ for easy tapping  
✨ **Responsive** - Looks great on phones, tablets, and desktops  
✨ **Native Feel** - Smooth animations and interactions  

## What's Different from Next.js?

1. **Routing**: Uses React Router instead of Next.js routing
2. **Environment**: Uses `import.meta.env.VITE_*` instead of `process.env.NEXT_PUBLIC_*`
3. **No SSR**: Pure client-side app (perfect for standalone)
4. **Build**: Vite instead of Next.js build system

## Next Steps

1. ✅ Install dependencies
2. ✅ Test web version
3. ✅ Build desktop app
4. ✅ Build mobile apps
5. 🎉 Distribute your app!

## Tips for Great Mobile Experience

- Test on real devices (not just emulators)
- Use safe area classes for iOS notch
- Ensure touch targets are 44px minimum
- Test in both light and dark mode
- Check performance on older devices

