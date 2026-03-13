use metis_contract::worker::{Worker, WorkerState};
use serde::{Deserialize, Serialize};

use crate::commands::{
    command_result, emit_best_effort, events::CommandEvent, service::{CommandService, CommandServices}, CommandResponse,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWorkersByTaskRequest {
    pub task_id: String,
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

#[tauri::command]
pub fn desktop_workers_list_by_task(
    services: tauri::State<'_, CommandServices>,
    request: ListWorkersByTaskRequest,
) -> CommandResponse<Vec<Worker>> {
    handle_workers_list_by_task(services.command_service(), request)
}

#[tauri::command]
pub fn desktop_workers_create(
    services: tauri::State<'_, CommandServices>,
    app: tauri::AppHandle,
    request: CreateWorkerRequest,
) -> CommandResponse<Worker> {
    handle_workers_create(services.command_service(), request, Some(&app))
}

#[tauri::command]
pub fn desktop_workers_update_state(
    services: tauri::State<'_, CommandServices>,
    app: tauri::AppHandle,
    request: UpdateWorkerStateRequest,
) -> CommandResponse<Worker> {
    handle_workers_update_state(services.command_service(), request, Some(&app))
}

#[tauri::command]
pub fn desktop_workers_heartbeat(
    services: tauri::State<'_, CommandServices>,
    app: tauri::AppHandle,
    request: WorkerHeartbeatRequest,
) -> CommandResponse<Worker> {
    handle_workers_heartbeat(services.command_service(), request, Some(&app))
}

pub(crate) fn handle_workers_list_by_task(
    service: &dyn CommandService,
    request: ListWorkersByTaskRequest,
) -> CommandResponse<Vec<Worker>> {
    command_result(service.workers_list_by_task(request))
}

pub(crate) fn handle_workers_create(
    service: &dyn CommandService,
    request: CreateWorkerRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<Worker> {
    let response = command_result(service.workers_create(request));
    if let Some(event) = event_for_workers_create(&response) {
        emit_best_effort(app, event);
    }
    response
}

pub(crate) fn handle_workers_update_state(
    service: &dyn CommandService,
    request: UpdateWorkerStateRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<Worker> {
    let response = command_result(service.workers_update_state(request));
    if let Some(event) = event_for_workers_update_state(&response) {
        emit_best_effort(app, event);
    }
    response
}

pub(crate) fn handle_workers_heartbeat(
    service: &dyn CommandService,
    request: WorkerHeartbeatRequest,
    app: Option<&tauri::AppHandle>,
) -> CommandResponse<Worker> {
    let response = command_result(service.workers_heartbeat(request));
    if let Some(event) = event_for_workers_heartbeat(&response) {
        emit_best_effort(app, event);
    }
    response
}

fn event_for_workers_create(response: &CommandResponse<Worker>) -> Option<CommandEvent> {
    match response {
        CommandResponse::Ok { data } => Some(CommandEvent::WorkerCreated(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

fn event_for_workers_update_state(response: &CommandResponse<Worker>) -> Option<CommandEvent> {
    match response {
        CommandResponse::Ok { data } => Some(CommandEvent::WorkerStateChanged(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

fn event_for_workers_heartbeat(response: &CommandResponse<Worker>) -> Option<CommandEvent> {
    match response {
        CommandResponse::Ok { data } => Some(CommandEvent::WorkerHeartbeat(data.clone())),
        CommandResponse::Err { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::events::event_name;

    #[test]
    fn event_routing_maps_expected_worker_event_names() {
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

        let created = event_for_workers_create(&CommandResponse::Ok {
            data: worker.clone(),
        })
        .expect("worker create should map event");
        let updated = event_for_workers_update_state(&CommandResponse::Ok {
            data: worker.clone(),
        })
        .expect("worker update should map event");
        let heartbeat = event_for_workers_heartbeat(&CommandResponse::Ok { data: worker })
            .expect("worker heartbeat should map event");

        assert_eq!(event_name(&created), "metis://worker-created");
        assert_eq!(event_name(&updated), "metis://worker-state-changed");
        assert_eq!(event_name(&heartbeat), "metis://worker-heartbeat");
    }
}
