use mtmf::core::cadence;

#[test]
fn test_load_template_mint() {
    let result = cadence::load_template("mint");
    assert!(result.is_ok(), "Should load mint template");
    
    let template = result.unwrap();
    assert!(template.contains("transaction"), "Template should contain transaction keyword");
    assert!(template.contains("Cadence 1.0"), "Template should be Cadence 1.0");
    assert!(template.len() > 0, "Template should not be empty");
}

#[test]
fn test_load_template_unknown() {
    let result = cadence::load_template("unknown");
    assert!(result.is_err(), "Should fail for unknown template");
}

#[test]
fn test_encode_string() {
    let result = cadence::encode_string("Hello World");
    assert!(result.is_ok());
    
    let encoded = result.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
    
    assert_eq!(json["type"], "String");
    assert_eq!(json["value"], "Hello World");
}

#[test]
fn test_encode_address() {
    let result = cadence::encode_address("0x1234567890abcdef");
    assert!(result.is_ok());
    
    let encoded = result.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
    
    assert_eq!(json["type"], "Address");
    assert_eq!(json["value"], "0x1234567890abcdef");
}

#[test]
fn test_encode_address_without_prefix() {
    let result = cadence::encode_address("1234567890abcdef");
    assert!(result.is_ok());
    
    let encoded = result.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
    
    assert_eq!(json["type"], "Address");
    assert_eq!(json["value"], "0x1234567890abcdef");
}

#[test]
fn test_encode_argument_string() {
    let value = serde_json::json!("test");
    let result = cadence::encode_argument(&value);
    assert!(result.is_ok());
    
    let encoded = result.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
    
    assert_eq!(json["type"], "String");
    assert_eq!(json["value"], "test");
}

#[test]
fn test_encode_argument_number() {
    let value = serde_json::json!(42);
    let result = cadence::encode_argument(&value);
    assert!(result.is_ok());
    
    let encoded = result.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
    
    assert_eq!(json["type"], "UInt64");
    assert_eq!(json["value"], 42);
}

#[test]
fn test_encode_argument_bool() {
    let value = serde_json::json!(true);
    let result = cadence::encode_argument(&value);
    assert!(result.is_ok());
    
    let encoded = result.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
    
    assert_eq!(json["type"], "Bool");
    assert_eq!(json["value"], true);
}
