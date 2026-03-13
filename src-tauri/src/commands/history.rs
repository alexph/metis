use metis_contract::history::HistoryEvent;
use serde::{Deserialize, Serialize};

use crate::commands::{
    command_result, emit_best_effort, events::CommandEvent, service::{CommandService, CommandServices}, CommandResponse,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHistoryByChannelRequest {
    pub channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHistoryByBranchRequest {
    pub branch_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendHistoryRequest {
    pub event: HistoryEvent,
}

#[tauri::command]
pub fn desktop_history_list_by_channel(
    services: tauri::State<'_, CommandServices>,
    request: ListHistoryByChannelRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    handle_history_list_by_channel(services.command_service(), request)
}

#[tauri::command]
pub fn desktop_history_list_by_branch(
    services: tauri::State<'_, CommandServices>,
    request: ListHistoryByBranchRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    handle_history_list_by_branch(services.command_service(), request)
}

#[tauri::command]
pub fn desktop_history_append(
    services: tauri::State<'_, CommandServices>,
    app: tauri::AppHandle,
    request: AppendHistoryRequest,
) -> CommandResponse<HistoryEvent> {
    handle_history_append(services.command_service(), request, Some(&app))
}

pub(crate) fn handle_history_list_by_channel(
    service: &dyn CommandService,
    request: ListHistoryByChannelRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    command_result(service.history_list_by_channel(request))
}

pub(crate) fn handle_history_list_by_branch(
    service: &dyn CommandService,
    request: ListHistoryByBranchRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    command_result(service.history_list_by_branch(request))
}

pub(crate) fn handle_history_append(
    service: &dyn CommandService,
    request: AppendHistoryRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<HistoryEvent> {
    let response = command_result(service.history_append(request));
    if let Some(event) = event_for_history_append(&response) {
        emit_best_effort(app, event);
    }
    response
}

fn event_for_history_append(response: &CommandResponse<HistoryEvent>) -> Option<CommandEvent> {
    match response {
        CommandResponse::Ok { data } => Some(CommandEvent::HistoryAppended(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::{command_result, events::event_name, service::StubCommandService};

    #[test]
    fn event_routing_maps_expected_history_event_name() {
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

        let appended = event_for_history_append(&CommandResponse::Ok { data: history })
            .expect("history append should map event");

        assert_eq!(event_name(&appended), "metis://history-appended");
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
