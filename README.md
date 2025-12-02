# Shadow

> "You are whatever your wallet is. Forget who you are online."

Shadow is a Tor-inspired, wallet-only pseudonymous web platform built on Solana. Users and developers can create profiles, build and deploy sites and dApps that live forever, all identified solely by Solana wallets — no emails, no usernames, no real-world linkage.

## Tech Stack

- **Blockchain**: Solana (devnet/mainnet)
- **On-chain Programs**: Rust + Anchor
- **Backend**: Rust (Actix-Web + Actix-WS)
- **Frontend**: Next.js 15 (App Router) + shadcn/ui + Tailwind CSS
- **Database**: MongoDB
- **Storage**: IPFS (Pinata) + Arweave (Bundlr)
- **Auth**: Privy (Google login → Solana wallet)

## Quick Start

### Prerequisites

- Rust (latest stable)
- Node.js 20+
- MongoDB (or use Docker)
- Docker & Docker Compose
- Solana CLI (for local devnet)

### Local Development

1. **Clone and setup environment:**

```bash
# Create .env file from example
cp env.example .env
# Or manually create .env file with the contents from env.example
# Edit .env with your keys
```

2. **Start MongoDB:**

```bash
docker-compose up mongodb -d
```

3. **Database setup:**

MongoDB automatically creates collections and indexes when the backend starts. No manual migrations needed!

4. **Build Solana programs:**

```bash
cd programs
anchor build
anchor deploy
```

5. **Start backend:**

```bash
cd backend
cargo run
```

6. **Start frontend:**

```bash
cd frontend
npm install
npm run dev
```

7. **Or use Docker Compose (all services):**

```bash
docker-compose up
```

## Project Structure

```
/shadow
  /programs          # Anchor Rust programs (registry + profiles)
  /backend           # Rust Actix-Web server
  /frontend          # Next.js app (the Shadow browser)
    /components/ui   # shadcn components + apple-spotlight.tsx
    /app             # Pages
  /sdk               # shadow-sdk CLI (TypeScript)
  /migrations        # SQLx database migrations
  docker-compose.yml
  Dockerfile
  railway.toml
  .env.example
```

## Features

- **Wallet-Only Identity**: Profiles tied to Solana wallet addresses
- **Anonymous by Default**: Optional public mode toggle
- **Site Registry**: Solana program addresses as site URLs
- **Real-time Updates**: WebSocket subscriptions to Solana events
- **IPFS/Arweave Storage**: Decentralized content hosting
- **Apple Spotlight Search**: Integrated search bar for navigation
- **Dark Mode**: Full dark mode support with smooth animations

## Deployment

### Railway

1. Connect your repository
2. Add PostgreSQL plugin
3. Set environment variables from `.env.example`
4. Deploy!

The `railway.toml` handles the multi-stage build automatically.

## SDK Usage

Deploy a site using the Shadow SDK:

```bash
npx shadow-sdk init my-site
cd my-site
npx shadow-sdk deploy
```

## License

MIT

