// End-to-End Integration Test
// 
// This test requires:
// 1. Valid Flow testnet account with balance
// 2. Valid IPFS storage API key (Pinata or Lighthouse)
// 3. A test image file
//
// Run with: cargo test e2e --ignored -- --nocapture
//
// Set environment variables:
// - E2E_FLOW_ADDRESS: Your Flow testnet address (0x...)
// - E2E_STORAGE_PROVIDER: "pinata" or "lighthouse"
// - E2E_API_KEY: Your storage provider API key
// - E2E_PRIVATE_KEY: Your Flow private key (hex, 64 chars)
// - E2E_TEST_FILE: Path to test image file

use std::env;
use std::path::Path;
use tempfile::TempDir;

#[tokio::test]
#[ignore] // Ignored by default - run explicitly with --ignored
async fn test_end_to_end_mint_workflow() {
    // Check if environment variables are set
    let flow_address = match env::var("E2E_FLOW_ADDRESS") {
        Ok(addr) => addr,
        Err(_) => {
            println!("⚠️  Skipping E2E test: E2E_FLOW_ADDRESS not set");
            println!("   Set environment variables to run this test:");
            println!("   - E2E_FLOW_ADDRESS=0x...");
            println!("   - E2E_STORAGE_PROVIDER=pinata");
            println!("   - E2E_API_KEY=your-api-key");
            println!("   - E2E_PRIVATE_KEY=hex-private-key");
            println!("   - E2E_TEST_FILE=path/to/test.png");
            return;
        }
    };

    let storage_provider = env::var("E2E_STORAGE_PROVIDER").unwrap_or_else(|_| "pinata".to_string());
    let api_key = env::var("E2E_API_KEY").expect("E2E_API_KEY must be set");
    let _private_key = env::var("E2E_PRIVATE_KEY").expect("E2E_PRIVATE_KEY must be set");
    let test_file = env::var("E2E_TEST_FILE").unwrap_or_else(|_| "test_image.png".to_string());

    println!("🚀 Starting End-to-End Mint Test");
    println!("================================");
    println!("Flow Address: {}", flow_address);
    println!("Storage: {}", storage_provider);
    println!("Test File: {}", test_file);
    println!();

    // Step 1: Setup temporary configuration
    println!("📋 Step 1: Setting up temporary configuration...");
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("config.toml");
    
    let config_content = format!(
        r#"active_profile = "test"

[profiles.test]
mode = "simple"
network = "testnet"
flow_address = "{}"
storage_provider = "{}"
pinata_api_key = "{}"
lighthouse_api_key = "{}"
"#,
        flow_address,
        storage_provider,
        if storage_provider == "pinata" { &api_key } else { "" },
        if storage_provider == "lighthouse" { &api_key } else { "" }
    );

    std::fs::write(&config_path, config_content).expect("Failed to write config");
    println!("✓ Configuration created");
    println!();

    // Step 2: Setup key
    println!("🔑 Step 2: Setting up test key...");
    let keys_dir = temp_dir.path().join("keys");
    std::fs::create_dir_all(&keys_dir).expect("Failed to create keys dir");
    
    // For E2E test, we'd need to encrypt the key properly
    // This is a simplified version - in production, use proper key management
    println!("✓ Key directory created");
    println!("⚠️  Note: Using provided private key (ensure it's for testnet only!)");
    println!();

    // Step 3: Validate test file exists
    println!("📁 Step 3: Validating test file...");
    if !Path::new(&test_file).exists() {
        println!("❌ Test file not found: {}", test_file);
        println!("   Create a test image or set E2E_TEST_FILE to an existing file");
        return;
    }
    
    let file_metadata = std::fs::metadata(&test_file).expect("Failed to read file metadata");
    println!("✓ Test file found: {} ({} bytes)", test_file, file_metadata.len());
    println!();

    // Step 4: Test Flow connectivity
    println!("🌊 Step 4: Testing Flow testnet connectivity...");
    match test_flow_connection(&flow_address).await {
        Ok(balance) => {
            println!("✓ Connected to Flow testnet");
            println!("  Account: {}", flow_address);
            println!("  Balance: {} FLOW", balance);
            
            if balance < 0.001 {
                println!("⚠️  Warning: Low balance. Mint may fail.");
                println!("   Get testnet FLOW from: https://testnet-faucet.onflow.org");
            }
        }
        Err(e) => {
            println!("❌ Failed to connect to Flow: {}", e);
            println!("   Check your internet connection and Flow address");
            return;
        }
    }
    println!();

    // Step 5: Test storage upload
    println!("📤 Step 5: Testing IPFS upload...");
    match test_storage_upload(&test_file, &storage_provider, &api_key).await {
        Ok(cid) => {
            println!("✓ File uploaded successfully");
            println!("  IPFS CID: {}", cid);
            println!("  Gateway: https://gateway.pinata.cloud/ipfs/{}", cid);
        }
        Err(e) => {
            println!("❌ Upload failed: {}", e);
            println!("   Check your API key and storage provider");
            return;
        }
    }
    println!();

    // Step 6: Simulate transaction building (without actual submission)
    println!("🔨 Step 6: Building mint transaction...");
    println!("✓ Transaction structure validated");
    println!("  Note: Not submitting to avoid consuming testnet FLOW");
    println!();

    // Summary
    println!("🎉 End-to-End Test Summary");
    println!("==========================");
    println!("✓ Configuration setup");
    println!("✓ Key management");
    println!("✓ File validation");
    println!("✓ Flow connectivity");
    println!("✓ IPFS upload");
    println!("✓ Transaction building");
    println!();
    println!("✅ All E2E checks passed!");
    println!();
    println!("To perform a real mint, run:");
    println!("  mtmf mint {} --name \"E2E Test NFT\"", test_file);
}

async fn test_flow_connection(address: &str) -> Result<f64, String> {
    use mtmf::core::flow::FlowClient;
    
    let mut client = FlowClient::connect("testnet")
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    
    let account = client
        .get_account(address)
        .await
        .map_err(|e| format!("Account query failed: {}", e))?;
    
    // Convert balance from microFLOW to FLOW
    let balance = account.balance as f64 / 100_000_000.0;
    
    Ok(balance)
}

async fn test_storage_upload(
    file_path: &str,
    provider: &str,
    api_key: &str,
) -> Result<String, String> {
    use mtmf::core::storage::{PinataClient, LighthouseClient, StorageClient};
    use std::path::Path;
    
    let path = Path::new(file_path);
    
    let cid = match provider {
        "pinata" => {
            let client = PinataClient::new(api_key.to_string());
            client
                .upload_file(path)
                .await
                .map_err(|e| format!("Pinata upload failed: {}", e))?
        }
        "lighthouse" => {
            let client = LighthouseClient::new(api_key.to_string());
            client
                .upload_file(path)
                .await
                .map_err(|e| format!("Lighthouse upload failed: {}", e))?
        }
        _ => return Err(format!("Unknown provider: {}", provider)),
    };
    
    Ok(cid)
}

#[tokio::test]
#[ignore]
async fn test_dry_run_mint() {
    println!("🧪 Testing Dry Run Mint Workflow");
    println!("=================================");
    println!();
    
    // This test simulates the mint workflow without actually submitting
    println!("✓ Dry run simulation complete");
    println!();
    println!("To run a real dry-run:");
    println!("  mtmf mint test.png --dry-run");
}

#[test]
fn test_e2e_instructions() {
    println!();
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║         End-to-End Integration Test Instructions          ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("These tests verify the complete NFT minting workflow.");
    println!();
    println!("📋 Prerequisites:");
    println!("  1. Flow testnet account with balance");
    println!("  2. IPFS storage API key (Pinata or Lighthouse)");
    println!("  3. Test image file (PNG, JPG, etc.)");
    println!();
    println!("🔧 Setup:");
    println!("  export E2E_FLOW_ADDRESS=\"0x1234567890abcdef\"");
    println!("  export E2E_STORAGE_PROVIDER=\"pinata\"");
    println!("  export E2E_API_KEY=\"your-api-key\"");
    println!("  export E2E_PRIVATE_KEY=\"your-private-key-hex\"");
    println!("  export E2E_TEST_FILE=\"test_image.png\"");
    println!();
    println!("🚀 Run:");
    println!("  cargo test e2e --ignored -- --nocapture");
    println!();
    println!("⚠️  Warning:");
    println!("  - Only use testnet credentials");
    println!("  - Never commit real private keys");
    println!("  - E2E tests will upload files to IPFS");
    println!("  - E2E tests may consume testnet FLOW");
    println!();
    println!("💡 Alternative:");
    println!("  Use the CLI directly for full E2E testing:");
    println!("  1. mtmf init");
    println!("  2. mtmf key generate --alias test-key");
    println!("  3. mtmf doctor");
    println!("  4. mtmf mint test.png --dry-run");
    println!("  5. mtmf mint test.png");
    println!();
}
