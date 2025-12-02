# Shadow SDK

CLI tool for deploying sites to the Shadow platform.

## Installation

```bash
npm install -g shadow-sdk
```

## Usage

### Initialize a new site

```bash
npx shadow-sdk init my-site
cd my-site
```

### Deploy a site

```bash
npx shadow-sdk deploy
```

### Options

- `--network <network>`: Network to deploy to (default: devnet)
- `--storage <storage>`: Storage provider - `ipfs` or `arweave` (default: ipfs)

## Environment Variables

- `PINATA_API_KEY`: Pinata API key for IPFS storage
- `PINATA_SECRET`: Pinata secret for IPFS storage
- `BUNDLR_PRIVATE_KEY`: Bundlr private key for Arweave storage

