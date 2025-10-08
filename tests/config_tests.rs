use mtmf::core::config::{Config, Mode, Network, Profile, StorageProvider};
use tempfile::TempDir;
use std::env;

#[tokio::test]
async fn test_config_default() {
    let config = Config::default();
    
    assert_eq!(config.version, "1.0");
    assert_eq!(config.active_profile, "default");
    assert_eq!(config.profiles.len(), 1);
    
    let profile = &config.profiles[0];
    assert_eq!(profile.name, "default");
    assert!(matches!(profile.mode, Mode::SelfCustodial));
    assert!(matches!(profile.network, Network::Testnet));
}

#[tokio::test]
async fn test_config_save_and_load() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    
    // Set custom config path
    env::set_var("HOME", temp_dir.path());
    
    let mut config = Config::default();
    config.active_profile = "test".to_string();
    
    // Save config
    let result = config.save().await;
    assert!(result.is_ok(), "Failed to save config: {:?}", result.err());
    
    // Load config
    let loaded = Config::load().await;
    assert!(loaded.is_ok(), "Failed to load config: {:?}", loaded.err());
    
    let loaded_config = loaded.unwrap();
    assert_eq!(loaded_config.active_profile, "test");
}

#[tokio::test]
async fn test_get_active_profile() {
    let config = Config::default();
    
    let profile = config.get_active_profile();
    assert!(profile.is_ok());
    
    let profile = profile.unwrap();
    assert_eq!(profile.name, "default");
}

#[tokio::test]
async fn test_profile_default() {
    let profile = Profile::default();
    
    assert_eq!(profile.name, "default");
    assert!(matches!(profile.mode, Mode::SelfCustodial));
    assert!(matches!(profile.network, Network::Testnet));
    assert!(matches!(profile.storage.provider, StorageProvider::Pinata));
    assert!(profile.storage.api_key.is_none());
    assert!(profile.flow.address.is_none());
}

#[test]
fn test_storage_provider_serialization() {
    let provider = StorageProvider::Pinata;
    let serialized = serde_json::to_string(&provider).unwrap();
    assert_eq!(serialized, "\"pinata\"");
    
    let provider = StorageProvider::Lighthouse;
    let serialized = serde_json::to_string(&provider).unwrap();
    assert_eq!(serialized, "\"lighthouse\"");
}

#[test]
fn test_mode_serialization() {
    let mode = Mode::SelfCustodial;
    let serialized = serde_json::to_string(&mode).unwrap();
    assert_eq!(serialized, "\"selfcustodial\"");
    
    let mode = Mode::Managed;
    let serialized = serde_json::to_string(&mode).unwrap();
    assert_eq!(serialized, "\"managed\"");
}

#[test]
fn test_network_serialization() {
    let network = Network::Testnet;
    let serialized = serde_json::to_string(&network).unwrap();
    assert_eq!(serialized, "\"testnet\"");
    
    let network = Network::Mainnet;
    let serialized = serde_json::to_string(&network).unwrap();
    assert_eq!(serialized, "\"mainnet\"");
}
