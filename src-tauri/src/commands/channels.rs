use metis_contract::channel::{Channel, ChannelStatus};
use serde::{Deserialize, Serialize};

use crate::commands::{
    command_result, emit_best_effort, events::CommandEvent, service::{CommandService, CommandServices}, CommandResponse,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelRequest {
    pub channel: Channel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChannelStatusRequest {
    pub channel_id: String,
    pub status: ChannelStatus,
}

#[tauri::command]
pub fn desktop_channels_list(
    services: tauri::State<'_, CommandServices>,
) -> CommandResponse<Vec<Channel>> {
    handle_channels_list(services.command_service())
}

#[tauri::command]
pub fn desktop_channels_create(
    services: tauri::State<'_, CommandServices>,
    app: tauri::AppHandle,
    request: CreateChannelRequest,
) -> CommandResponse<Channel> {
    handle_channels_create(services.command_service(), request, Some(&app))
}

#[tauri::command]
pub fn desktop_channels_update_status(
    services: tauri::State<'_, CommandServices>,
    app: tauri::AppHandle,
    request: UpdateChannelStatusRequest,
) -> CommandResponse<Channel> {
    handle_channels_update_status(services.command_service(), request, Some(&app))
}

pub(crate) fn handle_channels_list(service: &dyn CommandService) -> CommandResponse<Vec<Channel>> {
    command_result(service.channels_list())
}

pub(crate) fn handle_channels_create(
    service: &dyn CommandService,
    request: CreateChannelRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<Channel> {
    let response = command_result(service.channels_create(request));
    if let Some(event) = event_for_channels_create(&response) {
        emit_best_effort(app, event);
    }
    response
}

pub(crate) fn handle_channels_update_status(
    service: &dyn CommandService,
    request: UpdateChannelStatusRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<Channel> {
    let response = command_result(service.channels_update_status(request));
    if let Some(event) = event_for_channels_update_status(&response) {
        emit_best_effort(app, event);
    }
    response
}

fn event_for_channels_create(response: &CommandResponse<Channel>) -> Option<CommandEvent> {
    match response {
        CommandResponse::Ok { data } => Some(CommandEvent::ChannelCreated(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

fn event_for_channels_update_status(response: &CommandResponse<Channel>) -> Option<CommandEvent> {
    match response {
        CommandResponse::Ok { data } => Some(CommandEvent::ChannelUpdated(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        commands::{errors::CommandError, events::event_name, service::CommandService},
        core::service_error::ServiceError,
    };
    use metis_contract::channel::ChannelSourceType;

    struct TestCommandService;

    impl CommandService for TestCommandService {
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
            Err(CommandError::Service(ServiceError::validation("invalid channel")))
        }

        fn channels_update_status(
            &self,
            _request: UpdateChannelStatusRequest,
        ) -> Result<Channel, CommandError> {
            panic!("unexpected call")
        }

        fn branches_list_by_channel(
            &self,
            _request: crate::commands::branches::ListBranchesByChannelRequest,
        ) -> Result<Vec<metis_contract::branch::Branch>, CommandError> {
            panic!("unexpected call")
        }

        fn tasks_enqueue(
            &self,
            _request: crate::commands::tasks::EnqueueTaskRequest,
        ) -> Result<metis_contract::task::Task, CommandError> {
            panic!("unexpected call")
        }

        fn tasks_update_state(
            &self,
            _request: crate::commands::tasks::UpdateTaskStateRequest,
        ) -> Result<metis_contract::task::Task, CommandError> {
            panic!("unexpected call")
        }

        fn tasks_list_by_channel(
            &self,
            _request: crate::commands::tasks::ListTasksByChannelRequest,
        ) -> Result<Vec<metis_contract::task::Task>, CommandError> {
            panic!("unexpected call")
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
            CommandResponse::Err { error } => assert_eq!(error.code, "service_validation_error"),
            CommandResponse::Ok { .. } => panic!("expected command error response"),
        }
    }

    #[test]
    fn event_routing_maps_expected_channel_event_names() {
        let channel = Channel {
            id: "channel-1".to_string(),
            title: "One".to_string(),
            source_type: ChannelSourceType::Manual,
            source_ref: None,
            status: ChannelStatus::Active,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
        };

        let created = event_for_channels_create(&CommandResponse::Ok {
            data: channel.clone(),
        })
        .expect("channel create should map event");
        let updated = event_for_channels_update_status(&CommandResponse::Ok { data: channel })
            .expect("channel update should map event");

        assert_eq!(event_name(&created), "metis://channel-created");
        assert_eq!(event_name(&updated), "metis://channel-updated");
    }

    #[test]
    fn event_routing_returns_none_for_error_responses() {
        let err = CommandResponse::<Channel>::Err {
            error: metis_contract::error::ErrorEnvelope {
                code: "x".to_string(),
                message: "x".to_string(),
                details: None,
            },
        };

        assert!(event_for_channels_create(&err).is_none());
        assert!(event_for_channels_update_status(&err).is_none());
    }
}
