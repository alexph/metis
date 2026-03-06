use metis_contract::{
    branch::Branch, channel::Channel, error::ErrorEnvelope, history::HistoryEvent, task::Task,
    worker::Worker,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::adapters::desktop::errors::DesktopAdapterError;
use crate::adapters::desktop::service::SqliteDesktopCommandService;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum CommandResponse<T> {
    Ok { data: T },
    Err { error: ErrorEnvelope },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelRequest {
    pub channel: Channel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBranchesByChannelRequest {
    pub channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnqueueTaskRequest {
    pub task: Task,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTasksByChannelRequest {
    pub channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWorkersByTaskRequest {
    pub task_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHistoryByChannelRequest {
    pub channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHistoryByBranchRequest {
    pub branch_id: String,
}

pub trait DesktopCommandService: Send + Sync {
    fn channels_list(&self) -> Result<Vec<Channel>, DesktopAdapterError>;
    fn channels_create(
        &self,
        request: CreateChannelRequest,
    ) -> Result<Channel, DesktopAdapterError>;
    fn branches_list_by_channel(
        &self,
        request: ListBranchesByChannelRequest,
    ) -> Result<Vec<Branch>, DesktopAdapterError>;
    fn tasks_enqueue(&self, request: EnqueueTaskRequest) -> Result<Task, DesktopAdapterError>;
    fn tasks_list_by_channel(
        &self,
        request: ListTasksByChannelRequest,
    ) -> Result<Vec<Task>, DesktopAdapterError>;
    fn workers_list_by_task(
        &self,
        request: ListWorkersByTaskRequest,
    ) -> Result<Vec<Worker>, DesktopAdapterError>;
    fn history_list_by_channel(
        &self,
        request: ListHistoryByChannelRequest,
    ) -> Result<Vec<HistoryEvent>, DesktopAdapterError>;
    fn history_list_by_branch(
        &self,
        request: ListHistoryByBranchRequest,
    ) -> Result<Vec<HistoryEvent>, DesktopAdapterError>;
}

pub struct StubDesktopCommandService;

impl DesktopCommandService for StubDesktopCommandService {
    fn channels_list(&self) -> Result<Vec<Channel>, DesktopAdapterError> {
        Err(DesktopAdapterError::NotImplemented("channels.list"))
    }

    fn channels_create(
        &self,
        _request: CreateChannelRequest,
    ) -> Result<Channel, DesktopAdapterError> {
        Err(DesktopAdapterError::NotImplemented("channels.create"))
    }

    fn branches_list_by_channel(
        &self,
        _request: ListBranchesByChannelRequest,
    ) -> Result<Vec<Branch>, DesktopAdapterError> {
        Err(DesktopAdapterError::NotImplemented(
            "branches.list_by_channel",
        ))
    }

    fn tasks_enqueue(&self, _request: EnqueueTaskRequest) -> Result<Task, DesktopAdapterError> {
        Err(DesktopAdapterError::NotImplemented("tasks.enqueue"))
    }

    fn tasks_list_by_channel(
        &self,
        _request: ListTasksByChannelRequest,
    ) -> Result<Vec<Task>, DesktopAdapterError> {
        Err(DesktopAdapterError::NotImplemented("tasks.list_by_channel"))
    }

    fn workers_list_by_task(
        &self,
        _request: ListWorkersByTaskRequest,
    ) -> Result<Vec<Worker>, DesktopAdapterError> {
        Err(DesktopAdapterError::NotImplemented("workers.get_by_task"))
    }

    fn history_list_by_channel(
        &self,
        _request: ListHistoryByChannelRequest,
    ) -> Result<Vec<HistoryEvent>, DesktopAdapterError> {
        Err(DesktopAdapterError::NotImplemented(
            "history.list_by_channel",
        ))
    }

    fn history_list_by_branch(
        &self,
        _request: ListHistoryByBranchRequest,
    ) -> Result<Vec<HistoryEvent>, DesktopAdapterError> {
        Err(DesktopAdapterError::NotImplemented(
            "history.list_by_branch",
        ))
    }
}

pub struct DesktopCommandServices {
    command_service: Arc<dyn DesktopCommandService>,
}

impl DesktopCommandServices {
    pub fn new(command_service: Arc<dyn DesktopCommandService>) -> Self {
        Self { command_service }
    }

    pub fn new_real(pool: sqlx::SqlitePool) -> Self {
        Self {
            command_service: Arc::new(SqliteDesktopCommandService::new(pool)),
        }
    }

    pub fn new_stub() -> Self {
        Self {
            command_service: Arc::new(StubDesktopCommandService),
        }
    }

    fn command_service(&self) -> &dyn DesktopCommandService {
        self.command_service.as_ref()
    }
}

#[tauri::command]
pub fn desktop_channels_list(
    services: tauri::State<'_, DesktopCommandServices>,
) -> CommandResponse<Vec<Channel>> {
    command_result(services.command_service().channels_list())
}

#[tauri::command]
pub fn desktop_channels_create(
    services: tauri::State<'_, DesktopCommandServices>,
    request: CreateChannelRequest,
) -> CommandResponse<Channel> {
    command_result(services.command_service().channels_create(request))
}

#[tauri::command]
pub fn desktop_branches_list_by_channel(
    services: tauri::State<'_, DesktopCommandServices>,
    request: ListBranchesByChannelRequest,
) -> CommandResponse<Vec<Branch>> {
    command_result(services.command_service().branches_list_by_channel(request))
}

#[tauri::command]
pub fn desktop_tasks_enqueue(
    services: tauri::State<'_, DesktopCommandServices>,
    request: EnqueueTaskRequest,
) -> CommandResponse<Task> {
    command_result(services.command_service().tasks_enqueue(request))
}

#[tauri::command]
pub fn desktop_tasks_list_by_channel(
    services: tauri::State<'_, DesktopCommandServices>,
    request: ListTasksByChannelRequest,
) -> CommandResponse<Vec<Task>> {
    command_result(services.command_service().tasks_list_by_channel(request))
}

#[tauri::command]
pub fn desktop_workers_list_by_task(
    services: tauri::State<'_, DesktopCommandServices>,
    request: ListWorkersByTaskRequest,
) -> CommandResponse<Vec<Worker>> {
    command_result(services.command_service().workers_list_by_task(request))
}

#[tauri::command]
pub fn desktop_history_list_by_channel(
    services: tauri::State<'_, DesktopCommandServices>,
    request: ListHistoryByChannelRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    command_result(services.command_service().history_list_by_channel(request))
}

#[tauri::command]
pub fn desktop_history_list_by_branch(
    services: tauri::State<'_, DesktopCommandServices>,
    request: ListHistoryByBranchRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    command_result(services.command_service().history_list_by_branch(request))
}

fn command_result<T>(result: Result<T, DesktopAdapterError>) -> CommandResponse<T> {
    match result {
        Ok(data) => CommandResponse::Ok { data },
        Err(error) => CommandResponse::Err {
            error: error.to_envelope(),
        },
    }
}
