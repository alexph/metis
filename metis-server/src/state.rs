use std::path::PathBuf;

pub struct State {
    config_path: PathBuf,
    data_dir: PathBuf,
}

impl State {
    pub fn new(config_path: PathBuf, data_dir: PathBuf) -> Self {
        Self {
            config_path,
            data_dir,
        }
    }
}
