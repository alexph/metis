use std::io;
use std::path::{Path, PathBuf};

const APP_NAME: &str = "metis";
const CONFIG_FILE_NAME: &str = "config.toml";

pub fn config_dir() -> Result<PathBuf, io::Error> {
    platform_dir(dirs::config_dir())
}

pub fn data_dir() -> Result<PathBuf, io::Error> {
    platform_dir(dirs::data_dir())
}

pub fn default_config_file() -> Result<PathBuf, io::Error> {
    Ok(config_dir()?.join(CONFIG_FILE_NAME))
}

pub fn ensure_dir(path: &Path) -> Result<(), io::Error> {
    std::fs::create_dir_all(path)
}

fn platform_dir(base_dir: Option<PathBuf>) -> Result<PathBuf, io::Error> {
    base_dir.map(|path| path.join(APP_NAME)).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to resolve platform directory",
        )
    })
}
