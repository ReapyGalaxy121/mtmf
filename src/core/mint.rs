use crate::core::{cadence, config, flow, key, storage, utils};
use crate::errors::{MtmfError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MintResult {
    pub transaction_id: String,
    pub flowscan_link: String,
    pub ipfs_cid: String,
    pub nft_name: String,
}

pub async fn mint(
    file: &str,
    name: Option<String>,
    description: Option<String>,
    to: Option<String>,
    json: bool,
    dry_run: bool,
) -> Result<()> {
    // Validate file
    utils::validate_file(file).await?;

    // Load configuration
    let config = config::Config::load().await?;
    let profile = config.get_active_profile()?;

    // Interactive prompts for missing data
    let nft_name = if let Some(n) = name {
        n
    } else {
        prompt_input("NFT Name", None)?
    };

    let nft_description = if let Some(d) = description {
        d
    } else {
        prompt_input("NFT Description", Some("A unique NFT"))?
    };

    let recipient = if let Some(addr) = to {
        addr
    } else {
        profile
            .flow
            .address
            .clone()
            .ok_or_else(|| MtmfError::Config("No Flow address configured".to_string()))?
    };

    if !json {
        println!("\n🎨 Minting NFT");
        println!("  Name: {}", nft_name);
        println!("  Description: {}", nft_description);
        println!("  Recipient: {}", recipient);
        println!("  File: {}", file);
    }

    // Step 1: Upload to IPFS
    if !json {
        println!("\n📦 Step 1/4: Uploading to IPFS...");
    }
    let ipfs_cid = storage::upload(file, None).await?;
    let ipfs_url = format!("ipfs://{}", ipfs_cid);

    if !json {
        println!("  ✓ Uploaded: {}", ipfs_cid);
    }

    // Step 2: Load Cadence template
    if !json {
        println!("\n📝 Step 2/4: Preparing transaction...");
    }
    let script = cadence::load_template("mint")?;

    // Step 3: Build transaction
    if !json {
        println!("\n🔨 Step 3/4: Building transaction...");
    }

    // Connect to Flow
    let mut flow_client = flow::FlowClient::connect(&profile.flow.access_node).await?;

    // Get latest block for reference
    let latest_block = flow_client.get_latest_block().await?;
    let reference_block_id = latest_block.id.clone();

    // Get signer address
    let signer_address = profile
        .flow
        .address
        .as_ref()
        .ok_or_else(|| MtmfError::Config("No Flow address configured".to_string()))?;
    let signer_address_bytes = flow::parse_address(signer_address)?;

    // Get account to get sequence number
    let account = flow_client.get_account(signer_address).await?;
    let key_index = 0u32; // Assuming first key
    let sequence_number = account
        .keys
        .get(key_index as usize)
        .map(|k| k.sequence_number as u64)
        .unwrap_or(0u64);

    // Encode transaction arguments (recipient, name, description, thumbnail, ipfsHash)
    let args = vec![
        cadence::encode_address(&recipient)?,
        cadence::encode_string(&nft_name)?,
        cadence::encode_string(&nft_description)?,
        cadence::encode_string(&ipfs_url)?,
        cadence::encode_string(&ipfs_cid)?,
    ];

    // Build transaction
    let mut transaction = flow::TransactionBuilder::new(&script)
        .reference_block_id(reference_block_id.clone())
        .gas_limit(9999)
        .proposal_key(
            signer_address_bytes.clone(),
            key_index,
            sequence_number,
        )
        .payer(signer_address_bytes.clone())
        .add_authorizer(signer_address_bytes.clone())
        .build();

    // Add arguments
    transaction.arguments = args;

    if dry_run {
        if json {
            let result = serde_json::json!({
                "dry_run": true,
                "ipfs_cid": ipfs_cid,
                "transaction": {
                    "script_length": script.len(),
                    "gas_limit": transaction.gas_limit,
                    "reference_block": hex::encode(&reference_block_id),
                }
            });
            println!("{}", serde_json::to_string_pretty(&result)?);
        } else {
            println!("\n✓ Dry run complete");
            println!("  IPFS CID: {}", ipfs_cid);
            println!("  Transaction ready to submit");
        }
        return Ok(());
    }

    // Step 4: Sign and submit transaction
    if !json {
        println!("\n🔐 Step 4/4: Signing and submitting...");
    }

    // Get key alias from config
    let key_alias = profile
        .flow
        .key_alias
        .as_ref()
        .ok_or_else(|| MtmfError::Config("No key alias configured".to_string()))?;

    // Prompt for passphrase
    let passphrase = if !json {
        rpassword::prompt_password("Enter passphrase to unlock key: ")?
    } else {
        return Err(MtmfError::Auth(
            "Cannot prompt for passphrase in JSON mode. Use interactive mode.".to_string(),
        ));
    };

    // Load signing key
    let signing_key = key::load_signing_key(key_alias, &passphrase).await?;

    // Sign transaction
    flow::sign_transaction(&mut transaction, &signing_key, &signer_address_bytes, key_index)?;

    // Submit transaction
    let tx_id = flow_client.send_transaction(transaction).await?;

    if !json {
        println!("  ✓ Transaction submitted: {}", hex::encode(&tx_id));
        println!("\n⏳ Waiting for transaction to seal...");
    }

    // Wait for seal
    let result = flow_client.wait_for_seal(&tx_id, 30).await?;

    // Check for errors
    if result.status_code != 0 || !result.error_message.is_empty() {
        return Err(MtmfError::Transaction(format!(
            "Transaction failed: {}",
            result.error_message
        )));
    }

    // Generate Flowscan link
    let is_mainnet = matches!(profile.network, config::Network::Mainnet);
    let flowscan_link = flow::flowscan_link(&tx_id, is_mainnet);

    let mint_result = MintResult {
        transaction_id: hex::encode(&tx_id),
        flowscan_link: flowscan_link.clone(),
        ipfs_cid: ipfs_cid.clone(),
        nft_name: nft_name.clone(),
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&mint_result)?);
    } else {
        println!("\n🎉 NFT Minted Successfully!");
        println!("  Name: {}", nft_name);
        println!("  IPFS: {}", ipfs_cid);
        println!("  Transaction: {}", hex::encode(&tx_id));
        println!("  Flowscan: {}", flowscan_link);
    }

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
            Err(MtmfError::InvalidInput(format!(
                "{} is required",
                prompt
            )))
        }
    } else {
        Ok(input.to_string())
    }
}
