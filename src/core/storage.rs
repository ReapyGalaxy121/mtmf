use crate::core::config::{Config, StorageProvider};
use crate::core::utils;
use crate::errors::{MtmfError, Result};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::path::Path;
use std::time::Duration;
use tokio::fs;

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000;

#[derive(Debug, Serialize, Deserialize)]
struct PinataResponse {
    #[serde(rename = "IpfsHash")]
    ipfs_hash: String,
    #[serde(rename = "PinSize")]
    pin_size: u64,
    #[serde(rename = "Timestamp")]
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LighthouseResponse {
    #[serde(rename = "Hash")]
    hash: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Size")]
    size: String,
}

pub trait StorageClient: Send + Sync {
    fn upload_file(&self, file_path: &Path) -> impl Future<Output = Result<String>> + Send;
}

pub struct PinataClient {
    api_key: String,
    client: reqwest::Client,
}

impl PinataClient {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .unwrap();

        Self { api_key, client }
    }
}

impl StorageClient for PinataClient {
    async fn upload_file(&self, file_path: &Path) -> Result<String> {
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| MtmfError::Storage("Invalid file name".to_string()))?
            .to_string();

        let file_content = fs::read(file_path).await?;
        let file_size = file_content.len();

        tracing::info!("Uploading {} ({}) to Pinata...", file_name, utils::format_bytes(file_size as u64));

        // Upload with retry logic
        for attempt in 1..=MAX_RETRIES {
            // Create multipart form for each attempt
            let part = multipart::Part::bytes(file_content.clone())
                .file_name(file_name.clone())
                .mime_str("application/octet-stream")
                .map_err(|e| MtmfError::Storage(format!("Failed to create multipart: {}", e)))?;

            let form = multipart::Form::new().part("file", part);

            match self
                .client
                .post("https://api.pinata.cloud/pinning/pinFileToIPFS")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .multipart(form)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        let result: PinataResponse = response.json().await?;
                        tracing::info!("✓ Uploaded to IPFS: {}", result.ipfs_hash);
                        return Ok(result.ipfs_hash);
                    } else {
                        let status = response.status();
                        let error_text = response.text().await.unwrap_or_default();
                        tracing::warn!("Upload attempt {} failed: {} - {}", attempt, status, error_text);

                        if attempt < MAX_RETRIES {
                            tokio::time::sleep(Duration::from_millis(RETRY_DELAY_MS * attempt as u64)).await;
                        } else {
                            return Err(MtmfError::Storage(format!(
                                "Upload failed after {} attempts: {} - {}",
                                MAX_RETRIES, status, error_text
                            )));
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Upload attempt {} failed: {}", attempt, e);
                    if attempt < MAX_RETRIES {
                        tokio::time::sleep(Duration::from_millis(RETRY_DELAY_MS * attempt as u64)).await;
                    } else {
                        return Err(MtmfError::Storage(format!(
                            "Upload failed after {} attempts: {}",
                            MAX_RETRIES, e
                        )));
                    }
                }
            }
        }

        Err(MtmfError::Storage("Upload failed".to_string()))
    }
}

pub struct LighthouseClient {
    api_key: String,
    client: reqwest::Client,
}

impl LighthouseClient {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .unwrap();

        Self { api_key, client }
    }
}

impl StorageClient for LighthouseClient {
    async fn upload_file(&self, file_path: &Path) -> Result<String> {
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| MtmfError::Storage("Invalid file name".to_string()))?
            .to_string();

        let file_content = fs::read(file_path).await?;
        let file_size = file_content.len();

        tracing::info!("Uploading {} ({}) to Lighthouse...", file_name, utils::format_bytes(file_size as u64));

        // Upload with retry logic
        for attempt in 1..=MAX_RETRIES {
            // Create multipart form for each attempt
            let part = multipart::Part::bytes(file_content.clone())
                .file_name(file_name.clone())
                .mime_str("application/octet-stream")
                .map_err(|e| MtmfError::Storage(format!("Failed to create multipart: {}", e)))?;

            let form = multipart::Form::new().part("file", part);

            match self
                .client
                .post("https://node.lighthouse.storage/api/v0/add")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .multipart(form)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        let result: LighthouseResponse = response.json().await?;
                        tracing::info!("✓ Uploaded to IPFS: {}", result.hash);
                        return Ok(result.hash);
                    } else {
                        let status = response.status();
                        let error_text = response.text().await.unwrap_or_default();
                        tracing::warn!("Upload attempt {} failed: {} - {}", attempt, status, error_text);

                        if attempt < MAX_RETRIES {
                            tokio::time::sleep(Duration::from_millis(RETRY_DELAY_MS * attempt as u64)).await;
                        } else {
                            return Err(MtmfError::Storage(format!(
                                "Upload failed after {} attempts: {} - {}",
                                MAX_RETRIES, status, error_text
                            )));
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Upload attempt {} failed: {}", attempt, e);
                    if attempt < MAX_RETRIES {
                        tokio::time::sleep(Duration::from_millis(RETRY_DELAY_MS * attempt as u64)).await;
                    } else {
                        return Err(MtmfError::Storage(format!(
                            "Upload failed after {} attempts: {}",
                            MAX_RETRIES, e
                        )));
                    }
                }
            }
        }

        Err(MtmfError::Storage("Upload failed".to_string()))
    }
}

pub async fn upload(file: &str, storage: Option<&str>) -> Result<String> {
    // Validate file exists
    utils::validate_file(file).await?;
    let file_path = Path::new(file);

    // Load config to get storage provider
    let config = Config::load().await?;
    let profile = config.get_active_profile()?;

    // Determine which storage provider to use
    let provider = if let Some(storage_name) = storage {
        match storage_name.to_lowercase().as_str() {
            "pinata" => StorageProvider::Pinata,
            "lighthouse" => StorageProvider::Lighthouse,
            _ => {
                return Err(MtmfError::Storage(format!(
                    "Unknown storage provider: {}. Supported: pinata, lighthouse",
                    storage_name
                )))
            }
        }
    } else {
        profile.storage.provider
    };

    // Validate supported provider
    match provider {
        StorageProvider::Pinata | StorageProvider::Lighthouse => {},
        StorageProvider::Arweave => {
            return Err(MtmfError::Storage(
                "Arweave is not currently supported. Please use Pinata or Lighthouse.".to_string()
            ));
        }
    }

    // Get API key
    let api_key = profile.storage.api_key.clone().ok_or_else(|| {
        MtmfError::Storage(format!(
            "No API key configured for {:?}. Run 'mtmf init' or set it in config.",
            provider
        ))
    })?;

    // Create client and upload
    let cid = match provider {
        StorageProvider::Pinata => {
            let client = PinataClient::new(api_key);
            client.upload_file(file_path).await?
        }
        StorageProvider::Lighthouse => {
            let client = LighthouseClient::new(api_key);
            client.upload_file(file_path).await?
        }
        StorageProvider::Arweave => {
            unreachable!("Arweave validation should have caught this earlier")
        }
    };

    Ok(cid)
}
