use metis_contract::error::ErrorEnvelope;

use crate::storage::StorageError;

#[derive(Debug)]
pub enum DesktopAdapterError {
    NotImplemented(&'static str),
    Storage(StorageError),
    Internal(String),
}

impl From<StorageError> for DesktopAdapterError {
    fn from(value: StorageError) -> Self {
        Self::Storage(value)
    }
}

impl DesktopAdapterError {
    pub fn to_envelope(&self) -> ErrorEnvelope {
        match self {
            Self::NotImplemented(operation) => ErrorEnvelope {
                code: "desktop_not_implemented".to_string(),
                message: format!("desktop adapter operation is not implemented: {operation}"),
                details: None,
            },
            Self::Storage(error) => error.to_envelope(),
            Self::Internal(message) => ErrorEnvelope {
                code: "desktop_internal_error".to_string(),
                message: "desktop adapter internal error".to_string(),
                details: Some(message.clone()),
            },
        }
    }
}
