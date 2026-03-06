use metis_contract::error::ErrorEnvelope;

use crate::storage::StorageError;

#[derive(Debug, Clone)]
pub struct ServiceError {
    pub code: &'static str,
    pub message: String,
    pub details: Option<String>,
}

impl ServiceError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self {
            code: "service_validation_error",
            message: message.into(),
            details: None,
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            code: "service_not_found",
            message: message.into(),
            details: None,
        }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self {
            code: "service_conflict",
            message: message.into(),
            details: None,
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            code: "service_internal",
            message: message.into(),
            details: None,
        }
    }

    pub fn to_envelope(&self) -> ErrorEnvelope {
        ErrorEnvelope {
            code: self.code.to_string(),
            message: self.message.clone(),
            details: self.details.clone(),
        }
    }
}

impl From<StorageError> for ServiceError {
    fn from(value: StorageError) -> Self {
        let envelope = value.to_envelope();
        Self {
            code: "service_storage_error",
            message: envelope.message,
            details: envelope.details,
        }
    }
}
