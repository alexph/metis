use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub title: String,
    pub source_type: ChannelSourceType,
    pub source_ref: Option<String>,
    pub status: ChannelStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelSourceType {
    Manual,
    External,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelStatus {
    Active,
    Archived,
    Suspended,
}
