use crate::errors::{MtmfError, Result};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use directories::ProjectDirs;
use p256::ecdsa::{SigningKey, VerifyingKey};
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::path::PathBuf;
use tokio::fs;
use zeroize::Zeroize;

const PBKDF2_ITERATIONS: u32 = 600_000;
const SALT_LEN: usize = 32;
const NONCE_LEN: usize = 12;

#[derive(Debug, Serialize, Deserialize)]
struct EncryptedKey {
    alias: String,
    salt: Vec<u8>,
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
    public_key: Vec<u8>,
}

fn keys_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "mtmf", "mtmf")
        .ok_or_else(|| MtmfError::Key("Could not determine keys directory".to_string()))?;
    Ok(proj_dirs.data_dir().join("keys"))
}

fn key_path(alias: &str) -> Result<PathBuf> {
    Ok(keys_dir()?.join(format!("{}.key", alias)))
}

pub async fn generate(alias: &str) -> Result<()> {
    // Check if key already exists
    let path = key_path(alias)?;
    if path.exists() {
        return Err(MtmfError::Key(format!(
            "Key '{}' already exists. Use a different alias or delete the existing key.",
            alias
        )));
    }

    // Generate ECDSA P-256 key pair
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);

    // Extract private key bytes
    let private_key_bytes = signing_key.to_bytes();

    // Extract public key in Flow format (64 bytes: X||Y)
    let public_key_point = verifying_key.to_encoded_point(false);
    let public_key_bytes = public_key_point.as_bytes();
    // Skip the first byte (0x04 uncompressed marker) to get X||Y
    let public_key_flow = &public_key_bytes[1..];

    println!("🔑 Generated ECDSA P-256 key pair");
    println!("Public key (Flow format): {}", hex::encode(public_key_flow));

    // Prompt for passphrase
    let passphrase = rpassword::prompt_password("Enter passphrase to encrypt key: ")?;
    if passphrase.is_empty() {
        return Err(MtmfError::Key("Passphrase cannot be empty".to_string()));
    }

    let passphrase_confirm = rpassword::prompt_password("Confirm passphrase: ")?;
    if passphrase != passphrase_confirm {
        return Err(MtmfError::Key("Passphrases do not match".to_string()));
    }

    // Encrypt and save
    encrypt_and_save(alias, &private_key_bytes, public_key_flow, &passphrase).await?;

    // Zeroize sensitive data
    drop(signing_key);
    drop(passphrase);
    drop(passphrase_confirm);

    tracing::info!("Key pair generated and saved: {}", alias);
    Ok(())
}

async fn encrypt_and_save(
    alias: &str,
    private_key: &[u8],
    public_key: &[u8],
    passphrase: &str,
) -> Result<()> {
    // Generate random salt
    let mut salt = vec![0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);

    // Derive encryption key from passphrase using PBKDF2
    let mut encryption_key = [0u8; 32];
    pbkdf2_hmac::<sha3::Sha3_256>(
        passphrase.as_bytes(),
        &salt,
        PBKDF2_ITERATIONS,
        &mut encryption_key,
    );

    // Generate random nonce
    let mut nonce_bytes = vec![0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt private key
    let cipher = Aes256Gcm::new_from_slice(&encryption_key)
        .map_err(|e| MtmfError::Crypto(format!("Failed to create cipher: {}", e)))?;

    let ciphertext = cipher
        .encrypt(nonce, private_key)
        .map_err(|e| MtmfError::Crypto(format!("Encryption failed: {}", e)))?;

    // Create encrypted key structure
    let encrypted_key = EncryptedKey {
        alias: alias.to_string(),
        salt,
        nonce: nonce_bytes,
        ciphertext,
        public_key: public_key.to_vec(),
    };

    // Save to file
    let path = key_path(alias)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }

    let json = serde_json::to_string_pretty(&encrypted_key)?;
    fs::write(&path, json).await?;

    // Zeroize sensitive data
    encryption_key.zeroize();

    tracing::info!("Encrypted key saved to: {}", path.display());
    Ok(())
}

pub async fn import(alias: &str, file: &str) -> Result<()> {
    // Check if key already exists
    let dest_path = key_path(alias)?;
    if dest_path.exists() {
        return Err(MtmfError::Key(format!(
            "Key '{}' already exists. Use a different alias.",
            alias
        )));
    }

    // Read the private key from file (expecting hex-encoded private key)
    let content = fs::read_to_string(file).await?;
    let content = content.trim();

    // Try to decode as hex
    let private_key_bytes = hex::decode(content)
        .map_err(|_| MtmfError::Key("Invalid key format. Expected hex-encoded private key.".to_string()))?;

    if private_key_bytes.len() != 32 {
        return Err(MtmfError::Key(
            "Invalid key length. Expected 32 bytes for P-256 private key.".to_string(),
        ));
    }

    // Create signing key from bytes
    let signing_key = SigningKey::from_slice(&private_key_bytes)
        .map_err(|e| MtmfError::Crypto(format!("Invalid private key: {}", e)))?;

    let verifying_key = VerifyingKey::from(&signing_key);

    // Extract public key in Flow format
    let public_key_point = verifying_key.to_encoded_point(false);
    let public_key_bytes = public_key_point.as_bytes();
    let public_key_flow = &public_key_bytes[1..];

    println!("🔑 Imported ECDSA P-256 key");
    println!("Public key (Flow format): {}", hex::encode(public_key_flow));

    // Prompt for passphrase
    let passphrase = rpassword::prompt_password("Enter passphrase to encrypt key: ")?;
    if passphrase.is_empty() {
        return Err(MtmfError::Key("Passphrase cannot be empty".to_string()));
    }

    // Encrypt and save
    encrypt_and_save(alias, &private_key_bytes, public_key_flow, &passphrase).await?;

    tracing::info!("Key imported: {}", alias);
    Ok(())
}

pub async fn export(alias: &str, output: Option<&str>) -> Result<()> {
    let path = key_path(alias)?;
    if !path.exists() {
        return Err(MtmfError::Key(format!("Key '{}' not found", alias)));
    }

    // Load encrypted key
    let content = fs::read_to_string(&path).await?;
    let encrypted_key: EncryptedKey = serde_json::from_str(&content)?;

    // Prompt for passphrase
    let passphrase = rpassword::prompt_password("Enter passphrase to decrypt key: ")?;

    // Decrypt private key
    let private_key = decrypt_key(&encrypted_key, &passphrase)?;

    // Export to file or stdout
    let hex_key = hex::encode(&private_key);

    if let Some(output_path) = output {
        fs::write(output_path, &hex_key).await?;
        println!("✓ Key exported to: {}", output_path);
    } else {
        println!("Private key (hex): {}", hex_key);
        println!("\n⚠️  Keep this private key secure!");
    }

    Ok(())
}

fn decrypt_key(encrypted_key: &EncryptedKey, passphrase: &str) -> Result<Vec<u8>> {
    // Derive encryption key from passphrase
    let mut encryption_key = [0u8; 32];
    pbkdf2_hmac::<sha3::Sha3_256>(
        passphrase.as_bytes(),
        &encrypted_key.salt,
        PBKDF2_ITERATIONS,
        &mut encryption_key,
    );

    // Decrypt private key
    let cipher = Aes256Gcm::new_from_slice(&encryption_key)
        .map_err(|e| MtmfError::Crypto(format!("Failed to create cipher: {}", e)))?;

    let nonce = Nonce::from_slice(&encrypted_key.nonce);

    let plaintext = cipher
        .decrypt(nonce, encrypted_key.ciphertext.as_ref())
        .map_err(|_| MtmfError::Crypto("Decryption failed. Incorrect passphrase?".to_string()))?;

    // Zeroize encryption key
    encryption_key.zeroize();

    Ok(plaintext)
}

pub async fn list() -> Result<Vec<String>> {
    let keys_dir = keys_dir()?;

    if !keys_dir.exists() {
        return Ok(vec![]);
    }

    let mut entries = fs::read_dir(&keys_dir).await?;
    let mut keys = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("key") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                // Load key to get public key
                if let Ok(content) = fs::read_to_string(&path).await {
                    if let Ok(encrypted_key) = serde_json::from_str::<EncryptedKey>(&content) {
                        let pub_key_hex = hex::encode(&encrypted_key.public_key);
                        keys.push(format!("{} ({}...)", stem, &pub_key_hex[..16]));
                    }
                }
            }
        }
    }

    Ok(keys)
}

/// Load and decrypt a key for signing
pub async fn load_signing_key(alias: &str, passphrase: &str) -> Result<SigningKey> {
    let path = key_path(alias)?;
    if !path.exists() {
        return Err(MtmfError::Key(format!("Key '{}' not found", alias)));
    }

    let content = fs::read_to_string(&path).await?;
    let encrypted_key: EncryptedKey = serde_json::from_str(&content)?;

    let private_key_bytes = decrypt_key(&encrypted_key, passphrase)?;

    let signing_key = SigningKey::from_slice(&private_key_bytes)
        .map_err(|e| MtmfError::Crypto(format!("Invalid private key: {}", e)))?;

    Ok(signing_key)
}

/// Sign a message with SHA3-256 and return the signature
pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Result<Vec<u8>> {
    use p256::ecdsa::signature::Signer;

    // Hash message with SHA3-256
    let mut hasher = Sha3_256::new();
    hasher.update(message);
    let hash = hasher.finalize();

    // Sign the hash
    let signature: p256::ecdsa::Signature = signing_key.sign(&hash);

    Ok(signature.to_vec())
}
