use metis_contract::worker::Worker;

use crate::{
    app::requests::{
        CreateWorkerRequest, ListWorkersByTaskRequest, UpdateWorkerStateRequest,
        WorkerHeartbeatRequest,
    },
    app::response::{command_result, CommandResponse},
    app::use_cases::{AppUseCases, CommandUseCases},
};

#[tauri::command]
pub fn desktop_workers_list_by_task(
    app_use_cases: tauri::State<'_, AppUseCases>,
    request: ListWorkersByTaskRequest,
) -> CommandResponse<Vec<Worker>> {
    handle_workers_list_by_task(app_use_cases.inner(), request)
}

#[tauri::command]
pub fn desktop_workers_create(
    app_use_cases: tauri::State<'_, AppUseCases>,
    request: CreateWorkerRequest,
) -> CommandResponse<Worker> {
    handle_workers_create(app_use_cases.inner(), request)
}

#[tauri::command]
pub fn desktop_workers_update_state(
    app_use_cases: tauri::State<'_, AppUseCases>,
    request: UpdateWorkerStateRequest,
) -> CommandResponse<Worker> {
    handle_workers_update_state(app_use_cases.inner(), request)
}

#[tauri::command]
pub fn desktop_workers_heartbeat(
    app_use_cases: tauri::State<'_, AppUseCases>,
    request: WorkerHeartbeatRequest,
) -> CommandResponse<Worker> {
    handle_workers_heartbeat(app_use_cases.inner(), request)
}

pub(crate) fn handle_workers_list_by_task(
    use_cases: &dyn CommandUseCases,
    request: ListWorkersByTaskRequest,
) -> CommandResponse<Vec<Worker>> {
    command_result(use_cases.workers_list_by_task(request))
}

pub(crate) fn handle_workers_create(
    use_cases: &dyn CommandUseCases,
    request: CreateWorkerRequest,
) -> CommandResponse<Worker> {
    command_result(use_cases.workers_create(request))
}

pub(crate) fn handle_workers_update_state(
    use_cases: &dyn CommandUseCases,
    request: UpdateWorkerStateRequest,
) -> CommandResponse<Worker> {
    command_result(use_cases.workers_update_state(request))
}

pub(crate) fn handle_workers_heartbeat(
    use_cases: &dyn CommandUseCases,
    request: WorkerHeartbeatRequest,
) -> CommandResponse<Worker> {
    command_result(use_cases.workers_heartbeat(request))
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
        app::use_cases::CommandUseCases,
    };
    use metis_contract::worker::WorkerState;

    struct WorkerUseCases;

    impl CommandUseCases for WorkerUseCases {
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
        ) -> Result<Vec<Worker>, crate::app::errors::CommandError> {
            Ok(vec![])
        }

        fn workers_create(
            &self,
            request: CreateWorkerRequest,
        ) -> Result<Worker, crate::app::errors::CommandError> {
            Ok(request.worker)
        }

        fn workers_update_state(
            &self,
            _request: UpdateWorkerStateRequest,
        ) -> Result<Worker, crate::app::errors::CommandError> {
            Err(crate::app::errors::CommandError::NotImplemented(
                "workers.update_state",
            ))
        }

        fn workers_heartbeat(
            &self,
            request: WorkerHeartbeatRequest,
        ) -> Result<Worker, crate::app::errors::CommandError> {
            Ok(Worker {
                id: request.worker_id,
                task_id: "task-1".to_string(),
                worker_type: "agent".to_string(),
                state: WorkerState::Running,
                attempt: 0,
                last_heartbeat_at: Some(request.heartbeat_at),
                started_at: None,
                finished_at: None,
                created_at: "2026-01-01T00:00:00Z".to_string(),
                updated_at: "2026-01-01T00:00:00Z".to_string(),
            })
        }

        fn history_list_by_channel(
            &self,
            _request: ListHistoryByChannelRequest,
        ) -> Result<Vec<metis_contract::history::HistoryEvent>, crate::app::errors::CommandError>
        {
            panic!("unexpected call")
        }

        fn history_list_by_branch(
            &self,
            _request: ListHistoryByBranchRequest,
        ) -> Result<Vec<metis_contract::history::HistoryEvent>, crate::app::errors::CommandError>
        {
            panic!("unexpected call")
        }

        fn history_append(
            &self,
            _request: AppendHistoryRequest,
        ) -> Result<metis_contract::history::HistoryEvent, crate::app::errors::CommandError>
        {
            panic!("unexpected call")
        }
    }

    #[test]
    fn workers_handlers_delegate_to_use_cases() {
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

        let created = handle_workers_create(
            &WorkerUseCases,
            CreateWorkerRequest {
                worker: worker.clone(),
            },
        );
        match created {
            CommandResponse::Ok { data } => assert_eq!(data.id, "worker-1"),
            CommandResponse::Err { .. } => panic!("expected worker create success"),
        }

        let heartbeat = handle_workers_heartbeat(
            &WorkerUseCases,
            WorkerHeartbeatRequest {
                worker_id: "worker-1".to_string(),
                heartbeat_at: "2026-01-01T00:01:00Z".to_string(),
            },
        );

        match heartbeat {
            CommandResponse::Ok { data } => {
                assert_eq!(data.id, "worker-1");
                assert_eq!(
                    data.last_heartbeat_at.as_deref(),
                    Some("2026-01-01T00:01:00Z")
                );
            }
            CommandResponse::Err { .. } => panic!("expected heartbeat success"),
        }
    }
}
