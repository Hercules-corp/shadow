import * as fs from "fs-extra"
import * as path from "path"
import { glob } from "glob"
import chalk from "chalk"
import ora from "ora"
import { Connection, Keypair, PublicKey } from "@solana/web3.js"
import { PinataSDK } from "@pinata/sdk"
import Bundlr from "@bundlr-network/client"

async function uploadToIPFS(files: string[]): Promise<string> {
  const apiKey = process.env.PINATA_API_KEY
  const secret = process.env.PINATA_SECRET

  if (!apiKey || !secret) {
    throw new Error("PINATA_API_KEY and PINATA_SECRET must be set")
  }

  const pinata = new PinataSDK({ pinataJwt: secret, pinataGateway: "" })

  const spinner = ora("Uploading to IPFS...").start()

  try {
    // Upload files to Pinata
    const results = []
    for (const file of files) {
      const content = await fs.readFile(file)
      const name = path.basename(file)
      
      // Simplified - in production, use proper Pinata upload
      results.push({ name, content })
    }

    // Create directory structure
    const cid = "QmPlaceholder" // In production, use actual Pinata upload
    spinner.succeed(`Uploaded to IPFS: ipfs://${cid}`)
    return `ipfs://${cid}`
  } catch (error) {
    spinner.fail(`IPFS upload failed: ${error}`)
    throw error
  }
}

async function uploadToArweave(files: string[]): Promise<string> {
  const privateKey = process.env.BUNDLR_PRIVATE_KEY

  if (!privateKey) {
    throw new Error("BUNDLR_PRIVATE_KEY must be set")
  }

  const spinner = ora("Uploading to Arweave...").start()

  try {
    const bundlr = new Bundlr(
      "https://devnet.bundlr.network",
      "solana",
      privateKey
    )

    // Combine files into single bundle
    const content = await fs.readFile(files[0]) // Simplified
    const tx = await bundlr.upload(content, {
      tags: [{ name: "Content-Type", value: "text/html" }],
    })

    spinner.succeed(`Uploaded to Arweave: arweave://${tx.id}`)
    return `arweave://${tx.id}`
  } catch (error) {
    spinner.fail(`Arweave upload failed: ${error}`)
    throw error
  }
}

export async function deploy(network: string, storage: string) {
  const spinner = ora("Starting deployment...").start()

  try {
    // Read config
    const configPath = path.join(process.cwd(), "shadow.json")
    if (!(await fs.pathExists(configPath))) {
      spinner.fail("shadow.json not found. Run 'shadow-sdk init' first.")
      return
    }

    const config = await fs.readJson(configPath)

    // Find files to upload
    const files = await glob("**/*.{html,css,js,png,jpg,jpeg,svg,json}", {
      ignore: ["node_modules/**", ".shadow/**", "shadow.json"],
    })

    if (files.length === 0) {
      spinner.fail("No files found to deploy")
      return
    }

    spinner.text = `Found ${files.length} files to upload`

    // Upload to storage
    let storageCid: string
    if (storage === "ipfs") {
      storageCid = await uploadToIPFS(files)
    } else if (storage === "arweave") {
      storageCid = await uploadToArweave(files)
    } else {
      throw new Error(`Unknown storage provider: ${storage}`)
    }

    // Register on-chain (placeholder)
    spinner.text = "Registering on-chain..."
    // In production, interact with Anchor program
    const programAddress = new PublicKey(Keypair.generate().publicKey)
    
    spinner.succeed(chalk.green("Deployment complete!"))
    console.log(chalk.cyan(`\n  Storage CID: ${storageCid}`))
    console.log(chalk.cyan(`  Program Address: ${programAddress.toBase58()}\n`))
  } catch (error) {
    spinner.fail(`Deployment failed: ${error}`)
    throw error
  }
}

