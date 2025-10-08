use crate::errors::{MtmfError, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub active_profile: String,
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub mode: Mode,
    pub network: Network,
    pub storage: StorageConfig,
    pub flow: FlowConfig,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    SelfCustodial,
    Managed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    Testnet,
    Mainnet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub provider: StorageProvider,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageProvider {
    Pinata,
    Lighthouse,
    Arweave,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowConfig {
    pub address: Option<String>,
    pub key_alias: Option<String>,
    pub access_node: String,
}

impl Config {
    pub fn default_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "mtmf", "mtmf")
            .ok_or_else(|| MtmfError::Config("Could not determine config directory".to_string()))?;
        
        let config_dir = proj_dirs.config_dir();
        Ok(config_dir.join("config.toml"))
    }

    pub async fn load() -> Result<Self> {
        let path = Self::default_path()?;
        
        if !path.exists() {
            return Err(MtmfError::Config(
                "Configuration not found. Run 'mtmf init' first.".to_string(),
            ));
        }

        let content = fs::read_to_string(&path).await?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub async fn save(&self) -> Result<()> {
        let path = Self::default_path()?;
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content).await?;
        Ok(())
    }

    pub fn get_active_profile(&self) -> Result<&Profile> {
        self.profiles
            .iter()
            .find(|p| p.name == self.active_profile)
            .ok_or_else(|| {
                MtmfError::Config(format!("Active profile '{}' not found", self.active_profile))
            })
    }

    pub fn get_active_profile_mut(&mut self) -> Result<&mut Profile> {
        let active_name = self.active_profile.clone();
        self.profiles
            .iter_mut()
            .find(|p| p.name == active_name)
            .ok_or_else(|| {
                MtmfError::Config(format!("Active profile '{}' not found", active_name))
            })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version: "1.0".to_string(),
            active_profile: "default".to_string(),
            profiles: vec![Profile::default()],
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            name: "default".to_string(),
            mode: Mode::SelfCustodial,
            network: Network::Testnet,
            storage: StorageConfig {
                provider: StorageProvider::Pinata,
                api_key: None,
            },
            flow: FlowConfig {
                address: None,
                key_alias: None,
                access_node: "access.devnet.nodes.onflow.org:9000".to_string(),
            },
        }
    }
}

pub async fn init(force: bool) -> Result<()> {
    let path = Config::default_path()?;
    
    if path.exists() && !force {
        return Err(MtmfError::Config(
            "Configuration already exists. Use --force to overwrite.".to_string(),
        ));
    }

    println!("🚀 Welcome to MintThisMF!");
    println!("Let's set up your configuration.\n");

    // Interactive prompts
    let profile_name = prompt_input("Profile name", Some("default"))?;
    let mode = prompt_mode()?;
    let network = prompt_network()?;
    let storage_provider = prompt_storage_provider()?;
    
    println!("\n📦 Storage Configuration");
    let api_key = if matches!(storage_provider, StorageProvider::Pinata | StorageProvider::Lighthouse) {
        let key = prompt_input("API Key (leave empty to set later)", None)?;
        if key.is_empty() { None } else { Some(key) }
    } else {
        None
    };

    let access_node = match network {
        Network::Testnet => "access.devnet.nodes.onflow.org:9000".to_string(),
        Network::Mainnet => "access.mainnet.nodes.onflow.org:9000".to_string(),
    };

    let profile = Profile {
        name: profile_name.clone(),
        mode,
        network,
        storage: StorageConfig {
            provider: storage_provider,
            api_key,
        },
        flow: FlowConfig {
            address: None,
            key_alias: None,
            access_node,
        },
    };

    let config = Config {
        version: "1.0".to_string(),
        active_profile: profile_name,
        profiles: vec![profile],
    };

    config.save().await?;

    println!("\n✅ Configuration saved to: {}", path.display());
    println!("\n💡 Next steps:");
    if matches!(mode, Mode::SelfCustodial) {
        println!("  1. Generate a key: mtmf key generate --alias my-key");
        println!("  2. Get testnet FLOW: https://testnet-faucet.onflow.org/");
    }
    println!("  3. Mint your first NFT: mtmf mint image.png");
    
    Ok(())
}

fn prompt_input(prompt: &str, default: Option<&str>) -> Result<String> {
    use std::io::{self, Write};
    
    if let Some(def) = default {
        print!("{} [{}]: ", prompt, def);
    } else {
        print!("{}: ", prompt);
    }
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input.is_empty() {
        if let Some(def) = default {
            Ok(def.to_string())
        } else {
            Ok(String::new())
        }
    } else {
        Ok(input.to_string())
    }
}

fn prompt_mode() -> Result<Mode> {
    println!("\n🔐 Select mode:");
    println!("  1. Self-custodial (you manage your own keys)");
    println!("  2. Managed (use a managed service)");
    
    let choice = prompt_input("Choice", Some("1"))?;
    
    match choice.as_str() {
        "1" => Ok(Mode::SelfCustodial),
        "2" => Ok(Mode::Managed),
        _ => Ok(Mode::SelfCustodial),
    }
}

fn prompt_network() -> Result<Network> {
    println!("\n🌐 Select network:");
    println!("  1. Testnet (for development)");
    println!("  2. Mainnet (production)");
    
    let choice = prompt_input("Choice", Some("1"))?;
    
    match choice.as_str() {
        "1" => Ok(Network::Testnet),
        "2" => Ok(Network::Mainnet),
        _ => Ok(Network::Testnet),
    }
}

fn prompt_storage_provider() -> Result<StorageProvider> {
    println!("\n📦 Select storage provider:");
    println!("  1. Pinata (IPFS)");
    println!("  2. Lighthouse (IPFS)");
    println!("  3. Arweave");
    
    let choice = prompt_input("Choice", Some("1"))?;
    
    match choice.as_str() {
        "1" => Ok(StorageProvider::Pinata),
        "2" => Ok(StorageProvider::Lighthouse),
        "3" => Ok(StorageProvider::Arweave),
        _ => Ok(StorageProvider::Pinata),
    }
}
