import * as fs from "fs-extra"
import * as path from "path"
import chalk from "chalk"
import ora from "ora"

export async function init(name: string) {
  const spinner = ora(`Initializing Shadow site: ${name}`).start()

  try {
    const dir = path.resolve(process.cwd(), name)

    if (await fs.pathExists(dir)) {
      spinner.fail(`Directory ${name} already exists`)
      return
    }

    await fs.mkdirp(dir)

    // Create index.html template
    const indexHtml = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${name}</title>
    <style>
        body {
            font-family: system-ui, -apple-system, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
            background: #0a0a0a;
            color: #fff;
        }
        h1 {
            margin: 0 0 1rem 0;
        }
        p {
            color: #888;
        }
    </style>
</head>
<body>
    <h1>Welcome to ${name}</h1>
    <p>This is your Shadow site. Edit this file to customize your content.</p>
</body>
</html>`

    await fs.writeFile(path.join(dir, "index.html"), indexHtml)

    // Create shadow.json config
    const config = {
      name: name,
      version: "0.1.0",
      storage: "ipfs",
      network: "devnet",
    }

    await fs.writeFile(
      path.join(dir, "shadow.json"),
      JSON.stringify(config, null, 2)
    )

    // Create .gitignore
    await fs.writeFile(
      path.join(dir, ".gitignore"),
      "node_modules/\n.shadow/\n*.log\n"
    )

    spinner.succeed(chalk.green(`Shadow site initialized: ${name}`))
    console.log(chalk.cyan(`\n  cd ${name}`))
    console.log(chalk.cyan(`  npx shadow-sdk deploy\n`))
  } catch (error) {
    spinner.fail(`Failed to initialize: ${error}`)
    throw error
  }
}

