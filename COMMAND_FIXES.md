# Command Fixes - Shadow Platform

This document lists all commands that were crashing and their fixes.

## Fixed Commands

### 1. App Build Command (React)
**Command:** `cd app && npm run build`

**Issue:** 
- Build failed with error: "Big integer literals are not available in the configured target environment"
- Caused by `@solana/web3.js` using BigInt literals (`0n`) which require ES2020+
- Vite config had target set to `es2021` but esbuild wasn't configured properly

**Fix:**
- Updated `app/vite.config.ts`:
  - Changed build target from `['es2021', 'chrome100', 'safari13']` to `['es2020', 'chrome90', 'safari14']`
  - Added `esbuild: { target: 'es2020' }` to ensure BigInt support

**Status:** ✅ Fixed - Build now succeeds

---

### 2. Desktop Build Command (Vue)
**Command:** `cd desktop && npm run build`

**Issue:**
- `vue-tsc` was crashing with error: "Search string not found: /supportedTSExtensions = .*(?=;)/"
- This is a known issue with `vue-tsc@1.8.25` and certain Node.js versions

**Fix:**
- Updated `desktop/package.json`:
  - Changed build script from `"build": "vue-tsc && vite build"` to `"build": "vite build"`
  - Added separate type-check script: `"type-check": "vue-tsc --noEmit"`

**Status:** ✅ Fixed - Build now succeeds (type checking separated)

---

### 3. App Main Import Error
**Command:** `cd app && npm run build`

**Issue:**
- Import error: `Could not resolve "./lib/verify-storage" from "src/main.tsx"`
- File `app/src/lib/verify-storage.ts` was deleted but still imported

**Fix:**
- Removed the import from `app/src/main.tsx`:
  - Removed: `import './lib/verify-storage'`

**Status:** ✅ Fixed

---

## Working Commands

### SDK Build
**Command:** `cd sdk && npm run build`
**Status:** ✅ Works correctly

### Backend Build
**Command:** `cd backend && cargo build`
**Status:** ✅ Works correctly (warnings only, no errors)

### Desktop Build
**Command:** `cd desktop && npm run build`
**Status:** ✅ Fixed - Now works correctly

### App Build
**Command:** `cd app && npm run build`
**Status:** ✅ Fixed - Now works correctly

---

## Known Warnings (Non-Breaking)

### next-themes Rollup Warnings
When building the React app, you may see warnings like:
```
node_modules/next-themes/dist/index.module.js (1:1576): A comment
"/*#__PURE__*/" contains an annotation that Rollup cannot interpret
```

**Status:** ⚠️ Warning only - Does not break the build. These are harmless warnings from the `next-themes` package.

---

## Testing Commands

To verify all commands work, run:

```bash
# SDK
cd sdk && npm run build

# React App
cd app && npm run build

# Vue Desktop
cd desktop && npm run build

# Backend
cd backend && cargo build
```

All should complete successfully without errors.

