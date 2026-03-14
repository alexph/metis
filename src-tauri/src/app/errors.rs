use metis_contract::error::ErrorEnvelope;

use crate::storage::StorageError;

#[derive(Debug, Clone)]
pub struct Error {
    pub code: &'static str,
    pub message: String,
    pub details: Option<String>,
}

impl Error {
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

impl From<StorageError> for Error {
    fn from(value: StorageError) -> Self {
        let envelope = value.to_envelope();
        Self {
            code: "service_storage_error",
            message: envelope.message,
            details: envelope.details,
        }
    }
}

#[derive(Debug)]
pub enum CommandError {
    NotImplemented(&'static str),
    Service(Error),
    Storage(StorageError),
    Internal(String),
}

impl From<Error> for CommandError {
    fn from(value: Error) -> Self {
        Self::Service(value)
    }
}

impl From<StorageError> for CommandError {
    fn from(value: StorageError) -> Self {
        Self::Storage(value)
    }
}

impl CommandError {
    pub fn to_envelope(&self) -> ErrorEnvelope {
        match self {
            Self::NotImplemented(operation) => ErrorEnvelope {
                code: "desktop_not_implemented".to_string(),
                message: format!("desktop command is not implemented: {operation}"),
                details: None,
            },
            Self::Service(error) => error.to_envelope(),
            Self::Storage(error) => error.to_envelope(),
            Self::Internal(message) => ErrorEnvelope {
                code: "desktop_internal_error".to_string(),
                message: "desktop command internal error".to_string(),
                details: Some(message.clone()),
            },
        }
    }
}
