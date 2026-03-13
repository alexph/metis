use metis_contract::{
    branch::Branch,
    channel::Channel,
    history::HistoryEvent,
    task::Task,
    worker::Worker,
};
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::{
    branches::{BranchDomainService, BranchService},
    channels::{ChannelDomainService, ChannelService},
    commands::{
        branches::ListBranchesByChannelRequest,
        channels::{CreateChannelRequest, UpdateChannelStatusRequest},
        errors::CommandError,
        history::{AppendHistoryRequest, ListHistoryByBranchRequest, ListHistoryByChannelRequest},
        tasks::{EnqueueTaskRequest, ListTasksByChannelRequest, UpdateTaskStateRequest},
        workers::{CreateWorkerRequest, ListWorkersByTaskRequest, UpdateWorkerStateRequest, WorkerHeartbeatRequest},
    },
    history::{HistoryDomainService, HistoryService},
    storage::repositories::{
        SqliteBranchRepository, SqliteChannelRepository, SqliteHistoryRepository,
        SqliteTaskRepository, SqliteWorkerRepository,
    },
    tasks::{TaskDomainService, TaskService},
    workers::{WorkerDomainService, WorkerService},
};

pub trait CommandService: Send + Sync {
    fn channels_list(&self) -> Result<Vec<Channel>, CommandError>;
    fn channels_create(&self, request: CreateChannelRequest) -> Result<Channel, CommandError>;
    fn channels_update_status(
        &self,
        request: UpdateChannelStatusRequest,
    ) -> Result<Channel, CommandError>;
    fn branches_list_by_channel(
        &self,
        request: ListBranchesByChannelRequest,
    ) -> Result<Vec<Branch>, CommandError>;
    fn tasks_enqueue(&self, request: EnqueueTaskRequest) -> Result<Task, CommandError>;
    fn tasks_update_state(&self, request: UpdateTaskStateRequest) -> Result<Task, CommandError>;
    fn tasks_list_by_channel(
        &self,
        request: ListTasksByChannelRequest,
    ) -> Result<Vec<Task>, CommandError>;
    fn workers_list_by_task(
        &self,
        request: ListWorkersByTaskRequest,
    ) -> Result<Vec<Worker>, CommandError>;
    fn workers_create(&self, request: CreateWorkerRequest) -> Result<Worker, CommandError>;
    fn workers_update_state(
        &self,
        request: UpdateWorkerStateRequest,
    ) -> Result<Worker, CommandError>;
    fn workers_heartbeat(
        &self,
        request: WorkerHeartbeatRequest,
    ) -> Result<Worker, CommandError>;
    fn history_list_by_channel(
        &self,
        request: ListHistoryByChannelRequest,
    ) -> Result<Vec<HistoryEvent>, CommandError>;
    fn history_list_by_branch(
        &self,
        request: ListHistoryByBranchRequest,
    ) -> Result<Vec<HistoryEvent>, CommandError>;
    fn history_append(
        &self,
        request: AppendHistoryRequest,
    ) -> Result<HistoryEvent, CommandError>;
}

pub struct StubCommandService;

impl CommandService for StubCommandService {
    fn channels_list(&self) -> Result<Vec<Channel>, CommandError> {
        Err(CommandError::NotImplemented("channels.list"))
    }

    fn channels_create(&self, _request: CreateChannelRequest) -> Result<Channel, CommandError> {
        Err(CommandError::NotImplemented("channels.create"))
    }

    fn channels_update_status(
        &self,
        _request: UpdateChannelStatusRequest,
    ) -> Result<Channel, CommandError> {
        Err(CommandError::NotImplemented("channels.update_status"))
    }

    fn branches_list_by_channel(
        &self,
        _request: ListBranchesByChannelRequest,
    ) -> Result<Vec<Branch>, CommandError> {
        Err(CommandError::NotImplemented("branches.list_by_channel"))
    }

    fn tasks_enqueue(&self, _request: EnqueueTaskRequest) -> Result<Task, CommandError> {
        Err(CommandError::NotImplemented("tasks.enqueue"))
    }

    fn tasks_update_state(&self, _request: UpdateTaskStateRequest) -> Result<Task, CommandError> {
        Err(CommandError::NotImplemented("tasks.update_state"))
    }

    fn tasks_list_by_channel(
        &self,
        _request: ListTasksByChannelRequest,
    ) -> Result<Vec<Task>, CommandError> {
        Err(CommandError::NotImplemented("tasks.list_by_channel"))
    }

    fn workers_list_by_task(
        &self,
        _request: ListWorkersByTaskRequest,
    ) -> Result<Vec<Worker>, CommandError> {
        Err(CommandError::NotImplemented("workers.get_by_task"))
    }

    fn workers_create(&self, _request: CreateWorkerRequest) -> Result<Worker, CommandError> {
        Err(CommandError::NotImplemented("workers.create"))
    }

    fn workers_update_state(
        &self,
        _request: UpdateWorkerStateRequest,
    ) -> Result<Worker, CommandError> {
        Err(CommandError::NotImplemented("workers.update_state"))
    }

    fn workers_heartbeat(
        &self,
        _request: WorkerHeartbeatRequest,
    ) -> Result<Worker, CommandError> {
        Err(CommandError::NotImplemented("workers.heartbeat"))
    }

    fn history_list_by_channel(
        &self,
        _request: ListHistoryByChannelRequest,
    ) -> Result<Vec<HistoryEvent>, CommandError> {
        Err(CommandError::NotImplemented("history.list_by_channel"))
    }

    fn history_list_by_branch(
        &self,
        _request: ListHistoryByBranchRequest,
    ) -> Result<Vec<HistoryEvent>, CommandError> {
        Err(CommandError::NotImplemented("history.list_by_branch"))
    }

    fn history_append(&self, _request: AppendHistoryRequest) -> Result<HistoryEvent, CommandError> {
        Err(CommandError::NotImplemented("history.append"))
    }
}

pub struct CommandServices {
    command_service: Arc<dyn CommandService>,
}

impl CommandServices {
    pub fn new(command_service: Arc<dyn CommandService>) -> Self {
        Self { command_service }
    }

    pub fn new_real(pool: SqlitePool) -> Self {
        Self {
            command_service: Arc::new(SqliteCommandService::new(pool)),
        }
    }

    pub fn new_stub() -> Self {
        Self {
            command_service: Arc::new(StubCommandService),
        }
    }

    pub fn command_service(&self) -> &dyn CommandService {
        self.command_service.as_ref()
    }
}

pub struct SqliteCommandService {
    channels: ChannelDomainService<SqliteChannelRepository>,
    branches:
        BranchDomainService<SqliteBranchRepository, ChannelDomainService<SqliteChannelRepository>>,
    tasks: TaskDomainService<
        SqliteTaskRepository,
        ChannelDomainService<SqliteChannelRepository>,
        BranchDomainService<SqliteBranchRepository, ChannelDomainService<SqliteChannelRepository>>,
    >,
    workers: WorkerDomainService<
        SqliteWorkerRepository,
        TaskDomainService<
            SqliteTaskRepository,
            ChannelDomainService<SqliteChannelRepository>,
            BranchDomainService<
                SqliteBranchRepository,
                ChannelDomainService<SqliteChannelRepository>,
            >,
        >,
    >,
    history: HistoryDomainService<SqliteHistoryRepository>,
}

impl SqliteCommandService {
    pub fn new(pool: SqlitePool) -> Self {
        let channel_repo_for_channels = SqliteChannelRepository::new(pool.clone());
        let channel_repo_for_branches = SqliteChannelRepository::new(pool.clone());
        let channel_repo_for_tasks = SqliteChannelRepository::new(pool.clone());
        let channel_repo_for_worker_tasks = SqliteChannelRepository::new(pool.clone());

        let branch_repo_for_branches = SqliteBranchRepository::new(pool.clone());
        let branch_repo_for_tasks = SqliteBranchRepository::new(pool.clone());
        let branch_repo_for_worker_tasks = SqliteBranchRepository::new(pool.clone());

        let task_repo_for_tasks = SqliteTaskRepository::new(pool.clone());
        let task_repo_for_workers = SqliteTaskRepository::new(pool.clone());

        let channel_service = ChannelDomainService::new(channel_repo_for_channels);
        let branch_service = BranchDomainService::new(
            branch_repo_for_branches,
            ChannelDomainService::new(channel_repo_for_branches),
        );
        let task_service = TaskDomainService::new(
            task_repo_for_tasks,
            ChannelDomainService::new(channel_repo_for_tasks),
            BranchDomainService::new(
                branch_repo_for_tasks,
                ChannelDomainService::new(channel_repo_for_worker_tasks),
            ),
        );

        Self {
            channels: channel_service,
            branches: branch_service,
            tasks: task_service,
            workers: WorkerDomainService::new(
                SqliteWorkerRepository::new(pool.clone()),
                TaskDomainService::new(
                    task_repo_for_workers,
                    ChannelDomainService::new(SqliteChannelRepository::new(pool.clone())),
                    BranchDomainService::new(
                        branch_repo_for_worker_tasks,
                        ChannelDomainService::new(SqliteChannelRepository::new(pool.clone())),
                    ),
                ),
            ),
            history: HistoryDomainService::new(SqliteHistoryRepository::new(pool)),
        }
    }
}

impl CommandService for SqliteCommandService {
    fn channels_list(&self) -> Result<Vec<Channel>, CommandError> {
        self.channels.list_channels().map_err(CommandError::from)
    }

    fn channels_create(&self, request: CreateChannelRequest) -> Result<Channel, CommandError> {
        self.channels
            .create_channel(request.channel)
            .map_err(CommandError::from)
    }

    fn channels_update_status(
        &self,
        request: UpdateChannelStatusRequest,
    ) -> Result<Channel, CommandError> {
        self.channels
            .update_channel_status(&request.channel_id, request.status)
            .map_err(CommandError::from)?;

        self.channels
            .get_channel(&request.channel_id)
            .map_err(CommandError::from)?
            .ok_or_else(|| CommandError::Internal("updated channel not found".to_string()))
    }

    fn branches_list_by_channel(
        &self,
        request: ListBranchesByChannelRequest,
    ) -> Result<Vec<Branch>, CommandError> {
        self.branches
            .list_by_channel(&request.channel_id)
            .map_err(CommandError::from)
    }

    fn tasks_enqueue(&self, request: EnqueueTaskRequest) -> Result<Task, CommandError> {
        self.tasks.enqueue(request.task).map_err(CommandError::from)
    }

    fn tasks_update_state(&self, request: UpdateTaskStateRequest) -> Result<Task, CommandError> {
        self.tasks
            .update_state(&request.task_id, request.state)
            .map_err(CommandError::from)?;

        self.tasks
            .get(&request.task_id)
            .map_err(CommandError::from)?
            .ok_or_else(|| CommandError::Internal("updated task not found".to_string()))
    }

    fn tasks_list_by_channel(
        &self,
        request: ListTasksByChannelRequest,
    ) -> Result<Vec<Task>, CommandError> {
        self.tasks
            .list_by_channel(&request.channel_id)
            .map_err(CommandError::from)
    }

    fn workers_list_by_task(
        &self,
        request: ListWorkersByTaskRequest,
    ) -> Result<Vec<Worker>, CommandError> {
        self.workers
            .list_by_task(&request.task_id)
            .map_err(CommandError::from)
    }

    fn workers_create(&self, request: CreateWorkerRequest) -> Result<Worker, CommandError> {
        self.workers
            .create_worker(request.worker)
            .map_err(CommandError::from)
    }

    fn workers_update_state(
        &self,
        request: UpdateWorkerStateRequest,
    ) -> Result<Worker, CommandError> {
        self.workers
            .update_state(&request.worker_id, request.state)
            .map_err(CommandError::from)?;

        self.workers
            .get(&request.worker_id)
            .map_err(CommandError::from)?
            .ok_or_else(|| CommandError::Internal("updated worker not found".to_string()))
    }

    fn workers_heartbeat(
        &self,
        request: WorkerHeartbeatRequest,
    ) -> Result<Worker, CommandError> {
        self.workers
            .heartbeat(&request.worker_id, &request.heartbeat_at)
            .map_err(CommandError::from)?;

        self.workers
            .get(&request.worker_id)
            .map_err(CommandError::from)?
            .ok_or_else(|| CommandError::Internal("worker not found after heartbeat".to_string()))
    }

    fn history_list_by_channel(
        &self,
        request: ListHistoryByChannelRequest,
    ) -> Result<Vec<HistoryEvent>, CommandError> {
        self.history
            .list_by_channel(&request.channel_id)
            .map_err(CommandError::from)
    }

    fn history_list_by_branch(
        &self,
        request: ListHistoryByBranchRequest,
    ) -> Result<Vec<HistoryEvent>, CommandError> {
        self.history
            .list_by_branch(&request.branch_id)
            .map_err(CommandError::from)
    }

    fn history_append(&self, request: AppendHistoryRequest) -> Result<HistoryEvent, CommandError> {
        self.history
            .append_event(request.event)
            .map_err(CommandError::from)
    }
}
