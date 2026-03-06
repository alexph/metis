use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    pub id: String,
    pub task_id: String,
    pub worker_type: String,
    pub state: WorkerState,
    pub attempt: i64,
    pub last_heartbeat_at: Option<String>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WorkerState {
    Pending,
    Running,
    Completed,
    Failed,
    Stopped,
}
