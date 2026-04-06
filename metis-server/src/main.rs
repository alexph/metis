mod paths;
mod state;

use clap::{Command, arg, value_parser};
use std::path::PathBuf;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("metis")
        .arg(
            arg!(-c --config <FILE> "Path to the configuration file")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(--"data-dir" <DIR> "Path to the application data directory")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let config_path = matches
        .get_one::<PathBuf>("config")
        .cloned()
        .map_or_else(paths::default_config_file, Ok)?;

    let data_dir = matches
        .get_one::<PathBuf>("data-dir")
        .cloned()
        .map_or_else(paths::data_dir, Ok)?;

    paths::ensure_dir(&data_dir)?;

    println!("Config file: {}", config_path.display());
    println!("Data directory: {}", data_dir.display());

    let _state = state::State::new(config_path, data_dir);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
