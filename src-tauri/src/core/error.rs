use metis_contract::ErrorEnvelope;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetisError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("storage error: {0}")]
    Storage(#[from] crate::storage::StorageError),
}

impl MetisError {
    pub fn to_envelope(&self) -> ErrorEnvelope {
        match self {
            Self::Config(message) => ErrorEnvelope {
                code: "config_error".to_string(),
                message: message.clone(),
                details: None,
            },
            Self::Storage(error) => error.to_envelope(),
        }
    }
}
