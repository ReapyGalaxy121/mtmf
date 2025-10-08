use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use mtmf::{core, Result};

#[derive(Parser)]
#[command(name = "mtmf")]
#[command(about = "MintThisMF - A CLI tool for minting NFTs on Flow blockchain", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize mtmf configuration
    Init {
        /// Force re-initialization
        #[arg(short, long)]
        force: bool,
    },

    /// Generate, import, or manage keys
    #[command(subcommand)]
    Key(KeyCommands),

    /// Mint an NFT from a media file
    Mint {
        /// Path to the media file
        file: String,

        /// NFT name
        #[arg(short, long)]
        name: Option<String>,

        /// NFT description
        #[arg(short, long)]
        description: Option<String>,

        /// Recipient Flow address
        #[arg(short, long)]
        to: Option<String>,

        /// Output in JSON format
        #[arg(long)]
        json: bool,

        /// Dry run (don't submit transaction)
        #[arg(long)]
        dry_run: bool,
    },

    /// Upload a file to storage
    Upload {
        /// Path to the file
        file: String,

        /// Storage provider (ipfs, arweave)
        #[arg(short, long)]
        storage: Option<String>,
    },

    /// Manage profiles
    #[command(subcommand)]
    Profile(ProfileCommands),

    /// Run diagnostics
    Doctor,
}

#[derive(Subcommand)]
enum KeyCommands {
    /// Generate a new key pair
    Generate {
        /// Key alias
        #[arg(short, long)]
        alias: String,
    },

    /// Import an existing key
    Import {
        /// Key alias
        #[arg(short, long)]
        alias: String,

        /// Path to key file
        #[arg(short, long)]
        file: String,
    },

    /// Export a key
    Export {
        /// Key alias
        #[arg(short, long)]
        alias: String,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },

    /// List all keys
    List,
}

#[derive(Subcommand)]
enum ProfileCommands {
    /// List all profiles
    List,

    /// Create a new profile
    Create {
        /// Profile name
        name: String,
    },

    /// Switch to a profile
    Use {
        /// Profile name
        name: String,
    },

    /// Delete a profile
    Delete {
        /// Profile name
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("mtmf={}", log_level).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    match cli.command {
        Commands::Init { force } => {
            tracing::info!("Initializing mtmf configuration...");
            core::config::init(force).await?;
            println!("✓ Configuration initialized successfully");
        }
        Commands::Key(key_cmd) => match key_cmd {
            KeyCommands::Generate { alias } => {
                tracing::info!("Generating new key pair: {}", alias);
                core::key::generate(&alias).await?;
                println!("✓ Key pair generated: {}", alias);
            }
            KeyCommands::Import { alias, file } => {
                tracing::info!("Importing key: {}", alias);
                core::key::import(&alias, &file).await?;
                println!("✓ Key imported: {}", alias);
            }
            KeyCommands::Export { alias, output } => {
                tracing::info!("Exporting key: {}", alias);
                core::key::export(&alias, output.as_deref()).await?;
                println!("✓ Key exported: {}", alias);
            }
            KeyCommands::List => {
                let keys = core::key::list().await?;
                if keys.is_empty() {
                    println!("No keys found");
                } else {
                    println!("Available keys:");
                    for key in keys {
                        println!("  - {}", key);
                    }
                }
            }
        },
        Commands::Mint {
            file,
            name,
            description,
            to,
            json,
            dry_run,
        } => {
            tracing::info!("Minting NFT from: {}", file);
            core::mint::mint(&file, name, description, to, json, dry_run).await?;
        }
        Commands::Upload { file, storage } => {
            tracing::info!("Uploading file: {}", file);
            let cid = core::storage::upload(&file, storage.as_deref()).await?;
            println!("✓ File uploaded: {}", cid);
        }
        Commands::Profile(profile_cmd) => match profile_cmd {
            ProfileCommands::List => {
                let profiles = core::profile::list().await?;
                if profiles.is_empty() {
                    println!("No profiles found");
                } else {
                    println!("Available profiles:");
                    for profile in profiles {
                        println!("  - {}", profile);
                    }
                }
            }
            ProfileCommands::Create { name } => {
                tracing::info!("Creating profile: {}", name);
                core::profile::create(&name).await?;
                println!("✓ Profile created: {}", name);
            }
            ProfileCommands::Use { name } => {
                tracing::info!("Switching to profile: {}", name);
                core::profile::switch(&name).await?;
                println!("✓ Active profile: {}", name);
            }
            ProfileCommands::Delete { name } => {
                tracing::info!("Deleting profile: {}", name);
                core::profile::delete(&name).await?;
                println!("✓ Profile deleted: {}", name);
            }
        },
        Commands::Doctor => {
            tracing::info!("Running diagnostics...");
            core::doctor::run().await?;
        }
    }

    Ok(())
}
