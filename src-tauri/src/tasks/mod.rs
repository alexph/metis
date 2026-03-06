use metis_contract::task::{Task, TaskState};

pub fn is_valid_task_transition(_from: TaskState, _to: TaskState) -> bool {
    false
}

pub trait TaskService {
    fn enqueue(&self, task: Task) -> Result<Task, &'static str>;
}

pub struct StubTaskService;

impl TaskService for StubTaskService {
    fn enqueue(&self, _task: Task) -> Result<Task, &'static str> {
        Err("tasks are scaffolding-only in phase 1")
    }
}
