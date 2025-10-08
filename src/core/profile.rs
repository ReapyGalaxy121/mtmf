use crate::core::config::{Config, Profile};
use crate::errors::{MtmfError, Result};

pub async fn list() -> Result<Vec<String>> {
    let config = Config::load().await?;
    let profile_names: Vec<String> = config
        .profiles
        .iter()
        .map(|p| {
            if p.name == config.active_profile {
                format!("{} (active)", p.name)
            } else {
                p.name.clone()
            }
        })
        .collect();
    Ok(profile_names)
}

pub async fn create(name: &str) -> Result<()> {
    let mut config = Config::load().await?;
    
    // Check if profile already exists
    if config.profiles.iter().any(|p| p.name == name) {
        return Err(MtmfError::Config(format!(
            "Profile '{}' already exists",
            name
        )));
    }

    // Create new profile based on default
    let new_profile = Profile {
        name: name.to_string(),
        ..Profile::default()
    };

    config.profiles.push(new_profile);
    config.save().await?;

    tracing::info!("Created profile: {}", name);
    Ok(())
}

pub async fn switch(name: &str) -> Result<()> {
    let mut config = Config::load().await?;
    
    // Check if profile exists
    if !config.profiles.iter().any(|p| p.name == name) {
        return Err(MtmfError::Config(format!("Profile '{}' not found", name)));
    }

    config.active_profile = name.to_string();
    config.save().await?;

    tracing::info!("Switched to profile: {}", name);
    Ok(())
}

pub async fn delete(name: &str) -> Result<()> {
    let mut config = Config::load().await?;
    
    // Prevent deleting active profile
    if config.active_profile == name {
        return Err(MtmfError::Config(
            "Cannot delete active profile. Switch to another profile first.".to_string(),
        ));
    }

    // Find and remove profile
    let initial_len = config.profiles.len();
    config.profiles.retain(|p| p.name != name);

    if config.profiles.len() == initial_len {
        return Err(MtmfError::Config(format!("Profile '{}' not found", name)));
    }

    config.save().await?;

    tracing::info!("Deleted profile: {}", name);
    Ok(())
}
