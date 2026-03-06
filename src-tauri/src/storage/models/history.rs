#[derive(Debug, Clone)]
pub struct HistoryRecord {
    pub id: String,
    pub channel_id: String,
    pub branch_id: Option<String>,
    pub task_id: Option<String>,
    pub worker_id: Option<String>,
    pub event_type: String,
    pub role: Option<String>,
    pub content_json: String,
    pub correlation_id: Option<String>,
    pub created_at: String,
}
