use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub channel_id: String,
    pub branch_id: Option<String>,
    pub kind: String,
    pub state: TaskState,
    pub priority: i64,
    pub payload_json: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TaskState {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}
