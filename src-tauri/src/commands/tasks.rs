use metis_contract::task::Task;

use crate::{
    app::requests::{EnqueueTaskRequest, ListTasksByChannelRequest, UpdateTaskStateRequest},
    app::response::{command_result, CommandResponse},
    app::use_cases::{AppUseCases, CommandUseCases},
};

#[tauri::command]
pub fn desktop_tasks_enqueue(
    app_use_cases: tauri::State<'_, AppUseCases>,
    request: EnqueueTaskRequest,
) -> CommandResponse<Task> {
    handle_tasks_enqueue(app_use_cases.inner(), request)
}

#[tauri::command]
pub fn desktop_tasks_update_state(
    app_use_cases: tauri::State<'_, AppUseCases>,
    request: UpdateTaskStateRequest,
) -> CommandResponse<Task> {
    handle_tasks_update_state(app_use_cases.inner(), request)
}

#[tauri::command]
pub fn desktop_tasks_list_by_channel(
    app_use_cases: tauri::State<'_, AppUseCases>,
    request: ListTasksByChannelRequest,
) -> CommandResponse<Vec<Task>> {
    handle_tasks_list_by_channel(app_use_cases.inner(), request)
}

pub(crate) fn handle_tasks_enqueue(
    use_cases: &dyn CommandUseCases,
    request: EnqueueTaskRequest,
) -> CommandResponse<Task> {
    command_result(use_cases.tasks_enqueue(request))
}

pub(crate) fn handle_tasks_update_state(
    use_cases: &dyn CommandUseCases,
    request: UpdateTaskStateRequest,
) -> CommandResponse<Task> {
    command_result(use_cases.tasks_update_state(request))
}

pub(crate) fn handle_tasks_list_by_channel(
    use_cases: &dyn CommandUseCases,
    request: ListTasksByChannelRequest,
) -> CommandResponse<Vec<Task>> {
    command_result(use_cases.tasks_list_by_channel(request))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        app::errors::CommandError,
        app::requests::{
            AppendHistoryRequest, CreateChannelRequest, CreateWorkerRequest, EnqueueTaskRequest,
            ListBranchesByChannelRequest, ListHistoryByBranchRequest, ListHistoryByChannelRequest,
            ListTasksByChannelRequest, ListWorkersByTaskRequest, UpdateChannelStatusRequest,
            UpdateTaskStateRequest, UpdateWorkerStateRequest, WorkerHeartbeatRequest,
        },
        app::service::StubCommandService,
        app::use_cases::CommandUseCases,
        storage::StorageError,
    };
    use metis_contract::task::TaskState;

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

    impl CommandUseCases for ErrorTaskListService {
        fn channels_list(&self) -> Result<Vec<metis_contract::channel::Channel>, CommandError> {
            panic!("unexpected call")
        }

        fn channels_create(
            &self,
            _request: CreateChannelRequest,
        ) -> Result<metis_contract::channel::Channel, CommandError> {
            panic!("unexpected call")
        }

        fn channels_update_status(
            &self,
            _request: UpdateChannelStatusRequest,
        ) -> Result<metis_contract::channel::Channel, CommandError> {
            panic!("unexpected call")
        }

        fn branches_list_by_channel(
            &self,
            _request: ListBranchesByChannelRequest,
        ) -> Result<Vec<metis_contract::branch::Branch>, CommandError> {
            panic!("unexpected call")
        }

        fn tasks_enqueue(&self, _request: EnqueueTaskRequest) -> Result<Task, CommandError> {
            panic!("unexpected call")
        }

        fn tasks_update_state(
            &self,
            _request: UpdateTaskStateRequest,
        ) -> Result<Task, CommandError> {
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
            _request: ListWorkersByTaskRequest,
        ) -> Result<Vec<metis_contract::worker::Worker>, CommandError> {
            panic!("unexpected call")
        }

        fn workers_create(
            &self,
            _request: CreateWorkerRequest,
        ) -> Result<metis_contract::worker::Worker, CommandError> {
            panic!("unexpected call")
        }

        fn workers_update_state(
            &self,
            _request: UpdateWorkerStateRequest,
        ) -> Result<metis_contract::worker::Worker, CommandError> {
            panic!("unexpected call")
        }

        fn workers_heartbeat(
            &self,
            _request: WorkerHeartbeatRequest,
        ) -> Result<metis_contract::worker::Worker, CommandError> {
            panic!("unexpected call")
        }

        fn history_list_by_channel(
            &self,
            _request: ListHistoryByChannelRequest,
        ) -> Result<Vec<metis_contract::history::HistoryEvent>, CommandError> {
            panic!("unexpected call")
        }

        fn history_list_by_branch(
            &self,
            _request: ListHistoryByBranchRequest,
        ) -> Result<Vec<metis_contract::history::HistoryEvent>, CommandError> {
            panic!("unexpected call")
        }

        fn history_append(
            &self,
            _request: AppendHistoryRequest,
        ) -> Result<metis_contract::history::HistoryEvent, CommandError> {
            panic!("unexpected call")
        }
    }
}
