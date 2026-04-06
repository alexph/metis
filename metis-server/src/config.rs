pub struct Config {}

impl Config {
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Self {
        let contents = std::fs::read_to_string(path).unwrap_or_default();
        let _ = contents;
        Self {}
    }
}
