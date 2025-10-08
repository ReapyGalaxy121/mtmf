use crate::errors::{MtmfError, Result};

pub fn load_template(name: &str) -> Result<String> {
    match name {
        "mint" => Ok(include_str!("../../templates/mint.cdc").to_string()),
        _ => Err(MtmfError::InvalidInput(format!(
            "Unknown template: {}",
            name
        ))),
    }
}

/// Encode a Cadence argument as JSON-CDC format
pub fn encode_argument(value: &serde_json::Value) -> Result<Vec<u8>> {
    let json_cdc = serde_json::json!({
        "type": infer_cadence_type(value),
        "value": value
    });
    
    serde_json::to_vec(&json_cdc)
        .map_err(|e| MtmfError::Serialization(format!("Failed to encode argument: {}", e)))
}

fn infer_cadence_type(value: &serde_json::Value) -> &str {
    match value {
        serde_json::Value::String(_) => "String",
        serde_json::Value::Number(_) => "UInt64",
        serde_json::Value::Bool(_) => "Bool",
        serde_json::Value::Array(_) => "Array",
        serde_json::Value::Object(_) => "Dictionary",
        serde_json::Value::Null => "Optional",
    }
}

/// Encode a Flow address as JSON-CDC format
pub fn encode_address(address: &str) -> Result<Vec<u8>> {
    let address = address.strip_prefix("0x").unwrap_or(address);
    let json_cdc = serde_json::json!({
        "type": "Address",
        "value": format!("0x{}", address)
    });
    
    serde_json::to_vec(&json_cdc)
        .map_err(|e| MtmfError::Serialization(format!("Failed to encode address: {}", e)))
}

/// Encode a string as JSON-CDC format
pub fn encode_string(value: &str) -> Result<Vec<u8>> {
    let json_cdc = serde_json::json!({
        "type": "String",
        "value": value
    });
    
    serde_json::to_vec(&json_cdc)
        .map_err(|e| MtmfError::Serialization(format!("Failed to encode string: {}", e)))
}
