use std::str::FromStr;

use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

use crate::{core::paths::MetisPaths, storage::StorageError};

pub async fn connect_sqlite(paths: &MetisPaths) -> Result<SqlitePool, StorageError> {
    std::fs::create_dir_all(&paths.base_dir)?;
    std::fs::create_dir_all(&paths.logs_dir)?;

    let database_url = format!("sqlite:{}", paths.db_path.display());
    let options = SqliteConnectOptions::from_str(&database_url)?.create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;

    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await?;

    Ok(pool)
}
