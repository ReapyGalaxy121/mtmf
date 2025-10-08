use mtmf::core::key;
use p256::ecdsa::SigningKey;
use tempfile::TempDir;
use std::env;

#[test]
fn test_sign_message() {
    let signing_key = SigningKey::random(&mut rand::thread_rng());
    let message = b"Hello, Flow!";
    
    let result = key::sign_message(&signing_key, message);
    assert!(result.is_ok(), "Signing should succeed");
    
    let signature = result.unwrap();
    assert!(signature.len() > 0, "Signature should not be empty");
    
    // Signature should be deterministic for same message and key
    let signature2 = key::sign_message(&signing_key, message).unwrap();
    assert_eq!(signature, signature2, "Signatures should be deterministic");
}

#[test]
fn test_sign_different_messages() {
    let signing_key = SigningKey::random(&mut rand::thread_rng());
    
    let sig1 = key::sign_message(&signing_key, b"message1").unwrap();
    let sig2 = key::sign_message(&signing_key, b"message2").unwrap();
    
    assert_ne!(sig1, sig2, "Different messages should produce different signatures");
}

#[tokio::test]
async fn test_key_list_empty() {
    let temp_dir = TempDir::new().unwrap();
    env::set_var("HOME", temp_dir.path());
    
    let result = key::list().await;
    assert!(result.is_ok());
    
    let keys = result.unwrap();
    assert_eq!(keys.len(), 0, "Should have no keys initially");
}

// Note: Full key generation tests require interactive password input
// These would be better suited for integration tests with mocked input
