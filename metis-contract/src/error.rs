use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEnvelope {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}
