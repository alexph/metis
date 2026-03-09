use metis_contract::worker::{Worker, WorkerState};

use crate::{
    core::service_error::ServiceError, storage::repositories::WorkerRepository, tasks::TaskService,
};

pub fn is_valid_worker_transition(from: WorkerState, to: WorkerState) -> bool {
    matches!(
        (from, to),
        (WorkerState::Pending, WorkerState::Running)
            | (WorkerState::Pending, WorkerState::Stopped)
            | (WorkerState::Running, WorkerState::Completed)
            | (WorkerState::Running, WorkerState::Failed)
            | (WorkerState::Running, WorkerState::Stopped)
    )
}

pub trait WorkerService {
    fn create_worker(&self, worker: Worker) -> Result<Worker, ServiceError>;
    fn list_by_task(&self, task_id: &str) -> Result<Vec<Worker>, ServiceError>;
    fn update_state(&self, worker_id: &str, state: WorkerState) -> Result<(), ServiceError>;
    fn heartbeat(&self, worker_id: &str, heartbeat_at: &str) -> Result<(), ServiceError>;
}

pub struct WorkerDomainService<R, T>
where
    R: WorkerRepository,
    T: TaskService,
{
    repository: R,
    tasks: T,
}

impl<R, T> WorkerDomainService<R, T>
where
    R: WorkerRepository,
    T: TaskService,
{
    pub fn new(repository: R, tasks: T) -> Self {
        Self { repository, tasks }
    }
}

impl<R, T> WorkerService for WorkerDomainService<R, T>
where
    R: WorkerRepository,
    T: TaskService,
{
    fn create_worker(&self, worker: Worker) -> Result<Worker, ServiceError> {
        if worker.id.trim().is_empty() {
            return Err(ServiceError::validation("worker id is required"));
        }
        if worker.task_id.trim().is_empty() {
            return Err(ServiceError::validation("worker task_id is required"));
        }
        if worker.worker_type.trim().is_empty() {
            return Err(ServiceError::validation("worker worker_type is required"));
        }

        if self.tasks.get(&worker.task_id)?.is_none() {
            return Err(ServiceError::not_found("task not found for worker"));
        }

        self.repository.create(worker).map_err(Into::into)
    }

    fn list_by_task(&self, task_id: &str) -> Result<Vec<Worker>, ServiceError> {
        if task_id.trim().is_empty() {
            return Err(ServiceError::validation("task id is required"));
        }
        self.repository.get_by_task(task_id).map_err(Into::into)
    }

    fn update_state(&self, worker_id: &str, state: WorkerState) -> Result<(), ServiceError> {
        if worker_id.trim().is_empty() {
            return Err(ServiceError::validation("worker id is required"));
        }

        let Some(worker) = self.repository.get(worker_id).map_err(ServiceError::from)? else {
            return Err(ServiceError::not_found("worker not found"));
        };

        if !is_valid_worker_transition(worker.state, state) {
            return Err(ServiceError::conflict("invalid worker state transition"));
        }

        self.repository
            .update_state(worker_id, state)
            .map_err(Into::into)
    }

    fn heartbeat(&self, worker_id: &str, heartbeat_at: &str) -> Result<(), ServiceError> {
        if worker_id.trim().is_empty() {
            return Err(ServiceError::validation("worker id is required"));
        }
        if heartbeat_at.trim().is_empty() {
            return Err(ServiceError::validation("heartbeat timestamp is required"));
        }
        self.repository
            .heartbeat(worker_id, heartbeat_at)
            .map_err(Into::into)
    }
}
