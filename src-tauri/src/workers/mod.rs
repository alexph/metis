use metis_contract::worker::{Worker, WorkerState};

pub fn is_valid_worker_transition(_from: WorkerState, _to: WorkerState) -> bool {
    false
}

pub trait WorkerService {
    fn create_worker(&self, worker: Worker) -> Result<Worker, &'static str>;
}

pub struct StubWorkerService;

impl WorkerService for StubWorkerService {
    fn create_worker(&self, _worker: Worker) -> Result<Worker, &'static str> {
        Err("workers are scaffolding-only in phase 1")
    }
}
