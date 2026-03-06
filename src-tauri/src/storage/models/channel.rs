#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ChannelRecord {
    pub id: String,
    pub title: String,
    pub source_type: String,
    pub source_ref: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}
