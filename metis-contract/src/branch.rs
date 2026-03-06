use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub id: String,
    pub channel_id: String,
    pub parent_branch_id: Option<String>,
    pub name: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}
