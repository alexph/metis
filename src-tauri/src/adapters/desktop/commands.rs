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
    handle_channels_list(services.command_service())
}

#[tauri::command]
pub fn desktop_channels_create(
    services: tauri::State<'_, DesktopCommandServices>,
    request: CreateChannelRequest,
) -> CommandResponse<Channel> {
    handle_channels_create(services.command_service(), request)
}

#[tauri::command]
pub fn desktop_branches_list_by_channel(
    services: tauri::State<'_, DesktopCommandServices>,
    request: ListBranchesByChannelRequest,
) -> CommandResponse<Vec<Branch>> {
    handle_branches_list_by_channel(services.command_service(), request)
}

#[tauri::command]
pub fn desktop_tasks_enqueue(
    services: tauri::State<'_, DesktopCommandServices>,
    request: EnqueueTaskRequest,
) -> CommandResponse<Task> {
    handle_tasks_enqueue(services.command_service(), request)
}

#[tauri::command]
pub fn desktop_tasks_list_by_channel(
    services: tauri::State<'_, DesktopCommandServices>,
    request: ListTasksByChannelRequest,
) -> CommandResponse<Vec<Task>> {
    handle_tasks_list_by_channel(services.command_service(), request)
}

#[tauri::command]
pub fn desktop_workers_list_by_task(
    services: tauri::State<'_, DesktopCommandServices>,
    request: ListWorkersByTaskRequest,
) -> CommandResponse<Vec<Worker>> {
    handle_workers_list_by_task(services.command_service(), request)
}

#[tauri::command]
pub fn desktop_history_list_by_channel(
    services: tauri::State<'_, DesktopCommandServices>,
    request: ListHistoryByChannelRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    handle_history_list_by_channel(services.command_service(), request)
}

#[tauri::command]
pub fn desktop_history_list_by_branch(
    services: tauri::State<'_, DesktopCommandServices>,
    request: ListHistoryByBranchRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    handle_history_list_by_branch(services.command_service(), request)
}

fn handle_channels_list(service: &dyn DesktopCommandService) -> CommandResponse<Vec<Channel>> {
    command_result(service.channels_list())
}

fn handle_channels_create(
    service: &dyn DesktopCommandService,
    request: CreateChannelRequest,
) -> CommandResponse<Channel> {
    command_result(service.channels_create(request))
}

fn handle_branches_list_by_channel(
    service: &dyn DesktopCommandService,
    request: ListBranchesByChannelRequest,
) -> CommandResponse<Vec<Branch>> {
    command_result(service.branches_list_by_channel(request))
}

fn handle_tasks_enqueue(
    service: &dyn DesktopCommandService,
    request: EnqueueTaskRequest,
) -> CommandResponse<Task> {
    command_result(service.tasks_enqueue(request))
}

fn handle_tasks_list_by_channel(
    service: &dyn DesktopCommandService,
    request: ListTasksByChannelRequest,
) -> CommandResponse<Vec<Task>> {
    command_result(service.tasks_list_by_channel(request))
}

fn handle_workers_list_by_task(
    service: &dyn DesktopCommandService,
    request: ListWorkersByTaskRequest,
) -> CommandResponse<Vec<Worker>> {
    command_result(service.workers_list_by_task(request))
}

fn handle_history_list_by_channel(
    service: &dyn DesktopCommandService,
    request: ListHistoryByChannelRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    command_result(service.history_list_by_channel(request))
}

fn handle_history_list_by_branch(
    service: &dyn DesktopCommandService,
    request: ListHistoryByBranchRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    command_result(service.history_list_by_branch(request))
}

fn command_result<T>(result: Result<T, DesktopAdapterError>) -> CommandResponse<T> {
    match result {
        Ok(data) => CommandResponse::Ok { data },
        Err(error) => CommandResponse::Err {
            error: error.to_envelope(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::service_error::ServiceError, storage::StorageError};
    use metis_contract::channel::{ChannelSourceType, ChannelStatus};

    struct TestCommandService;

    impl DesktopCommandService for TestCommandService {
        fn channels_list(&self) -> Result<Vec<Channel>, DesktopAdapterError> {
            Ok(vec![Channel {
                id: "channel-1".to_string(),
                title: "One".to_string(),
                source_type: ChannelSourceType::Manual,
                source_ref: None,
                status: ChannelStatus::Active,
                created_at: "2026-01-01T00:00:00Z".to_string(),
                updated_at: "2026-01-01T00:00:00Z".to_string(),
            }])
        }

        fn channels_create(
            &self,
            _request: CreateChannelRequest,
        ) -> Result<Channel, DesktopAdapterError> {
            Err(DesktopAdapterError::Service(ServiceError::validation(
                "invalid channel",
            )))
        }

        fn branches_list_by_channel(
            &self,
            _request: ListBranchesByChannelRequest,
        ) -> Result<Vec<Branch>, DesktopAdapterError> {
            panic!("unexpected call")
        }

        fn tasks_enqueue(&self, _request: EnqueueTaskRequest) -> Result<Task, DesktopAdapterError> {
            panic!("unexpected call")
        }

        fn tasks_list_by_channel(
            &self,
            _request: ListTasksByChannelRequest,
        ) -> Result<Vec<Task>, DesktopAdapterError> {
            Err(DesktopAdapterError::Storage(StorageError::NotImplemented(
                "tasks.list_by_channel",
            )))
        }

        fn workers_list_by_task(
            &self,
            _request: ListWorkersByTaskRequest,
        ) -> Result<Vec<Worker>, DesktopAdapterError> {
            panic!("unexpected call")
        }

        fn history_list_by_channel(
            &self,
            _request: ListHistoryByChannelRequest,
        ) -> Result<Vec<HistoryEvent>, DesktopAdapterError> {
            panic!("unexpected call")
        }

        fn history_list_by_branch(
            &self,
            _request: ListHistoryByBranchRequest,
        ) -> Result<Vec<HistoryEvent>, DesktopAdapterError> {
            panic!("unexpected call")
        }
    }

    #[test]
    fn channels_list_returns_ok_response_shape() {
        let response = handle_channels_list(&TestCommandService);
        let json = serde_json::to_value(response).expect("response should serialize");

        assert_eq!(json["status"], "ok");
        assert_eq!(json["data"][0]["id"], "channel-1");
    }

    #[test]
    fn channels_create_maps_service_error_to_envelope() {
        let response = handle_channels_create(
            &TestCommandService,
            CreateChannelRequest {
                channel: Channel {
                    id: "x".to_string(),
                    title: "x".to_string(),
                    source_type: ChannelSourceType::Manual,
                    source_ref: None,
                    status: ChannelStatus::Active,
                    created_at: "2026-01-01T00:00:00Z".to_string(),
                    updated_at: "2026-01-01T00:00:00Z".to_string(),
                },
            },
        );

        match response {
            CommandResponse::Err { error } => {
                assert_eq!(error.code, "service_validation_error");
            }
            CommandResponse::Ok { .. } => panic!("expected command error response"),
        }
    }

    #[test]
    fn tasks_list_maps_storage_error_to_envelope_code() {
        let response = handle_tasks_list_by_channel(
            &TestCommandService,
            ListTasksByChannelRequest {
                channel_id: "channel-1".to_string(),
            },
        );

        match response {
            CommandResponse::Err { error } => {
                assert_eq!(error.code, "storage_not_implemented");
            }
            CommandResponse::Ok { .. } => panic!("expected command error response"),
        }
    }

    #[test]
    fn stub_service_path_uses_desktop_not_implemented_code() {
        let service = StubDesktopCommandService;
        let response = command_result(service.channels_list());

        match response {
            CommandResponse::Err { error } => {
                assert_eq!(error.code, "desktop_not_implemented");
            }
            CommandResponse::Ok { .. } => panic!("expected command error response"),
        }
    }
}
