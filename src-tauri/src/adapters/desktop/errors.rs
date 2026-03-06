use metis_contract::error::ErrorEnvelope;

#[derive(Debug, Clone)]
pub enum DesktopAdapterError {
    NotImplemented(&'static str),
    Internal(String),
}

impl DesktopAdapterError {
    pub fn to_envelope(&self) -> ErrorEnvelope {
        match self {
            Self::NotImplemented(operation) => ErrorEnvelope {
                code: "desktop_not_implemented".to_string(),
                message: format!("desktop adapter operation is not implemented: {operation}"),
                details: None,
            },
            Self::Internal(message) => ErrorEnvelope {
                code: "desktop_internal_error".to_string(),
                message: "desktop adapter internal error".to_string(),
                details: Some(message.clone()),
            },
        }
    }
}
