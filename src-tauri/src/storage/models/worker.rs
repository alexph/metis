#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WorkerRecord {
    pub id: String,
    pub task_id: String,
    pub worker_type: String,
    pub state: String,
    pub attempt: i64,
    pub last_heartbeat_at: Option<String>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
