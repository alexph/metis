use metis_contract::{
    branch::Branch, channel::Channel, history::HistoryEvent, task::Task, worker::Worker,
};
use sqlx::SqlitePool;

use crate::{
    adapters::desktop::{
        commands::{
            CreateChannelRequest, DesktopCommandService, EnqueueTaskRequest,
            ListBranchesByChannelRequest, ListHistoryByBranchRequest, ListHistoryByChannelRequest,
            ListTasksByChannelRequest, ListWorkersByTaskRequest,
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
        self.channels.list_channels().map_err(Into::into)
    }

    fn channels_create(
        &self,
        request: CreateChannelRequest,
    ) -> Result<Channel, DesktopAdapterError> {
        self.channels
            .create_channel(request.channel)
            .map_err(Into::into)
    }

    fn branches_list_by_channel(
        &self,
        request: ListBranchesByChannelRequest,
    ) -> Result<Vec<Branch>, DesktopAdapterError> {
        self.branches
            .list_by_channel(&request.channel_id)
            .map_err(Into::into)
    }

    fn tasks_enqueue(&self, request: EnqueueTaskRequest) -> Result<Task, DesktopAdapterError> {
        self.tasks.enqueue(request.task).map_err(Into::into)
    }

    fn tasks_list_by_channel(
        &self,
        request: ListTasksByChannelRequest,
    ) -> Result<Vec<Task>, DesktopAdapterError> {
        self.tasks
            .list_by_channel(&request.channel_id)
            .map_err(Into::into)
    }

    fn workers_list_by_task(
        &self,
        request: ListWorkersByTaskRequest,
    ) -> Result<Vec<Worker>, DesktopAdapterError> {
        self.workers
            .list_by_task(&request.task_id)
            .map_err(Into::into)
    }

    fn history_list_by_channel(
        &self,
        request: ListHistoryByChannelRequest,
    ) -> Result<Vec<HistoryEvent>, DesktopAdapterError> {
        self.history
            .list_by_channel(&request.channel_id)
            .map_err(Into::into)
    }

    fn history_list_by_branch(
        &self,
        request: ListHistoryByBranchRequest,
    ) -> Result<Vec<HistoryEvent>, DesktopAdapterError> {
        self.history
            .list_by_branch(&request.branch_id)
            .map_err(Into::into)
    }
}
