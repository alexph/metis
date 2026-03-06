use metis_contract::{
    branch::Branch,
    channel::{Channel, ChannelStatus},
    error::ErrorEnvelope,
    history::HistoryEvent,
    metadata::Metadata,
    task::{Task, TaskState},
    worker::{Worker, WorkerState},
};
use sqlx::SqlitePool;
use thiserror::Error;

use crate::{
    core::time::now_utc_rfc3339,
    storage::{
        mappings::{
            branch_record_to_contract, channel_contract_to_record, channel_record_to_contract,
            channel_status_to_storage, history_record_to_contract, task_contract_to_record,
            task_record_to_contract, worker_record_to_contract,
        },
        models::{BranchRecord, ChannelRecord, HistoryRecord, TaskRecord, WorkerRecord},
    },
};

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("mapping error in {context}: {value}")]
    Mapping {
        context: &'static str,
        value: String,
    },
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
}

impl StorageError {
    pub fn to_envelope(&self) -> ErrorEnvelope {
        match self {
            Self::Io(error) => ErrorEnvelope {
                code: "storage_io_error".to_string(),
                message: error.to_string(),
                details: None,
            },
            Self::Database(error) => ErrorEnvelope {
                code: "storage_database_error".to_string(),
                message: error.to_string(),
                details: None,
            },
            Self::Migration(error) => ErrorEnvelope {
                code: "storage_migration_error".to_string(),
                message: error.to_string(),
                details: None,
            },
            Self::Mapping { context, value } => ErrorEnvelope {
                code: "storage_mapping_error".to_string(),
                message: format!("invalid value for {context}"),
                details: Some(value.clone()),
            },
            Self::NotImplemented(message) => ErrorEnvelope {
                code: "storage_not_implemented".to_string(),
                message: (*message).to_string(),
                details: None,
            },
        }
    }
}

pub trait MetadataRepository {
    fn get(&self) -> Result<Option<Metadata>, StorageError>;
    fn upsert_instance(&self, metadata: Metadata) -> Result<Metadata, StorageError>;
}

pub trait ChannelRepository {
    fn create(&self, channel: Channel) -> Result<Channel, StorageError>;
    fn get(&self, id: &str) -> Result<Option<Channel>, StorageError>;
    fn list(&self) -> Result<Vec<Channel>, StorageError>;
    fn update_status(&self, id: &str, status: ChannelStatus) -> Result<(), StorageError>;
}

pub trait BranchRepository {
    fn create(&self, branch: Branch) -> Result<Branch, StorageError>;
    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<Branch>, StorageError>;
}

pub trait TaskRepository {
    fn enqueue(&self, task: Task) -> Result<Task, StorageError>;
    fn get(&self, id: &str) -> Result<Option<Task>, StorageError>;
    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<Task>, StorageError>;
    fn update_state(&self, id: &str, state: TaskState) -> Result<(), StorageError>;
}

pub trait WorkerRepository {
    fn create(&self, worker: Worker) -> Result<Worker, StorageError>;
    fn get_by_task(&self, task_id: &str) -> Result<Vec<Worker>, StorageError>;
    fn update_state(&self, id: &str, state: WorkerState) -> Result<(), StorageError>;
    fn heartbeat(&self, id: &str, heartbeat_at: &str) -> Result<(), StorageError>;
}

pub trait HistoryRepository {
    fn append(&self, event: HistoryEvent) -> Result<HistoryEvent, StorageError>;
    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<HistoryEvent>, StorageError>;
    fn list_by_branch(&self, branch_id: &str) -> Result<Vec<HistoryEvent>, StorageError>;
}

#[derive(Clone)]
pub struct SqliteMetadataRepository {
    _pool: SqlitePool,
}

impl SqliteMetadataRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { _pool: pool }
    }
}

impl MetadataRepository for SqliteMetadataRepository {
    fn get(&self) -> Result<Option<Metadata>, StorageError> {
        Err(StorageError::NotImplemented("metadata.get"))
    }

    fn upsert_instance(&self, _metadata: Metadata) -> Result<Metadata, StorageError> {
        Err(StorageError::NotImplemented("metadata.upsert_instance"))
    }
}

#[derive(Clone)]
pub struct SqliteChannelRepository {
    _pool: SqlitePool,
}

impl SqliteChannelRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { _pool: pool }
    }
}

impl ChannelRepository for SqliteChannelRepository {
    fn create(&self, channel: Channel) -> Result<Channel, StorageError> {
        let record = channel_contract_to_record(channel);
        let pool = self._pool.clone();

        tauri::async_runtime::block_on(async move {
            sqlx::query(
                "INSERT INTO channels (id, title, source_type, source_ref, status, created_at, updated_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&record.id)
            .bind(&record.title)
            .bind(&record.source_type)
            .bind(&record.source_ref)
            .bind(&record.status)
            .bind(&record.created_at)
            .bind(&record.updated_at)
            .execute(&pool)
            .await?;

            let inserted = sqlx::query_as::<_, ChannelRecord>(
                "SELECT id, title, source_type, source_ref, status, created_at, updated_at
                 FROM channels WHERE id = ?",
            )
            .bind(&record.id)
            .fetch_one(&pool)
            .await?;

            channel_record_to_contract(inserted)
        })
    }

    fn get(&self, id: &str) -> Result<Option<Channel>, StorageError> {
        let pool = self._pool.clone();
        let id = id.to_string();

        tauri::async_runtime::block_on(async move {
            let record = sqlx::query_as::<_, ChannelRecord>(
                "SELECT id, title, source_type, source_ref, status, created_at, updated_at
                 FROM channels WHERE id = ?",
            )
            .bind(id)
            .fetch_optional(&pool)
            .await?;

            record.map(channel_record_to_contract).transpose()
        })
    }

    fn list(&self) -> Result<Vec<Channel>, StorageError> {
        let pool = self._pool.clone();

        tauri::async_runtime::block_on(async move {
            let records = sqlx::query_as::<_, ChannelRecord>(
                "SELECT id, title, source_type, source_ref, status, created_at, updated_at
                 FROM channels ORDER BY created_at ASC",
            )
            .fetch_all(&pool)
            .await?;

            records
                .into_iter()
                .map(channel_record_to_contract)
                .collect::<Result<Vec<_>, _>>()
        })
    }

    fn update_status(&self, id: &str, status: ChannelStatus) -> Result<(), StorageError> {
        let pool = self._pool.clone();
        let id = id.to_string();
        let status_value = channel_status_to_storage(status);
        let updated_at = now_utc_rfc3339();

        tauri::async_runtime::block_on(async move {
            sqlx::query("UPDATE channels SET status = ?, updated_at = ? WHERE id = ?")
                .bind(status_value)
                .bind(updated_at)
                .bind(id)
                .execute(&pool)
                .await?;
            Ok(())
        })
    }
}

#[derive(Clone)]
pub struct SqliteBranchRepository {
    _pool: SqlitePool,
}

impl SqliteBranchRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { _pool: pool }
    }
}

impl BranchRepository for SqliteBranchRepository {
    fn create(&self, _branch: Branch) -> Result<Branch, StorageError> {
        Err(StorageError::NotImplemented("branches.create"))
    }

    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<Branch>, StorageError> {
        let pool = self._pool.clone();
        let channel_id = channel_id.to_string();

        tauri::async_runtime::block_on(async move {
            let records = sqlx::query_as::<_, BranchRecord>(
                "SELECT id, channel_id, parent_branch_id, name, is_active, created_at, updated_at
                 FROM branches WHERE channel_id = ? ORDER BY created_at ASC",
            )
            .bind(channel_id)
            .fetch_all(&pool)
            .await?;

            Ok(records.into_iter().map(branch_record_to_contract).collect())
        })
    }
}

#[derive(Clone)]
pub struct SqliteTaskRepository {
    _pool: SqlitePool,
}

impl SqliteTaskRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { _pool: pool }
    }
}

impl TaskRepository for SqliteTaskRepository {
    fn enqueue(&self, task: Task) -> Result<Task, StorageError> {
        let record = task_contract_to_record(task);
        let pool = self._pool.clone();

        tauri::async_runtime::block_on(async move {
            sqlx::query(
                "INSERT INTO tasks (id, channel_id, branch_id, kind, state, priority, payload_json, created_at, updated_at, started_at, finished_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&record.id)
            .bind(&record.channel_id)
            .bind(&record.branch_id)
            .bind(&record.kind)
            .bind(&record.state)
            .bind(record.priority)
            .bind(&record.payload_json)
            .bind(&record.created_at)
            .bind(&record.updated_at)
            .bind(&record.started_at)
            .bind(&record.finished_at)
            .execute(&pool)
            .await?;

            let inserted = sqlx::query_as::<_, TaskRecord>(
                "SELECT id, channel_id, branch_id, kind, state, priority, payload_json, created_at, updated_at, started_at, finished_at
                 FROM tasks WHERE id = ?",
            )
            .bind(&record.id)
            .fetch_one(&pool)
            .await?;

            task_record_to_contract(inserted)
        })
    }

    fn get(&self, id: &str) -> Result<Option<Task>, StorageError> {
        let pool = self._pool.clone();
        let id = id.to_string();

        tauri::async_runtime::block_on(async move {
            let record = sqlx::query_as::<_, TaskRecord>(
                "SELECT id, channel_id, branch_id, kind, state, priority, payload_json, created_at, updated_at, started_at, finished_at
                 FROM tasks WHERE id = ?",
            )
            .bind(id)
            .fetch_optional(&pool)
            .await?;

            record.map(task_record_to_contract).transpose()
        })
    }

    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<Task>, StorageError> {
        let pool = self._pool.clone();
        let channel_id = channel_id.to_string();

        tauri::async_runtime::block_on(async move {
            let records = sqlx::query_as::<_, TaskRecord>(
                "SELECT id, channel_id, branch_id, kind, state, priority, payload_json, created_at, updated_at, started_at, finished_at
                 FROM tasks WHERE channel_id = ? ORDER BY created_at ASC",
            )
            .bind(channel_id)
            .fetch_all(&pool)
            .await?;

            records
                .into_iter()
                .map(task_record_to_contract)
                .collect::<Result<Vec<_>, _>>()
        })
    }

    fn update_state(&self, _id: &str, _state: TaskState) -> Result<(), StorageError> {
        Err(StorageError::NotImplemented("tasks.update_state"))
    }
}

#[derive(Clone)]
pub struct SqliteWorkerRepository {
    _pool: SqlitePool,
}

impl SqliteWorkerRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { _pool: pool }
    }
}

impl WorkerRepository for SqliteWorkerRepository {
    fn create(&self, _worker: Worker) -> Result<Worker, StorageError> {
        Err(StorageError::NotImplemented("workers.create"))
    }

    fn get_by_task(&self, task_id: &str) -> Result<Vec<Worker>, StorageError> {
        let pool = self._pool.clone();
        let task_id = task_id.to_string();

        tauri::async_runtime::block_on(async move {
            let records = sqlx::query_as::<_, WorkerRecord>(
                "SELECT id, task_id, worker_type, state, attempt, last_heartbeat_at, started_at, finished_at, created_at, updated_at
                 FROM workers WHERE task_id = ? ORDER BY created_at ASC",
            )
            .bind(task_id)
            .fetch_all(&pool)
            .await?;

            records
                .into_iter()
                .map(worker_record_to_contract)
                .collect::<Result<Vec<_>, _>>()
        })
    }

    fn update_state(&self, _id: &str, _state: WorkerState) -> Result<(), StorageError> {
        Err(StorageError::NotImplemented("workers.update_state"))
    }

    fn heartbeat(&self, _id: &str, _heartbeat_at: &str) -> Result<(), StorageError> {
        Err(StorageError::NotImplemented("workers.heartbeat"))
    }
}

#[derive(Clone)]
pub struct SqliteHistoryRepository {
    _pool: SqlitePool,
}

impl SqliteHistoryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { _pool: pool }
    }
}

impl HistoryRepository for SqliteHistoryRepository {
    fn append(&self, _event: HistoryEvent) -> Result<HistoryEvent, StorageError> {
        Err(StorageError::NotImplemented("history.append"))
    }

    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<HistoryEvent>, StorageError> {
        let pool = self._pool.clone();
        let channel_id = channel_id.to_string();

        tauri::async_runtime::block_on(async move {
            let records = sqlx::query_as::<_, HistoryRecord>(
                "SELECT id, channel_id, branch_id, task_id, worker_id, event_type, role, content_json, correlation_id, created_at
                 FROM history WHERE channel_id = ? ORDER BY created_at ASC",
            )
            .bind(channel_id)
            .fetch_all(&pool)
            .await?;

            records
                .into_iter()
                .map(history_record_to_contract)
                .collect::<Result<Vec<_>, _>>()
        })
    }

    fn list_by_branch(&self, branch_id: &str) -> Result<Vec<HistoryEvent>, StorageError> {
        let pool = self._pool.clone();
        let branch_id = branch_id.to_string();

        tauri::async_runtime::block_on(async move {
            let records = sqlx::query_as::<_, HistoryRecord>(
                "SELECT id, channel_id, branch_id, task_id, worker_id, event_type, role, content_json, correlation_id, created_at
                 FROM history WHERE branch_id = ? ORDER BY created_at ASC",
            )
            .bind(branch_id)
            .fetch_all(&pool)
            .await?;

            records
                .into_iter()
                .map(history_record_to_contract)
                .collect::<Result<Vec<_>, _>>()
        })
    }
}
