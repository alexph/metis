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

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
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
    fn create(&self, _channel: Channel) -> Result<Channel, StorageError> {
        Err(StorageError::NotImplemented("channels.create"))
    }

    fn get(&self, _id: &str) -> Result<Option<Channel>, StorageError> {
        Err(StorageError::NotImplemented("channels.get"))
    }

    fn list(&self) -> Result<Vec<Channel>, StorageError> {
        Err(StorageError::NotImplemented("channels.list"))
    }

    fn update_status(&self, _id: &str, _status: ChannelStatus) -> Result<(), StorageError> {
        Err(StorageError::NotImplemented("channels.update_status"))
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

    fn list_by_channel(&self, _channel_id: &str) -> Result<Vec<Branch>, StorageError> {
        Err(StorageError::NotImplemented("branches.list_by_channel"))
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
    fn enqueue(&self, _task: Task) -> Result<Task, StorageError> {
        Err(StorageError::NotImplemented("tasks.enqueue"))
    }

    fn get(&self, _id: &str) -> Result<Option<Task>, StorageError> {
        Err(StorageError::NotImplemented("tasks.get"))
    }

    fn list_by_channel(&self, _channel_id: &str) -> Result<Vec<Task>, StorageError> {
        Err(StorageError::NotImplemented("tasks.list_by_channel"))
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

    fn get_by_task(&self, _task_id: &str) -> Result<Vec<Worker>, StorageError> {
        Err(StorageError::NotImplemented("workers.get_by_task"))
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

    fn list_by_channel(&self, _channel_id: &str) -> Result<Vec<HistoryEvent>, StorageError> {
        Err(StorageError::NotImplemented("history.list_by_channel"))
    }

    fn list_by_branch(&self, _branch_id: &str) -> Result<Vec<HistoryEvent>, StorageError> {
        Err(StorageError::NotImplemented("history.list_by_branch"))
    }
}
