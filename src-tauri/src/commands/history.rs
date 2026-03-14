use metis_contract::history::HistoryEvent;

use crate::{
    app::requests::{
        AppendHistoryRequest, ListHistoryByBranchRequest, ListHistoryByChannelRequest,
    },
    app::response::{command_result, CommandResponse},
    app::use_cases::{AppUseCases, CommandUseCases},
};

#[tauri::command]
pub fn desktop_history_list_by_channel(
    app_use_cases: tauri::State<'_, AppUseCases>,
    request: ListHistoryByChannelRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    handle_history_list_by_channel(app_use_cases.inner(), request)
}

#[tauri::command]
pub fn desktop_history_list_by_branch(
    app_use_cases: tauri::State<'_, AppUseCases>,
    request: ListHistoryByBranchRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    handle_history_list_by_branch(app_use_cases.inner(), request)
}

#[tauri::command]
pub fn desktop_history_append(
    app_use_cases: tauri::State<'_, AppUseCases>,
    request: AppendHistoryRequest,
) -> CommandResponse<HistoryEvent> {
    handle_history_append(app_use_cases.inner(), request)
}

pub(crate) fn handle_history_list_by_channel(
    use_cases: &dyn CommandUseCases,
    request: ListHistoryByChannelRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    command_result(use_cases.history_list_by_channel(request))
}

pub(crate) fn handle_history_list_by_branch(
    use_cases: &dyn CommandUseCases,
    request: ListHistoryByBranchRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    command_result(use_cases.history_list_by_branch(request))
}

pub(crate) fn handle_history_append(
    use_cases: &dyn CommandUseCases,
    request: AppendHistoryRequest,
) -> CommandResponse<HistoryEvent> {
    command_result(use_cases.history_append(request))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        app::requests::{
            AppendHistoryRequest, CreateChannelRequest, CreateWorkerRequest, EnqueueTaskRequest,
            ListBranchesByChannelRequest, ListHistoryByBranchRequest, ListHistoryByChannelRequest,
            ListTasksByChannelRequest, ListWorkersByTaskRequest, UpdateChannelStatusRequest,
            UpdateTaskStateRequest, UpdateWorkerStateRequest, WorkerHeartbeatRequest,
        },
        app::response::command_result,
        app::service::StubCommandService,
        app::use_cases::CommandUseCases,
    };

    struct RecordingUseCases;

    impl CommandUseCases for RecordingUseCases {
        fn channels_list(
            &self,
        ) -> Result<Vec<metis_contract::channel::Channel>, crate::app::errors::CommandError>
        {
            panic!("unexpected call")
        }

        fn channels_create(
            &self,
            _request: CreateChannelRequest,
        ) -> Result<metis_contract::channel::Channel, crate::app::errors::CommandError> {
            panic!("unexpected call")
        }

        fn channels_update_status(
            &self,
            _request: UpdateChannelStatusRequest,
        ) -> Result<metis_contract::channel::Channel, crate::app::errors::CommandError> {
            panic!("unexpected call")
        }

        fn branches_list_by_channel(
            &self,
            _request: ListBranchesByChannelRequest,
        ) -> Result<Vec<metis_contract::branch::Branch>, crate::app::errors::CommandError> {
            panic!("unexpected call")
        }

        fn tasks_enqueue(
            &self,
            _request: EnqueueTaskRequest,
        ) -> Result<metis_contract::task::Task, crate::app::errors::CommandError> {
            panic!("unexpected call")
        }

        fn tasks_update_state(
            &self,
            _request: UpdateTaskStateRequest,
        ) -> Result<metis_contract::task::Task, crate::app::errors::CommandError> {
            panic!("unexpected call")
        }

        fn tasks_list_by_channel(
            &self,
            _request: ListTasksByChannelRequest,
        ) -> Result<Vec<metis_contract::task::Task>, crate::app::errors::CommandError> {
            panic!("unexpected call")
        }

        fn workers_list_by_task(
            &self,
            _request: ListWorkersByTaskRequest,
        ) -> Result<Vec<metis_contract::worker::Worker>, crate::app::errors::CommandError> {
            panic!("unexpected call")
        }

        fn workers_create(
            &self,
            _request: CreateWorkerRequest,
        ) -> Result<metis_contract::worker::Worker, crate::app::errors::CommandError> {
            panic!("unexpected call")
        }

        fn workers_update_state(
            &self,
            _request: UpdateWorkerStateRequest,
        ) -> Result<metis_contract::worker::Worker, crate::app::errors::CommandError> {
            panic!("unexpected call")
        }

        fn workers_heartbeat(
            &self,
            _request: WorkerHeartbeatRequest,
        ) -> Result<metis_contract::worker::Worker, crate::app::errors::CommandError> {
            panic!("unexpected call")
        }

        fn history_list_by_channel(
            &self,
            _request: ListHistoryByChannelRequest,
        ) -> Result<Vec<HistoryEvent>, crate::app::errors::CommandError> {
            Ok(vec![])
        }

        fn history_list_by_branch(
            &self,
            _request: ListHistoryByBranchRequest,
        ) -> Result<Vec<HistoryEvent>, crate::app::errors::CommandError> {
            Ok(vec![])
        }

        fn history_append(
            &self,
            request: AppendHistoryRequest,
        ) -> Result<HistoryEvent, crate::app::errors::CommandError> {
            Ok(request.event)
        }
    }

    #[test]
    fn history_append_delegates_to_use_cases() {
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

        let response = handle_history_append(
            &RecordingUseCases,
            AppendHistoryRequest {
                event: history.clone(),
            },
        );

        match response {
            CommandResponse::Ok { data } => assert_eq!(data.id, history.id),
            CommandResponse::Err { .. } => panic!("expected success response"),
        }
    }

    #[test]
    fn stub_service_history_append_returns_not_implemented_envelope() {
        let service = StubCommandService;
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

        let response = serde_json::to_value(history_result).expect("serialize history response");
        assert_eq!(response["status"], "err");
        assert_eq!(response["error"]["code"], "desktop_not_implemented");
    }
}
