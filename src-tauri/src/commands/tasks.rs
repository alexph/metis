use metis_contract::task::{Task, TaskState};
use serde::{Deserialize, Serialize};

use crate::commands::{
    command_result, emit_best_effort, events::CommandEvent, service::{CommandService, CommandServices}, CommandResponse,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnqueueTaskRequest {
    pub task: Task,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTasksByChannelRequest {
    pub channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskStateRequest {
    pub task_id: String,
    pub state: TaskState,
}

#[tauri::command]
pub fn desktop_tasks_enqueue(
    services: tauri::State<'_, CommandServices>,
    app: tauri::AppHandle,
    request: EnqueueTaskRequest,
) -> CommandResponse<Task> {
    handle_tasks_enqueue(services.command_service(), request, Some(&app))
}

#[tauri::command]
pub fn desktop_tasks_update_state(
    services: tauri::State<'_, CommandServices>,
    app: tauri::AppHandle,
    request: UpdateTaskStateRequest,
) -> CommandResponse<Task> {
    handle_tasks_update_state(services.command_service(), request, Some(&app))
}

#[tauri::command]
pub fn desktop_tasks_list_by_channel(
    services: tauri::State<'_, CommandServices>,
    request: ListTasksByChannelRequest,
) -> CommandResponse<Vec<Task>> {
    handle_tasks_list_by_channel(services.command_service(), request)
}

pub(crate) fn handle_tasks_enqueue(
    service: &dyn CommandService,
    request: EnqueueTaskRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<Task> {
    let response = command_result(service.tasks_enqueue(request));
    if let Some(event) = event_for_tasks_enqueue(&response) {
        emit_best_effort(app, event);
    }
    response
}

pub(crate) fn handle_tasks_update_state(
    service: &dyn CommandService,
    request: UpdateTaskStateRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<Task> {
    let response = command_result(service.tasks_update_state(request));
    if let Some(event) = event_for_tasks_update_state(&response) {
        emit_best_effort(app, event);
    }
    response
}

pub(crate) fn handle_tasks_list_by_channel(
    service: &dyn CommandService,
    request: ListTasksByChannelRequest,
) -> CommandResponse<Vec<Task>> {
    command_result(service.tasks_list_by_channel(request))
}

fn event_for_tasks_enqueue(response: &CommandResponse<Task>) -> Option<CommandEvent> {
    match response {
        CommandResponse::Ok { data } => Some(CommandEvent::TaskEnqueued(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

fn event_for_tasks_update_state(response: &CommandResponse<Task>) -> Option<CommandEvent> {
    match response {
        CommandResponse::Ok { data } => Some(CommandEvent::TaskStateChanged(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{commands::{errors::CommandError, service::{CommandService, StubCommandService}}, storage::StorageError};

    #[test]
    fn tasks_list_maps_storage_error_to_envelope_code() {
        let response = handle_tasks_list_by_channel(
            &ErrorTaskListService,
            ListTasksByChannelRequest {
                channel_id: "channel-1".to_string(),
            },
        );

        match response {
            CommandResponse::Err { error } => assert_eq!(error.code, "storage_not_implemented"),
            CommandResponse::Ok { .. } => panic!("expected command error response"),
        }
    }

    #[test]
    fn stub_service_new_mutations_return_not_implemented_envelope() {
        let service = StubCommandService;

        let task_result = command_result(service.tasks_update_state(UpdateTaskStateRequest {
            task_id: "task-1".to_string(),
            state: TaskState::Running,
        }));

        let response = serde_json::to_value(task_result).expect("serialize task response");
        assert_eq!(response["status"], "err");
        assert_eq!(response["error"]["code"], "desktop_not_implemented");
    }

    struct ErrorTaskListService;

    impl CommandService for ErrorTaskListService {
        fn channels_list(&self) -> Result<Vec<metis_contract::channel::Channel>, CommandError> {
            panic!("unexpected call")
        }

        fn channels_create(
            &self,
            _request: crate::commands::channels::CreateChannelRequest,
        ) -> Result<metis_contract::channel::Channel, CommandError> {
            panic!("unexpected call")
        }

        fn channels_update_status(
            &self,
            _request: crate::commands::channels::UpdateChannelStatusRequest,
        ) -> Result<metis_contract::channel::Channel, CommandError> {
            panic!("unexpected call")
        }

        fn branches_list_by_channel(
            &self,
            _request: crate::commands::branches::ListBranchesByChannelRequest,
        ) -> Result<Vec<metis_contract::branch::Branch>, CommandError> {
            panic!("unexpected call")
        }

        fn tasks_enqueue(&self, _request: EnqueueTaskRequest) -> Result<Task, CommandError> {
            panic!("unexpected call")
        }

        fn tasks_update_state(&self, _request: UpdateTaskStateRequest) -> Result<Task, CommandError> {
            panic!("unexpected call")
        }

        fn tasks_list_by_channel(
            &self,
            _request: ListTasksByChannelRequest,
        ) -> Result<Vec<Task>, CommandError> {
            Err(CommandError::Storage(StorageError::NotImplemented(
                "tasks.list_by_channel",
            )))
        }

        fn workers_list_by_task(
            &self,
            _request: crate::commands::workers::ListWorkersByTaskRequest,
        ) -> Result<Vec<metis_contract::worker::Worker>, CommandError> {
            panic!("unexpected call")
        }

        fn workers_create(
            &self,
            _request: crate::commands::workers::CreateWorkerRequest,
        ) -> Result<metis_contract::worker::Worker, CommandError> {
            panic!("unexpected call")
        }

        fn workers_update_state(
            &self,
            _request: crate::commands::workers::UpdateWorkerStateRequest,
        ) -> Result<metis_contract::worker::Worker, CommandError> {
            panic!("unexpected call")
        }

        fn workers_heartbeat(
            &self,
            _request: crate::commands::workers::WorkerHeartbeatRequest,
        ) -> Result<metis_contract::worker::Worker, CommandError> {
            panic!("unexpected call")
        }

        fn history_list_by_channel(
            &self,
            _request: crate::commands::history::ListHistoryByChannelRequest,
        ) -> Result<Vec<metis_contract::history::HistoryEvent>, CommandError> {
            panic!("unexpected call")
        }

        fn history_list_by_branch(
            &self,
            _request: crate::commands::history::ListHistoryByBranchRequest,
        ) -> Result<Vec<metis_contract::history::HistoryEvent>, CommandError> {
            panic!("unexpected call")
        }

        fn history_append(
            &self,
            _request: crate::commands::history::AppendHistoryRequest,
        ) -> Result<metis_contract::history::HistoryEvent, CommandError> {
            panic!("unexpected call")
        }
    }
}
