use metis_contract::task::{Task, TaskState};

use crate::{
    branches::BranchService, channels::ChannelService, core::service_error::ServiceError,
    storage::repositories::TaskRepository,
};

pub fn is_valid_task_transition(from: TaskState, to: TaskState) -> bool {
    matches!(
        (from, to),
        (TaskState::Queued, TaskState::Running)
            | (TaskState::Queued, TaskState::Cancelled)
            | (TaskState::Running, TaskState::Completed)
            | (TaskState::Running, TaskState::Failed)
            | (TaskState::Running, TaskState::Cancelled)
    )
}

pub trait TaskService {
    fn enqueue(&self, task: Task) -> Result<Task, ServiceError>;
    fn get(&self, task_id: &str) -> Result<Option<Task>, ServiceError>;
    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<Task>, ServiceError>;
    fn update_state(&self, task_id: &str, state: TaskState) -> Result<(), ServiceError>;
}

pub struct TaskDomainService<R, C, B>
where
    R: TaskRepository,
    C: ChannelService,
    B: BranchService,
{
    repository: R,
    channels: C,
    branches: B,
}

impl<R, C, B> TaskDomainService<R, C, B>
where
    R: TaskRepository,
    C: ChannelService,
    B: BranchService,
{
    pub fn new(repository: R, channels: C, branches: B) -> Self {
        Self {
            repository,
            channels,
            branches,
        }
    }
}

impl<R, C, B> TaskService for TaskDomainService<R, C, B>
where
    R: TaskRepository,
    C: ChannelService,
    B: BranchService,
{
    fn enqueue(&self, task: Task) -> Result<Task, ServiceError> {
        if task.id.trim().is_empty() {
            return Err(ServiceError::validation("task id is required"));
        }
        if task.channel_id.trim().is_empty() {
            return Err(ServiceError::validation("task channel_id is required"));
        }
        if task.kind.trim().is_empty() {
            return Err(ServiceError::validation("task kind is required"));
        }

        if self.channels.get_channel(&task.channel_id)?.is_none() {
            return Err(ServiceError::not_found("channel not found for task"));
        }

        if let Some(branch_id) = &task.branch_id {
            let branch_exists = self
                .branches
                .list_by_channel(&task.channel_id)?
                .into_iter()
                .any(|branch| branch.id == *branch_id);
            if !branch_exists {
                return Err(ServiceError::not_found("branch not found for task"));
            }
        }

        self.repository.enqueue(task).map_err(Into::into)
    }

    fn get(&self, task_id: &str) -> Result<Option<Task>, ServiceError> {
        if task_id.trim().is_empty() {
            return Err(ServiceError::validation("task id is required"));
        }
        self.repository.get(task_id).map_err(Into::into)
    }

    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<Task>, ServiceError> {
        if channel_id.trim().is_empty() {
            return Err(ServiceError::validation("channel id is required"));
        }
        self.repository
            .list_by_channel(channel_id)
            .map_err(Into::into)
    }

    fn update_state(&self, task_id: &str, state: TaskState) -> Result<(), ServiceError> {
        let Some(task) = self.get(task_id)? else {
            return Err(ServiceError::not_found("task not found"));
        };

        if !is_valid_task_transition(task.state, state) {
            return Err(ServiceError::conflict("invalid task state transition"));
        }

        self.repository
            .update_state(task_id, state)
            .map_err(Into::into)
    }
}

pub struct StubTaskService;

impl TaskService for StubTaskService {
    fn enqueue(&self, _task: Task) -> Result<Task, ServiceError> {
        Err(ServiceError::internal(
            "tasks are scaffolding-only in phase 1",
        ))
    }

    fn get(&self, _task_id: &str) -> Result<Option<Task>, ServiceError> {
        Err(ServiceError::internal(
            "tasks are scaffolding-only in phase 1",
        ))
    }

    fn list_by_channel(&self, _channel_id: &str) -> Result<Vec<Task>, ServiceError> {
        Err(ServiceError::internal(
            "tasks are scaffolding-only in phase 1",
        ))
    }

    fn update_state(&self, _task_id: &str, _state: TaskState) -> Result<(), ServiceError> {
        Err(ServiceError::internal(
            "tasks are scaffolding-only in phase 1",
        ))
    }
}
