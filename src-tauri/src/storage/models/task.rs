#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TaskRecord {
    pub id: String,
    pub channel_id: String,
    pub branch_id: Option<String>,
    pub kind: String,
    pub state: String,
    pub priority: i64,
    pub payload_json: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
}
