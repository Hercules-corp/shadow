#!/usr/bin/env node

import { Command } from "commander"
import { init } from "./commands/init"
import { deploy } from "./commands/deploy"
import chalk from "chalk"

const program = new Command()

program
  .name("shadow-sdk")
  .description("Shadow CLI SDK for deploying sites to the decentralized web")
  .version("0.1.0")

program
  .command("init")
  .description("Initialize a new Shadow site")
  .argument("[name]", "Name of the site")
  .action(async (name) => {
    try {
      await init(name || "my-site")
    } catch (error) {
      console.error(chalk.red("Error:"), error)
      process.exit(1)
    }
  })

program
  .command("deploy")
  .description("Deploy a Shadow site")
  .option("-n, --network <network>", "Network to deploy to", "devnet")
  .option("-s, --storage <storage>", "Storage provider (ipfs|arweave)", "ipfs")
  .action(async (options) => {
    try {
      await deploy(options.network, options.storage)
    } catch (error) {
      console.error(chalk.red("Error:"), error)
      process.exit(1)
    }
  })

program.parse()

