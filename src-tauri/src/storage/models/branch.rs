#[derive(Debug, Clone)]
pub struct BranchRecord {
    pub id: String,
    pub channel_id: String,
    pub parent_branch_id: Option<String>,
    pub name: String,
    pub is_active: i64,
    pub created_at: String,
    pub updated_at: String,
}
