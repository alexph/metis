#[derive(Debug, Clone)]
pub struct MetadataRecord {
    pub id: String,
    pub instance_id: String,
    pub schema_version: i64,
    pub created_at: String,
    pub updated_at: String,
}
