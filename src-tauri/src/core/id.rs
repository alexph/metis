pub fn new_uuid_v7_string() -> String {
    uuid::Uuid::now_v7().to_string()
}
