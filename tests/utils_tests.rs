use mtmf::core::utils;
use tempfile::NamedTempFile;
use std::io::Write;

#[tokio::test]
async fn test_validate_file_exists() {
    let mut temp_file = NamedTempFile::new().unwrap();
    write!(temp_file, "test content").unwrap();
    
    let path = temp_file.path().to_str().unwrap();
    let result = utils::validate_file(path).await;
    
    assert!(result.is_ok(), "File validation should succeed for existing file");
}

#[tokio::test]
async fn test_validate_file_not_exists() {
    let result = utils::validate_file("/nonexistent/file.txt").await;
    
    assert!(result.is_err(), "File validation should fail for non-existent file");
}

#[tokio::test]
async fn test_validate_file_is_directory() {
    let temp_dir = tempfile::tempdir().unwrap();
    let path = temp_dir.path().to_str().unwrap();
    
    let result = utils::validate_file(path).await;
    
    assert!(result.is_err(), "File validation should fail for directory");
}

#[test]
fn test_format_bytes() {
    assert_eq!(utils::format_bytes(0), "0.00 B");
    assert_eq!(utils::format_bytes(512), "512.00 B");
    assert_eq!(utils::format_bytes(1024), "1.00 KB");
    assert_eq!(utils::format_bytes(1536), "1.50 KB");
    assert_eq!(utils::format_bytes(1048576), "1.00 MB");
    assert_eq!(utils::format_bytes(1073741824), "1.00 GB");
}

#[tokio::test]
async fn test_hash_file() {
    let mut temp_file = NamedTempFile::new().unwrap();
    write!(temp_file, "test content").unwrap();
    
    let path = temp_file.path();
    let hash = utils::hash_file(path).await;
    
    assert!(hash.is_ok(), "Hash calculation should succeed");
    
    let hash_value = hash.unwrap();
    assert_eq!(hash_value.len(), 64, "SHA3-256 hash should be 64 hex characters");
    
    // Hash should be deterministic
    let hash2 = utils::hash_file(path).await.unwrap();
    assert_eq!(hash_value, hash2, "Hash should be deterministic");
}
