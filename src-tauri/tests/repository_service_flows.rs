use std::str::FromStr;

use metis_contract::{
    branch::Branch,
    channel::{Channel, ChannelSourceType, ChannelStatus},
    history::HistoryEvent,
    task::{Task, TaskState},
    worker::{Worker, WorkerState},
};
use metis_lib::{
    branches::BranchDomainService,
    channels::ChannelDomainService,
    storage::{migrations, repositories::*},
    tasks::{TaskDomainService, TaskService},
};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tempfile::TempDir;

struct TestDb {
    _temp_dir: TempDir,
    pool: SqlitePool,
}

impl TestDb {
    fn new() -> Self {
        let temp_dir = tempfile::tempdir().expect("temp dir should be created");
        let db_path = temp_dir.path().join("metis-test.db");
        let database_url = format!("sqlite:{}", db_path.display());

        let options = SqliteConnectOptions::from_str(&database_url)
            .expect("sqlite options should parse")
            .create_if_missing(true);

        let pool = tauri::async_runtime::block_on(SqlitePool::connect_with(options))
            .expect("sqlite pool should connect");

        tauri::async_runtime::block_on(async {
            sqlx::query("PRAGMA foreign_keys = ON")
                .execute(&pool)
                .await
                .expect("foreign keys should enable");
            migrations::run_migrations(&pool)
                .await
                .expect("migrations should run");
        });

        Self {
            _temp_dir: temp_dir,
            pool,
        }
    }
}

fn channel_fixture(id: &str) -> Channel {
    Channel {
        id: id.to_string(),
        title: "Test Channel".to_string(),
        source_type: ChannelSourceType::Manual,
        source_ref: None,
        status: ChannelStatus::Active,
        created_at: "2026-01-01T00:00:00Z".to_string(),
        updated_at: "2026-01-01T00:00:00Z".to_string(),
    }
}

#[test]
fn task_repository_state_updates_set_timestamps() {
    let db = TestDb::new();
    let channels = SqliteChannelRepository::new(db.pool.clone());
    let tasks = SqliteTaskRepository::new(db.pool.clone());

    channels
        .create(channel_fixture("channel-task"))
        .expect("channel should be created");

    tasks
        .enqueue(Task {
            id: "task-1".to_string(),
            channel_id: "channel-task".to_string(),
            branch_id: None,
            kind: "analysis".to_string(),
            state: TaskState::Queued,
            priority: 0,
            payload_json: None,
            created_at: "2026-01-01T00:00:01Z".to_string(),
            updated_at: "2026-01-01T00:00:01Z".to_string(),
            started_at: None,
            finished_at: None,
        })
        .expect("task should be enqueued");

    tasks
        .update_state("task-1", TaskState::Running)
        .expect("task should transition to running");

    let running_task = tasks
        .get("task-1")
        .expect("task get should succeed")
        .expect("task should exist");
    assert!(running_task.started_at.is_some());
    assert!(running_task.finished_at.is_none());

    tasks
        .update_state("task-1", TaskState::Completed)
        .expect("task should transition to completed");

    let completed_task = tasks
        .get("task-1")
        .expect("task get should succeed")
        .expect("task should exist");
    assert!(completed_task.started_at.is_some());
    assert!(completed_task.finished_at.is_some());
}

#[test]
fn worker_repository_state_and_heartbeat_are_persisted() {
    let db = TestDb::new();
    let channels = SqliteChannelRepository::new(db.pool.clone());
    let tasks = SqliteTaskRepository::new(db.pool.clone());
    let workers = SqliteWorkerRepository::new(db.pool.clone());

    channels
        .create(channel_fixture("channel-worker"))
        .expect("channel should be created");

    tasks
        .enqueue(Task {
            id: "task-worker".to_string(),
            channel_id: "channel-worker".to_string(),
            branch_id: None,
            kind: "worker-task".to_string(),
            state: TaskState::Queued,
            priority: 1,
            payload_json: None,
            created_at: "2026-01-01T00:00:01Z".to_string(),
            updated_at: "2026-01-01T00:00:01Z".to_string(),
            started_at: None,
            finished_at: None,
        })
        .expect("task should be enqueued");

    workers
        .create(Worker {
            id: "worker-1".to_string(),
            task_id: "task-worker".to_string(),
            worker_type: "agent".to_string(),
            state: WorkerState::Pending,
            attempt: 0,
            last_heartbeat_at: None,
            started_at: None,
            finished_at: None,
            created_at: "2026-01-01T00:00:02Z".to_string(),
            updated_at: "2026-01-01T00:00:02Z".to_string(),
        })
        .expect("worker should be created");

    workers
        .update_state("worker-1", WorkerState::Running)
        .expect("worker should transition to running");

    let running_worker = workers
        .get("worker-1")
        .expect("worker get should succeed")
        .expect("worker should exist");
    assert!(running_worker.started_at.is_some());
    assert!(running_worker.finished_at.is_none());

    let heartbeat_at = "2026-01-01T00:00:10Z";
    workers
        .heartbeat("worker-1", heartbeat_at)
        .expect("worker heartbeat should persist");

    let after_heartbeat = workers
        .get("worker-1")
        .expect("worker get should succeed")
        .expect("worker should exist");
    assert_eq!(
        after_heartbeat.last_heartbeat_at.as_deref(),
        Some(heartbeat_at)
    );

    workers
        .update_state("worker-1", WorkerState::Completed)
        .expect("worker should transition to completed");

    let completed_worker = workers
        .get("worker-1")
        .expect("worker get should succeed")
        .expect("worker should exist");
    assert!(completed_worker.finished_at.is_some());
}

#[test]
fn history_append_and_lists_are_ordered() {
    let db = TestDb::new();
    let channels = SqliteChannelRepository::new(db.pool.clone());
    let branches = SqliteBranchRepository::new(db.pool.clone());
    let history = SqliteHistoryRepository::new(db.pool.clone());

    channels
        .create(channel_fixture("channel-history"))
        .expect("channel should be created");

    branches
        .create(Branch {
            id: "branch-history".to_string(),
            channel_id: "channel-history".to_string(),
            parent_branch_id: None,
            name: "main".to_string(),
            is_active: true,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
        })
        .expect("branch should be created");

    history
        .append(HistoryEvent {
            id: "hist-1".to_string(),
            channel_id: "channel-history".to_string(),
            branch_id: Some("branch-history".to_string()),
            task_id: None,
            worker_id: None,
            event_type: "message".to_string(),
            role: None,
            content_json: "{\"text\":\"first\"}".to_string(),
            correlation_id: Some("corr-1".to_string()),
            created_at: "2026-01-01T00:00:01Z".to_string(),
        })
        .expect("first history event should append");

    history
        .append(HistoryEvent {
            id: "hist-2".to_string(),
            channel_id: "channel-history".to_string(),
            branch_id: Some("branch-history".to_string()),
            task_id: None,
            worker_id: None,
            event_type: "message".to_string(),
            role: None,
            content_json: "{\"text\":\"second\"}".to_string(),
            correlation_id: Some("corr-1".to_string()),
            created_at: "2026-01-01T00:00:02Z".to_string(),
        })
        .expect("second history event should append");

    let by_channel = history
        .list_by_channel("channel-history")
        .expect("history channel list should succeed");
    assert_eq!(by_channel.len(), 2);
    assert_eq!(by_channel[0].id, "hist-1");
    assert_eq!(by_channel[1].id, "hist-2");

    let by_branch = history
        .list_by_branch("branch-history")
        .expect("history branch list should succeed");
    assert_eq!(by_branch.len(), 2);
    assert_eq!(by_branch[0].id, "hist-1");
    assert_eq!(by_branch[1].id, "hist-2");
}

#[test]
fn task_service_rejects_invalid_transition() {
    let db = TestDb::new();

    let channels_repo = SqliteChannelRepository::new(db.pool.clone());
    channels_repo
        .create(channel_fixture("channel-service"))
        .expect("channel should be created");

    let task_service = TaskDomainService::new(
        SqliteTaskRepository::new(db.pool.clone()),
        ChannelDomainService::new(SqliteChannelRepository::new(db.pool.clone())),
        BranchDomainService::new(
            SqliteBranchRepository::new(db.pool.clone()),
            ChannelDomainService::new(SqliteChannelRepository::new(db.pool.clone())),
        ),
    );

    task_service
        .enqueue(Task {
            id: "task-service-1".to_string(),
            channel_id: "channel-service".to_string(),
            branch_id: None,
            kind: "service".to_string(),
            state: TaskState::Queued,
            priority: 0,
            payload_json: None,
            created_at: "2026-01-01T00:00:01Z".to_string(),
            updated_at: "2026-01-01T00:00:01Z".to_string(),
            started_at: None,
            finished_at: None,
        })
        .expect("task should enqueue through service");

    task_service
        .update_state("task-service-1", TaskState::Running)
        .expect("queued -> running should pass");
    task_service
        .update_state("task-service-1", TaskState::Completed)
        .expect("running -> completed should pass");

    let err = task_service
        .update_state("task-service-1", TaskState::Running)
        .expect_err("completed -> running should fail");
    assert_eq!(err.code, "service_conflict");
}
