use metis_contract::error::ErrorEnvelope;

use crate::{core::service_error::ServiceError, storage::StorageError};

#[derive(Debug)]
pub enum CommandError {
    NotImplemented(&'static str),
    Service(ServiceError),
    Storage(StorageError),
    Internal(String),
}

impl From<ServiceError> for CommandError {
    fn from(value: ServiceError) -> Self {
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
