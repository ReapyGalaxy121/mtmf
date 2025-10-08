use crate::core::config::Config;
use crate::errors::Result;

pub async fn run() -> Result<()> {
    println!("🔍 Running diagnostics...\n");

    let mut all_ok = true;

    // Check 1: Configuration file
    print!("✓ Checking configuration... ");
    match Config::load().await {
        Ok(config) => {
            println!("✓ OK");
            println!("  - Active profile: {}", config.active_profile);
            println!("  - Profiles: {}", config.profiles.len());

            // Check active profile
            match config.get_active_profile() {
                Ok(profile) => {
                    println!("  - Mode: {:?}", profile.mode);
                    println!("  - Network: {:?}", profile.network);
                    println!("  - Storage: {:?}", profile.storage.provider);
                    
                    if profile.storage.api_key.is_none() {
                        println!("  ⚠️  Warning: No storage API key configured");
                        all_ok = false;
                    }
                }
                Err(e) => {
                    println!("  ✗ Error: {}", e);
                    all_ok = false;
                }
            }
        }
        Err(e) => {
            println!("✗ FAILED");
            println!("  Error: {}", e);
            all_ok = false;
        }
    }

    // Check 2: Keys directory
    print!("\n✓ Checking keys... ");
    match crate::core::key::list().await {
        Ok(keys) => {
            if keys.is_empty() {
                println!("⚠️  No keys found");
                println!("  Run 'mtmf key generate --alias my-key' to create one");
            } else {
                println!("✓ OK");
                println!("  - Found {} key(s)", keys.len());
                for key in keys {
                    println!("    - {}", key);
                }
            }
        }
        Err(e) => {
            println!("✗ FAILED");
            println!("  Error: {}", e);
            all_ok = false;
        }
    }

    // Check 3: Network connectivity (basic check)
    print!("\n✓ Checking network connectivity... ");
    match reqwest::Client::new()
        .get("https://www.google.com")
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
    {
        Ok(_) => println!("✓ OK"),
        Err(e) => {
            println!("✗ FAILED");
            println!("  Error: {}", e);
            all_ok = false;
        }
    }

    // Summary
    println!("\n{}", "=".repeat(50));
    if all_ok {
        println!("✅ All checks passed!");
    } else {
        println!("⚠️  Some checks failed. Please review the issues above.");
    }

    Ok(())
}
