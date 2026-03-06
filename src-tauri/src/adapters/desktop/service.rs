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
    storage::repositories::{
        BranchRepository, ChannelRepository, HistoryRepository, SqliteBranchRepository,
        SqliteChannelRepository, SqliteHistoryRepository, SqliteTaskRepository,
        SqliteWorkerRepository, TaskRepository, WorkerRepository,
    },
};

pub struct SqliteDesktopCommandService {
    channels: SqliteChannelRepository,
    branches: SqliteBranchRepository,
    tasks: SqliteTaskRepository,
    workers: SqliteWorkerRepository,
    history: SqliteHistoryRepository,
}

impl SqliteDesktopCommandService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            channels: SqliteChannelRepository::new(pool.clone()),
            branches: SqliteBranchRepository::new(pool.clone()),
            tasks: SqliteTaskRepository::new(pool.clone()),
            workers: SqliteWorkerRepository::new(pool.clone()),
            history: SqliteHistoryRepository::new(pool),
        }
    }
}

impl DesktopCommandService for SqliteDesktopCommandService {
    fn channels_list(&self) -> Result<Vec<Channel>, DesktopAdapterError> {
        self.channels.list().map_err(Into::into)
    }

    fn channels_create(
        &self,
        request: CreateChannelRequest,
    ) -> Result<Channel, DesktopAdapterError> {
        self.channels.create(request.channel).map_err(Into::into)
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
            .get_by_task(&request.task_id)
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
