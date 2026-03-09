use metis_contract::{
    branch::Branch, channel::Channel, history::HistoryEvent, task::Task, worker::Worker,
};
use sqlx::SqlitePool;

use crate::{
    adapters::desktop::{
        commands::{
            AppendHistoryRequest, CreateChannelRequest, CreateWorkerRequest, DesktopCommandService,
            EnqueueTaskRequest, ListBranchesByChannelRequest, ListHistoryByBranchRequest,
            ListHistoryByChannelRequest, ListTasksByChannelRequest, ListWorkersByTaskRequest,
            UpdateChannelStatusRequest, UpdateTaskStateRequest, UpdateWorkerStateRequest,
            WorkerHeartbeatRequest,
        },
        errors::DesktopAdapterError,
    },
    branches::{BranchDomainService, BranchService},
    channels::{ChannelDomainService, ChannelService},
    history::{HistoryDomainService, HistoryService},
    storage::repositories::{
        SqliteBranchRepository, SqliteChannelRepository, SqliteHistoryRepository,
        SqliteTaskRepository, SqliteWorkerRepository,
    },
    tasks::{TaskDomainService, TaskService},
    workers::{WorkerDomainService, WorkerService},
};

pub struct SqliteDesktopCommandService {
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

impl SqliteDesktopCommandService {
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

impl DesktopCommandService for SqliteDesktopCommandService {
    fn channels_list(&self) -> Result<Vec<Channel>, DesktopAdapterError> {
        self.channels
            .list_channels()
            .map_err(DesktopAdapterError::from)
    }

    fn channels_create(
        &self,
        request: CreateChannelRequest,
    ) -> Result<Channel, DesktopAdapterError> {
        self.channels
            .create_channel(request.channel)
            .map_err(DesktopAdapterError::from)
    }

    fn channels_update_status(
        &self,
        request: UpdateChannelStatusRequest,
    ) -> Result<Channel, DesktopAdapterError> {
        self.channels
            .update_channel_status(&request.channel_id, request.status)
            .map_err(DesktopAdapterError::from)?;

        self.channels
            .get_channel(&request.channel_id)
            .map_err(DesktopAdapterError::from)?
            .ok_or_else(|| DesktopAdapterError::Internal("updated channel not found".to_string()))
    }

    fn branches_list_by_channel(
        &self,
        request: ListBranchesByChannelRequest,
    ) -> Result<Vec<Branch>, DesktopAdapterError> {
        self.branches
            .list_by_channel(&request.channel_id)
            .map_err(DesktopAdapterError::from)
    }

    fn tasks_enqueue(&self, request: EnqueueTaskRequest) -> Result<Task, DesktopAdapterError> {
        self.tasks
            .enqueue(request.task)
            .map_err(DesktopAdapterError::from)
    }

    fn tasks_update_state(
        &self,
        request: UpdateTaskStateRequest,
    ) -> Result<Task, DesktopAdapterError> {
        self.tasks
            .update_state(&request.task_id, request.state)
            .map_err(DesktopAdapterError::from)?;

        self.tasks
            .get(&request.task_id)
            .map_err(DesktopAdapterError::from)?
            .ok_or_else(|| DesktopAdapterError::Internal("updated task not found".to_string()))
    }

    fn tasks_list_by_channel(
        &self,
        request: ListTasksByChannelRequest,
    ) -> Result<Vec<Task>, DesktopAdapterError> {
        self.tasks
            .list_by_channel(&request.channel_id)
            .map_err(DesktopAdapterError::from)
    }

    fn workers_list_by_task(
        &self,
        request: ListWorkersByTaskRequest,
    ) -> Result<Vec<Worker>, DesktopAdapterError> {
        self.workers
            .list_by_task(&request.task_id)
            .map_err(DesktopAdapterError::from)
    }

    fn workers_create(&self, request: CreateWorkerRequest) -> Result<Worker, DesktopAdapterError> {
        self.workers
            .create_worker(request.worker)
            .map_err(DesktopAdapterError::from)
    }

    fn workers_update_state(
        &self,
        request: UpdateWorkerStateRequest,
    ) -> Result<Worker, DesktopAdapterError> {
        self.workers
            .update_state(&request.worker_id, request.state)
            .map_err(DesktopAdapterError::from)?;

        self.workers
            .get(&request.worker_id)
            .map_err(DesktopAdapterError::from)?
            .ok_or_else(|| DesktopAdapterError::Internal("updated worker not found".to_string()))
    }

    fn workers_heartbeat(
        &self,
        request: WorkerHeartbeatRequest,
    ) -> Result<Worker, DesktopAdapterError> {
        self.workers
            .heartbeat(&request.worker_id, &request.heartbeat_at)
            .map_err(DesktopAdapterError::from)?;

        self.workers
            .get(&request.worker_id)
            .map_err(DesktopAdapterError::from)?
            .ok_or_else(|| {
                DesktopAdapterError::Internal("worker not found after heartbeat".to_string())
            })
    }

    fn history_list_by_channel(
        &self,
        request: ListHistoryByChannelRequest,
    ) -> Result<Vec<HistoryEvent>, DesktopAdapterError> {
        self.history
            .list_by_channel(&request.channel_id)
            .map_err(DesktopAdapterError::from)
    }

    fn history_list_by_branch(
        &self,
        request: ListHistoryByBranchRequest,
    ) -> Result<Vec<HistoryEvent>, DesktopAdapterError> {
        self.history
            .list_by_branch(&request.branch_id)
            .map_err(DesktopAdapterError::from)
    }

    fn history_append(
        &self,
        request: AppendHistoryRequest,
    ) -> Result<HistoryEvent, DesktopAdapterError> {
        self.history
            .append_event(request.event)
            .map_err(DesktopAdapterError::from)
    }
}
