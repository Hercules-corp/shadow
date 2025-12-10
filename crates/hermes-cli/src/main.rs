use anyhow::Result;
use clap::{Parser, Subcommand};
use hermes_client::{convert_site, deploy_site, register_domain, ClientConfig};

#[derive(Parser, Debug)]
#[command(name = "hermes", about = "Hermes CLI (Rust) for Shadow")]
struct Cli {
    /// Backend endpoint, e.g. http://localhost:8787
    #[arg(long, global = true, default_value = "http://localhost:8787")]
    backend: String,

    /// Network/cluster name
    #[arg(long, global = true, default_value = "devnet")]
    network: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Convert an existing site into Shadow format
    Convert {
        /// Path to project directory
        #[arg(default_value = ".")]
        path: String,
    },
    /// Deploy a site (program + assets) via backend pipeline
    Deploy {
        /// Path to shadow.json
        #[arg(default_value = ".")]
        path: String,
        /// Optional domain to register
        #[arg(long)]
        domain: Option<String>,
        /// Mint the site token during deployment
        #[arg(long, default_value_t = false)]
        mint_token: bool,
    },
    /// Register a .shadow domain to a program address
    RegisterDomain {
        /// Domain name, e.g. mysite.shadow
        domain: String,
        /// Program or contract address
        program: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = ClientConfig {
        backend: cli.backend,
        network: cli.network,
    };

    match cli.command {
        Commands::Convert { path } => {
            convert_site(&config, &path).await?;
        }
        Commands::Deploy { path, domain, mint_token } => {
            deploy_site(&config, &path, domain.as_deref(), mint_token).await?;
        }
        Commands::RegisterDomain { domain, program } => {
            register_domain(&config, &domain, &program).await?;
        }
    }

    Ok(())
}


