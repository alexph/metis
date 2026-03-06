use std::path::PathBuf;

use crate::core::error::MetisError;

#[derive(Debug, Clone)]
pub struct MetisPaths {
    pub base_dir: PathBuf,
    pub db_path: PathBuf,
    pub logs_dir: PathBuf,
    pub config_path: PathBuf,
}

impl MetisPaths {
    pub fn resolve() -> Result<Self, MetisError> {
        let home = dirs::home_dir().ok_or_else(|| {
            MetisError::Config("failed to resolve user home directory".to_string())
        })?;

        let base_dir = home.join(".metis");
        let db_path = base_dir.join("metis.db");
        let logs_dir = base_dir.join("logs");
        let config_path = base_dir.join("config.toml");

        Ok(Self {
            base_dir,
            db_path,
            logs_dir,
            config_path,
        })
    }
}
