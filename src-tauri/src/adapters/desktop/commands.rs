use metis_contract::{
    branch::Branch,
    channel::{Channel, ChannelStatus},
    error::ErrorEnvelope,
    history::HistoryEvent,
    task::{Task, TaskState},
    worker::{Worker, WorkerState},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::adapters::desktop::errors::DesktopAdapterError;
use crate::adapters::desktop::events::{emit_tauri_event, DesktopEvent};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChannelStatusRequest {
    pub channel_id: String,
    pub status: ChannelStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskStateRequest {
    pub task_id: String,
    pub state: TaskState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkerRequest {
    pub worker: Worker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkerStateRequest {
    pub worker_id: String,
    pub state: WorkerState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerHeartbeatRequest {
    pub worker_id: String,
    pub heartbeat_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendHistoryRequest {
    pub event: HistoryEvent,
}

pub trait DesktopCommandService: Send + Sync {
    fn channels_list(&self) -> Result<Vec<Channel>, DesktopAdapterError>;
    fn channels_create(
        &self,
        request: CreateChannelRequest,
    ) -> Result<Channel, DesktopAdapterError>;
    fn channels_update_status(
        &self,
        request: UpdateChannelStatusRequest,
    ) -> Result<Channel, DesktopAdapterError>;
    fn branches_list_by_channel(
        &self,
        request: ListBranchesByChannelRequest,
    ) -> Result<Vec<Branch>, DesktopAdapterError>;
    fn tasks_enqueue(&self, request: EnqueueTaskRequest) -> Result<Task, DesktopAdapterError>;
    fn tasks_update_state(
        &self,
        request: UpdateTaskStateRequest,
    ) -> Result<Task, DesktopAdapterError>;
    fn tasks_list_by_channel(
        &self,
        request: ListTasksByChannelRequest,
    ) -> Result<Vec<Task>, DesktopAdapterError>;
    fn workers_list_by_task(
        &self,
        request: ListWorkersByTaskRequest,
    ) -> Result<Vec<Worker>, DesktopAdapterError>;
    fn workers_create(&self, request: CreateWorkerRequest) -> Result<Worker, DesktopAdapterError>;
    fn workers_update_state(
        &self,
        request: UpdateWorkerStateRequest,
    ) -> Result<Worker, DesktopAdapterError>;
    fn workers_heartbeat(
        &self,
        request: WorkerHeartbeatRequest,
    ) -> Result<Worker, DesktopAdapterError>;
    fn history_list_by_channel(
        &self,
        request: ListHistoryByChannelRequest,
    ) -> Result<Vec<HistoryEvent>, DesktopAdapterError>;
    fn history_list_by_branch(
        &self,
        request: ListHistoryByBranchRequest,
    ) -> Result<Vec<HistoryEvent>, DesktopAdapterError>;
    fn history_append(
        &self,
        request: AppendHistoryRequest,
    ) -> Result<HistoryEvent, DesktopAdapterError>;
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

    fn channels_update_status(
        &self,
        _request: UpdateChannelStatusRequest,
    ) -> Result<Channel, DesktopAdapterError> {
        Err(DesktopAdapterError::NotImplemented(
            "channels.update_status",
        ))
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

    fn tasks_update_state(
        &self,
        _request: UpdateTaskStateRequest,
    ) -> Result<Task, DesktopAdapterError> {
        Err(DesktopAdapterError::NotImplemented("tasks.update_state"))
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

    fn workers_create(&self, _request: CreateWorkerRequest) -> Result<Worker, DesktopAdapterError> {
        Err(DesktopAdapterError::NotImplemented("workers.create"))
    }

    fn workers_update_state(
        &self,
        _request: UpdateWorkerStateRequest,
    ) -> Result<Worker, DesktopAdapterError> {
        Err(DesktopAdapterError::NotImplemented("workers.update_state"))
    }

    fn workers_heartbeat(
        &self,
        _request: WorkerHeartbeatRequest,
    ) -> Result<Worker, DesktopAdapterError> {
        Err(DesktopAdapterError::NotImplemented("workers.heartbeat"))
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

    fn history_append(
        &self,
        _request: AppendHistoryRequest,
    ) -> Result<HistoryEvent, DesktopAdapterError> {
        Err(DesktopAdapterError::NotImplemented("history.append"))
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
    app: tauri::AppHandle,
    request: CreateChannelRequest,
) -> CommandResponse<Channel> {
    handle_channels_create(services.command_service(), request, Some(&app))
}

#[tauri::command]
pub fn desktop_channels_update_status(
    services: tauri::State<'_, DesktopCommandServices>,
    app: tauri::AppHandle,
    request: UpdateChannelStatusRequest,
) -> CommandResponse<Channel> {
    handle_channels_update_status(services.command_service(), request, Some(&app))
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
    app: tauri::AppHandle,
    request: EnqueueTaskRequest,
) -> CommandResponse<Task> {
    handle_tasks_enqueue(services.command_service(), request, Some(&app))
}

#[tauri::command]
pub fn desktop_tasks_update_state(
    services: tauri::State<'_, DesktopCommandServices>,
    app: tauri::AppHandle,
    request: UpdateTaskStateRequest,
) -> CommandResponse<Task> {
    handle_tasks_update_state(services.command_service(), request, Some(&app))
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
pub fn desktop_workers_create(
    services: tauri::State<'_, DesktopCommandServices>,
    app: tauri::AppHandle,
    request: CreateWorkerRequest,
) -> CommandResponse<Worker> {
    handle_workers_create(services.command_service(), request, Some(&app))
}

#[tauri::command]
pub fn desktop_workers_update_state(
    services: tauri::State<'_, DesktopCommandServices>,
    app: tauri::AppHandle,
    request: UpdateWorkerStateRequest,
) -> CommandResponse<Worker> {
    handle_workers_update_state(services.command_service(), request, Some(&app))
}

#[tauri::command]
pub fn desktop_workers_heartbeat(
    services: tauri::State<'_, DesktopCommandServices>,
    app: tauri::AppHandle,
    request: WorkerHeartbeatRequest,
) -> CommandResponse<Worker> {
    handle_workers_heartbeat(services.command_service(), request, Some(&app))
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

#[tauri::command]
pub fn desktop_history_append(
    services: tauri::State<'_, DesktopCommandServices>,
    app: tauri::AppHandle,
    request: AppendHistoryRequest,
) -> CommandResponse<HistoryEvent> {
    handle_history_append(services.command_service(), request, Some(&app))
}

fn handle_channels_list(service: &dyn DesktopCommandService) -> CommandResponse<Vec<Channel>> {
    command_result(service.channels_list())
}

fn handle_channels_create(
    service: &dyn DesktopCommandService,
    request: CreateChannelRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<Channel> {
    let response = command_result(service.channels_create(request));
    if let Some(event) = event_for_channels_create(&response) {
        emit_best_effort(app, event);
    }
    response
}

fn handle_channels_update_status(
    service: &dyn DesktopCommandService,
    request: UpdateChannelStatusRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<Channel> {
    let response = command_result(service.channels_update_status(request));
    if let Some(event) = event_for_channels_update_status(&response) {
        emit_best_effort(app, event);
    }
    response
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
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<Task> {
    let response = command_result(service.tasks_enqueue(request));
    if let Some(event) = event_for_tasks_enqueue(&response) {
        emit_best_effort(app, event);
    }
    response
}

fn handle_tasks_update_state(
    service: &dyn DesktopCommandService,
    request: UpdateTaskStateRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<Task> {
    let response = command_result(service.tasks_update_state(request));
    if let Some(event) = event_for_tasks_update_state(&response) {
        emit_best_effort(app, event);
    }
    response
}

fn emit_best_effort(app: Option<&tauri::AppHandle>, event: DesktopEvent) {
    let Some(app) = app else {
        return;
    };

    let event_name = crate::adapters::desktop::events::event_name(&event);
    if let Err(error) = emit_tauri_event(app, event) {
        tracing::warn!(
            event_name,
            code = %error.code,
            message = %error.message,
            "desktop event emission failed"
        );
    }
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

fn handle_workers_create(
    service: &dyn DesktopCommandService,
    request: CreateWorkerRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<Worker> {
    let response = command_result(service.workers_create(request));
    if let Some(event) = event_for_workers_create(&response) {
        emit_best_effort(app, event);
    }
    response
}

fn handle_workers_update_state(
    service: &dyn DesktopCommandService,
    request: UpdateWorkerStateRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<Worker> {
    let response = command_result(service.workers_update_state(request));
    if let Some(event) = event_for_workers_update_state(&response) {
        emit_best_effort(app, event);
    }
    response
}

fn handle_workers_heartbeat(
    service: &dyn DesktopCommandService,
    request: WorkerHeartbeatRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<Worker> {
    let response = command_result(service.workers_heartbeat(request));
    if let Some(event) = event_for_workers_heartbeat(&response) {
        emit_best_effort(app, event);
    }
    response
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

fn handle_history_append(
    service: &dyn DesktopCommandService,
    request: AppendHistoryRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<HistoryEvent> {
    let response = command_result(service.history_append(request));
    if let Some(event) = event_for_history_append(&response) {
        emit_best_effort(app, event);
    }
    response
}

fn event_for_channels_create(response: &CommandResponse<Channel>) -> Option<DesktopEvent> {
    match response {
        CommandResponse::Ok { data } => Some(DesktopEvent::ChannelCreated(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

fn event_for_channels_update_status(response: &CommandResponse<Channel>) -> Option<DesktopEvent> {
    match response {
        CommandResponse::Ok { data } => Some(DesktopEvent::ChannelUpdated(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

fn event_for_tasks_enqueue(response: &CommandResponse<Task>) -> Option<DesktopEvent> {
    match response {
        CommandResponse::Ok { data } => Some(DesktopEvent::TaskEnqueued(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

fn event_for_tasks_update_state(response: &CommandResponse<Task>) -> Option<DesktopEvent> {
    match response {
        CommandResponse::Ok { data } => Some(DesktopEvent::TaskStateChanged(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

fn event_for_workers_create(response: &CommandResponse<Worker>) -> Option<DesktopEvent> {
    match response {
        CommandResponse::Ok { data } => Some(DesktopEvent::WorkerCreated(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

fn event_for_workers_update_state(response: &CommandResponse<Worker>) -> Option<DesktopEvent> {
    match response {
        CommandResponse::Ok { data } => Some(DesktopEvent::WorkerStateChanged(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

fn event_for_workers_heartbeat(response: &CommandResponse<Worker>) -> Option<DesktopEvent> {
    match response {
        CommandResponse::Ok { data } => Some(DesktopEvent::WorkerHeartbeat(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

fn event_for_history_append(response: &CommandResponse<HistoryEvent>) -> Option<DesktopEvent> {
    match response {
        CommandResponse::Ok { data } => Some(DesktopEvent::HistoryAppended(data.clone())),
        CommandResponse::Err { .. } => None,
    }
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
    use metis_contract::task::TaskState;
    use metis_contract::worker::WorkerState;

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

        fn channels_update_status(
            &self,
            _request: UpdateChannelStatusRequest,
        ) -> Result<Channel, DesktopAdapterError> {
            panic!("unexpected call")
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

        fn tasks_update_state(
            &self,
            _request: UpdateTaskStateRequest,
        ) -> Result<Task, DesktopAdapterError> {
            panic!("unexpected call")
        }

        fn workers_list_by_task(
            &self,
            _request: ListWorkersByTaskRequest,
        ) -> Result<Vec<Worker>, DesktopAdapterError> {
            panic!("unexpected call")
        }

        fn workers_create(
            &self,
            _request: CreateWorkerRequest,
        ) -> Result<Worker, DesktopAdapterError> {
            panic!("unexpected call")
        }

        fn workers_update_state(
            &self,
            _request: UpdateWorkerStateRequest,
        ) -> Result<Worker, DesktopAdapterError> {
            panic!("unexpected call")
        }

        fn workers_heartbeat(
            &self,
            _request: WorkerHeartbeatRequest,
        ) -> Result<Worker, DesktopAdapterError> {
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

        fn history_append(
            &self,
            _request: AppendHistoryRequest,
        ) -> Result<HistoryEvent, DesktopAdapterError> {
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
            None,
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

    #[test]
    fn event_routing_maps_expected_event_names() {
        let channel = Channel {
            id: "channel-1".to_string(),
            title: "One".to_string(),
            source_type: ChannelSourceType::Manual,
            source_ref: None,
            status: ChannelStatus::Active,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
        };
        let task = Task {
            id: "task-1".to_string(),
            channel_id: "channel-1".to_string(),
            branch_id: None,
            kind: "analysis".to_string(),
            state: TaskState::Queued,
            priority: 0,
            payload_json: None,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
            started_at: None,
            finished_at: None,
        };
        let worker = Worker {
            id: "worker-1".to_string(),
            task_id: "task-1".to_string(),
            worker_type: "agent".to_string(),
            state: WorkerState::Pending,
            attempt: 0,
            last_heartbeat_at: None,
            started_at: None,
            finished_at: None,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
        };
        let history = HistoryEvent {
            id: "hist-1".to_string(),
            channel_id: "channel-1".to_string(),
            branch_id: None,
            task_id: None,
            worker_id: None,
            event_type: "message".to_string(),
            role: None,
            content_json: "{\"text\":\"hello\"}".to_string(),
            correlation_id: None,
            created_at: "2026-01-01T00:00:00Z".to_string(),
        };

        let created = event_for_channels_create(&CommandResponse::Ok {
            data: channel.clone(),
        })
        .expect("channel create should map event");
        let updated = event_for_channels_update_status(&CommandResponse::Ok { data: channel })
            .expect("channel update should map event");
        let enqueued = event_for_tasks_enqueue(&CommandResponse::Ok { data: task.clone() })
            .expect("task enqueue should map event");
        let task_changed = event_for_tasks_update_state(&CommandResponse::Ok { data: task })
            .expect("task state should map event");
        let worker_created = event_for_workers_create(&CommandResponse::Ok {
            data: worker.clone(),
        })
        .expect("worker create should map event");
        let worker_changed = event_for_workers_update_state(&CommandResponse::Ok {
            data: worker.clone(),
        })
        .expect("worker update should map event");
        let worker_heartbeat = event_for_workers_heartbeat(&CommandResponse::Ok { data: worker })
            .expect("worker heartbeat should map event");
        let history_appended = event_for_history_append(&CommandResponse::Ok { data: history })
            .expect("history append should map event");

        assert_eq!(
            crate::adapters::desktop::events::event_name(&created),
            "metis://channel-created"
        );
        assert_eq!(
            crate::adapters::desktop::events::event_name(&updated),
            "metis://channel-updated"
        );
        assert_eq!(
            crate::adapters::desktop::events::event_name(&enqueued),
            "metis://task-enqueued"
        );
        assert_eq!(
            crate::adapters::desktop::events::event_name(&task_changed),
            "metis://task-state-changed"
        );
        assert_eq!(
            crate::adapters::desktop::events::event_name(&worker_created),
            "metis://worker-created"
        );
        assert_eq!(
            crate::adapters::desktop::events::event_name(&worker_changed),
            "metis://worker-state-changed"
        );
        assert_eq!(
            crate::adapters::desktop::events::event_name(&worker_heartbeat),
            "metis://worker-heartbeat"
        );
        assert_eq!(
            crate::adapters::desktop::events::event_name(&history_appended),
            "metis://history-appended"
        );
    }

    #[test]
    fn event_routing_returns_none_for_error_responses() {
        let err = CommandResponse::<Channel>::Err {
            error: ErrorEnvelope {
                code: "x".to_string(),
                message: "x".to_string(),
                details: None,
            },
        };
        assert!(event_for_channels_create(&err).is_none());
        assert!(event_for_channels_update_status(&err).is_none());
    }

    #[test]
    fn stub_service_new_mutations_return_not_implemented_envelope() {
        let service = StubDesktopCommandService;

        let channel_result =
            command_result(service.channels_update_status(UpdateChannelStatusRequest {
                channel_id: "channel-1".to_string(),
                status: ChannelStatus::Archived,
            }));
        let task_result = command_result(service.tasks_update_state(UpdateTaskStateRequest {
            task_id: "task-1".to_string(),
            state: TaskState::Running,
        }));
        let history_result = command_result(service.history_append(AppendHistoryRequest {
            event: HistoryEvent {
                id: "hist-1".to_string(),
                channel_id: "channel-1".to_string(),
                branch_id: None,
                task_id: None,
                worker_id: None,
                event_type: "message".to_string(),
                role: None,
                content_json: "{}".to_string(),
                correlation_id: None,
                created_at: "2026-01-01T00:00:00Z".to_string(),
            },
        }));

        for response in [
            serde_json::to_value(channel_result).expect("serialize channel response"),
            serde_json::to_value(task_result).expect("serialize task response"),
            serde_json::to_value(history_result).expect("serialize history response"),
        ] {
            assert_eq!(response["status"], "err");
            assert_eq!(response["error"]["code"], "desktop_not_implemented");
        }
    }
}
