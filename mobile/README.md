# Shadow Mobile (Flutter)

Mobile application for Shadow Browser built with Flutter for Android and iOS.

## Features

- **Hermes Wallet Service**: Secure Solana wallet generation and encryption
- **AES-GCM Encryption**: 256-bit encryption with PBKDF2 key derivation
- **Password-Protected Storage**: Wallets stored securely in device storage
- **Material Design 3**: Modern UI with dark mode support

## Setup

1. Install Flutter: https://flutter.dev/docs/get-started/install
2. Install dependencies:
```bash
flutter pub get
```

3. Run on device/emulator:
```bash
flutter run
```

## Architecture

- `lib/services/wallet_service.dart`: Wallet encryption/decryption (Hermes)
- `lib/providers/wallet_provider.dart`: State management for wallet
- `lib/screens/`: UI screens (Home, Profile, Site)
- `lib/widgets/`: Reusable widgets (PasswordDialog)

## Security

- Wallets encrypted with AES-GCM (256-bit)
- PBKDF2 key derivation (100,000 iterations)
- Random salt and IV for each encryption
- Secure storage via SharedPreferences

