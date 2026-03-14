use metis_contract::worker::{Worker, WorkerState};

use crate::{app::errors::Error, storage::repositories::WorkerRepository, tasks::TaskService};

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
    fn create_worker(&self, worker: Worker) -> Result<Worker, Error>;
    fn get(&self, worker_id: &str) -> Result<Option<Worker>, Error>;
    fn list_by_task(&self, task_id: &str) -> Result<Vec<Worker>, Error>;
    fn update_state(&self, worker_id: &str, state: WorkerState) -> Result<(), Error>;
    fn heartbeat(&self, worker_id: &str, heartbeat_at: &str) -> Result<(), Error>;
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
    fn create_worker(&self, worker: Worker) -> Result<Worker, Error> {
        if worker.id.trim().is_empty() {
            return Err(Error::validation("worker id is required"));
        }
        if worker.task_id.trim().is_empty() {
            return Err(Error::validation("worker task_id is required"));
        }
        if worker.worker_type.trim().is_empty() {
            return Err(Error::validation("worker worker_type is required"));
        }

        if self.tasks.get(&worker.task_id)?.is_none() {
            return Err(Error::not_found("task not found for worker"));
        }

        self.repository.create(worker).map_err(Into::into)
    }

    fn list_by_task(&self, task_id: &str) -> Result<Vec<Worker>, Error> {
        if task_id.trim().is_empty() {
            return Err(Error::validation("task id is required"));
        }
        self.repository.get_by_task(task_id).map_err(Into::into)
    }

    fn get(&self, worker_id: &str) -> Result<Option<Worker>, Error> {
        if worker_id.trim().is_empty() {
            return Err(Error::validation("worker id is required"));
        }
        self.repository.get(worker_id).map_err(Into::into)
    }

    fn update_state(&self, worker_id: &str, state: WorkerState) -> Result<(), Error> {
        if worker_id.trim().is_empty() {
            return Err(Error::validation("worker id is required"));
        }

        let Some(worker) = self.repository.get(worker_id).map_err(Error::from)? else {
            return Err(Error::not_found("worker not found"));
        };

        if !is_valid_worker_transition(worker.state, state) {
            return Err(Error::conflict("invalid worker state transition"));
        }

        self.repository
            .update_state(worker_id, state)
            .map_err(Into::into)
    }

    fn heartbeat(&self, worker_id: &str, heartbeat_at: &str) -> Result<(), Error> {
        if worker_id.trim().is_empty() {
            return Err(Error::validation("worker id is required"));
        }
        if heartbeat_at.trim().is_empty() {
            return Err(Error::validation("heartbeat timestamp is required"));
        }
        self.repository
            .heartbeat(worker_id, heartbeat_at)
            .map_err(Into::into)
    }
}
