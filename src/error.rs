use thiserror::Error;

/// Custom error types for XRPL operations
#[derive(Error, Debug)]
pub enum XrplError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Invalid secret key: {0}")]
    InvalidSecret(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Invalid transaction data: {0}")]
    InvalidTransaction(String),

    #[error("Signing failed: {0}")]
    SigningFailed(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("XRPL API error: {0}")]
    ApiError(String),

    #[error("Invalid currency code: {0}")]
    InvalidCurrency(String),

    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),

    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
}

impl From<reqwest::Error> for XrplError {
    fn from(err: reqwest::Error) -> Self {
        XrplError::Network(err.to_string())
    }
}

impl From<serde_json::Error> for XrplError {
    fn from(err: serde_json::Error) -> Self {
        XrplError::Serialization(err.to_string())
    }
}

impl From<hex::FromHexError> for XrplError {
    fn from(err: hex::FromHexError) -> Self {
        XrplError::Serialization(err.to_string())
    }
}

impl From<ed25519_dalek::ed25519::Error> for XrplError {
    fn from(err: ed25519_dalek::ed25519::Error) -> Self {
        XrplError::SigningFailed(err.to_string())
    }
}
