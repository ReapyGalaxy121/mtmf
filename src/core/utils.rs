use crate::errors::{MtmfError, Result};
use sha3::{Digest, Sha3_256};
use std::path::Path;
use tokio::fs;

/// Calculate SHA-256 hash of a file
pub async fn hash_file(path: &Path) -> Result<String> {
    let content = fs::read(path).await?;
    let mut hasher = Sha3_256::new();
    hasher.update(&content);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

/// Validate that a file exists and is readable
pub async fn validate_file(path: &str) -> Result<()> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(MtmfError::FileNotFound(path.display().to_string()));
    }

    if !path.is_file() {
        return Err(MtmfError::InvalidInput(format!(
            "{} is not a file",
            path.display()
        )));
    }

    Ok(())
}

/// Format bytes as human-readable size
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_idx])
}
