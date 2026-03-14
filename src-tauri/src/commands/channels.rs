use metis_contract::channel::Channel;

use crate::{
    app::requests::{CreateChannelRequest, UpdateChannelStatusRequest},
    app::response::{command_result, CommandResponse},
    app::use_cases::{AppUseCases, CommandUseCases},
};

#[tauri::command]
pub fn desktop_channels_list(
    app_use_cases: tauri::State<'_, AppUseCases>,
) -> CommandResponse<Vec<Channel>> {
    handle_channels_list(app_use_cases.inner())
}

#[tauri::command]
pub fn desktop_channels_create(
    app_use_cases: tauri::State<'_, AppUseCases>,
    request: CreateChannelRequest,
) -> CommandResponse<Channel> {
    handle_channels_create(app_use_cases.inner(), request)
}

#[tauri::command]
pub fn desktop_channels_update_status(
    app_use_cases: tauri::State<'_, AppUseCases>,
    request: UpdateChannelStatusRequest,
) -> CommandResponse<Channel> {
    handle_channels_update_status(app_use_cases.inner(), request)
}

pub(crate) fn handle_channels_list(
    use_cases: &dyn CommandUseCases,
) -> CommandResponse<Vec<Channel>> {
    command_result(use_cases.channels_list())
}

pub(crate) fn handle_channels_create(
    use_cases: &dyn CommandUseCases,
    request: CreateChannelRequest,
) -> CommandResponse<Channel> {
    command_result(use_cases.channels_create(request))
}

pub(crate) fn handle_channels_update_status(
    use_cases: &dyn CommandUseCases,
    request: UpdateChannelStatusRequest,
) -> CommandResponse<Channel> {
    command_result(use_cases.channels_update_status(request))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        app::errors::CommandError,
        app::errors::Error,
        app::requests::{
            AppendHistoryRequest, CreateWorkerRequest, EnqueueTaskRequest,
            ListBranchesByChannelRequest, ListHistoryByBranchRequest, ListHistoryByChannelRequest,
            ListTasksByChannelRequest, ListWorkersByTaskRequest, UpdateChannelStatusRequest,
            UpdateTaskStateRequest, UpdateWorkerStateRequest, WorkerHeartbeatRequest,
        },
        app::use_cases::CommandUseCases,
    };
    use metis_contract::channel::{ChannelSourceType, ChannelStatus};

    struct TestCommandService;

    impl CommandUseCases for TestCommandService {
        fn channels_list(&self) -> Result<Vec<Channel>, CommandError> {
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

        fn channels_create(&self, _request: CreateChannelRequest) -> Result<Channel, CommandError> {
            Err(CommandError::Service(Error::validation("invalid channel")))
        }

        fn channels_update_status(
            &self,
            _request: UpdateChannelStatusRequest,
        ) -> Result<Channel, CommandError> {
            panic!("unexpected call")
        }

        fn branches_list_by_channel(
            &self,
            _request: ListBranchesByChannelRequest,
        ) -> Result<Vec<metis_contract::branch::Branch>, CommandError> {
            panic!("unexpected call")
        }

        fn tasks_enqueue(
            &self,
            _request: EnqueueTaskRequest,
        ) -> Result<metis_contract::task::Task, CommandError> {
            panic!("unexpected call")
        }

        fn tasks_update_state(
            &self,
            _request: UpdateTaskStateRequest,
        ) -> Result<metis_contract::task::Task, CommandError> {
            panic!("unexpected call")
        }

        fn tasks_list_by_channel(
            &self,
            _request: ListTasksByChannelRequest,
        ) -> Result<Vec<metis_contract::task::Task>, CommandError> {
            panic!("unexpected call")
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
            CommandResponse::Err { error } => assert_eq!(error.code, "service_validation_error"),
            CommandResponse::Ok { .. } => panic!("expected command error response"),
        }
    }
}
