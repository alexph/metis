use sqlx::SqlitePool;

use crate::storage::StorageError;

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), StorageError> {
    sqlx::migrate!().run(pool).await?;
    Ok(())
}
