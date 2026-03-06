use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEvent {
    pub id: String,
    pub channel_id: String,
    pub branch_id: Option<String>,
    pub task_id: Option<String>,
    pub worker_id: Option<String>,
    pub event_type: String,
    pub role: Option<HistoryRole>,
    pub content_json: String,
    pub correlation_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HistoryRole {
    System,
    User,
    Assistant,
    Tool,
}
