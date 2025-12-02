# Shadow Platform - Setup Guide

This guide will help you set up and run the Shadow platform locally.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (latest stable) - [Install Rust](https://rustup.rs/)
- **Node.js** 20+ - [Install Node.js](https://nodejs.org/)
- **MongoDB** (optional, can use Docker) - [Install MongoDB](https://www.mongodb.com/try/download/community)
- **Docker & Docker Compose** - [Install Docker](https://docs.docker.com/get-docker/)
- **Solana CLI** (for program deployment) - [Install Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- **Anchor** (for Solana programs) - [Install Anchor](https://www.anchor-lang.com/docs/installation)

## Environment Setup

1. **Copy the environment file:**

```bash
cp env.example .env
# Or manually create .env file using env.example as a template
```

2. **Edit `.env` with your configuration:**

Required variables:
- `DATABASE_URL` - MongoDB connection string (e.g., `mongodb://localhost:27017/shadow`)
- `SOLANA_RPC_URL` - Solana RPC endpoint (devnet: https://api.devnet.solana.com)
- `PRIVY_APP_ID` - Get from [Privy Dashboard](https://dashboard.privy.io/)
- `PINATA_API_KEY` & `PINATA_SECRET` - Get from [Pinata](https://www.pinata.cloud/)
- `BUNDLR_PRIVATE_KEY` - For Arweave uploads (optional)

## Database Setup

1. **Start MongoDB with Docker:**

```bash
docker-compose up mongodb -d
```

2. **No migrations needed!**

MongoDB automatically creates collections and indexes when the backend starts. The backend will create the necessary indexes on startup.

## Solana Programs

1. **Build the programs:**

```bash
cd programs
anchor build
```

2. **Deploy to devnet:**

```bash
anchor deploy
```

Note: Update program IDs in `programs/Anchor.toml` after deployment.

## Backend Setup

1. **Navigate to backend directory:**

```bash
cd backend
```

2. **Install dependencies (first time only):**

Rust dependencies are managed by Cargo and will install automatically.

3. **Run the backend:**

```bash
cargo run
```

The backend will start on `http://localhost:8080`

## Frontend Setup

1. **Navigate to frontend directory:**

```bash
cd frontend
```

2. **Install dependencies:**

```bash
npm install
```

3. **Run development server:**

```bash
npm run dev
```

The frontend will start on `http://localhost:3000`

## Docker Compose (All Services)

Alternatively, run everything with Docker Compose:

```bash
docker-compose up
```

This will start:
- PostgreSQL on port 5432
- Backend on port 8080
- Frontend on port 3000

## SDK Setup

1. **Navigate to SDK directory:**

```bash
cd sdk
```

2. **Install dependencies:**

```bash
npm install
```

3. **Build the SDK:**

```bash
npm run build
```

4. **Link globally (optional):**

```bash
npm link
```

Now you can use `shadow-sdk` from anywhere.

## Testing the Platform

1. **Start all services** (via Docker Compose or individually)

2. **Open the frontend** at http://localhost:3000

3. **Sign in with Google** - This will create a Solana wallet via Web3Auth

4. **Use Cmd+K (or Ctrl+K)** to open the Apple Spotlight search

5. **Search for profiles or sites** by wallet address or program address

6. **Deploy a test site:**

```bash
npx shadow-sdk init test-site
cd test-site
npx shadow-sdk deploy
```

## Troubleshooting

### Backend won't start

- Check that MongoDB is running
- Verify `DATABASE_URL` in `.env` is correct (format: `mongodb://localhost:27017/shadow`)
- Ensure MongoDB is accessible

### Frontend build errors

- Delete `node_modules` and `.next` folders
- Run `npm install` again
- Check Node.js version (should be 20+)

### Solana program deployment fails

- Ensure Solana CLI is installed
- Configure Solana CLI: `solana config set --url devnet`
- Get devnet SOL: `solana airdrop 2`

### Privy errors

- Verify `PRIVY_APP_ID` is set correctly
- Check that your Privy app is configured for Google login and embedded wallets
- Ensure Solana network is enabled in your Privy app settings

## Next Steps

- Customize your profiles and sites
- Deploy to mainnet (change network settings)
- Set up Railway deployment (see README.md)

## Support

For issues or questions, check the main README.md or open an issue on GitHub.

