use thiserror::Error;

pub type Result<T> = std::result::Result<T, MtmfError>;

#[derive(Error, Debug)]
pub enum MtmfError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Key management error: {0}")]
    Key(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Flow blockchain error: {0}")]
    Flow(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Cryptography error: {0}")]
    Crypto(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<toml::de::Error> for MtmfError {
    fn from(err: toml::de::Error) -> Self {
        MtmfError::Serialization(err.to_string())
    }
}

impl From<toml::ser::Error> for MtmfError {
    fn from(err: toml::ser::Error) -> Self {
        MtmfError::Serialization(err.to_string())
    }
}

impl From<serde_json::Error> for MtmfError {
    fn from(err: serde_json::Error) -> Self {
        MtmfError::Serialization(err.to_string())
    }
}

impl From<reqwest::Error> for MtmfError {
    fn from(err: reqwest::Error) -> Self {
        MtmfError::Network(err.to_string())
    }
}

impl From<hex::FromHexError> for MtmfError {
    fn from(err: hex::FromHexError) -> Self {
        MtmfError::Crypto(format!("Hex decoding error: {}", err))
    }
}
