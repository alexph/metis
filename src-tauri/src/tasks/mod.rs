use metis_contract::task::{Task, TaskState};

use crate::{
    app::errors::Error, branches::BranchService, channels::ChannelService,
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
    fn enqueue(&self, task: Task) -> Result<Task, Error>;
    fn get(&self, task_id: &str) -> Result<Option<Task>, Error>;
    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<Task>, Error>;
    fn update_state(&self, task_id: &str, state: TaskState) -> Result<(), Error>;
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
    fn enqueue(&self, task: Task) -> Result<Task, Error> {
        if task.id.trim().is_empty() {
            return Err(Error::validation("task id is required"));
        }
        if task.channel_id.trim().is_empty() {
            return Err(Error::validation("task channel_id is required"));
        }
        if task.kind.trim().is_empty() {
            return Err(Error::validation("task kind is required"));
        }

        if self.channels.get_channel(&task.channel_id)?.is_none() {
            return Err(Error::not_found("channel not found for task"));
        }

        if let Some(branch_id) = &task.branch_id {
            let branch_exists = self
                .branches
                .list_by_channel(&task.channel_id)?
                .into_iter()
                .any(|branch| branch.id == *branch_id);
            if !branch_exists {
                return Err(Error::not_found("branch not found for task"));
            }
        }

        self.repository.enqueue(task).map_err(Into::into)
    }

    fn get(&self, task_id: &str) -> Result<Option<Task>, Error> {
        if task_id.trim().is_empty() {
            return Err(Error::validation("task id is required"));
        }
        self.repository.get(task_id).map_err(Into::into)
    }

    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<Task>, Error> {
        if channel_id.trim().is_empty() {
            return Err(Error::validation("channel id is required"));
        }
        self.repository
            .list_by_channel(channel_id)
            .map_err(Into::into)
    }

    fn update_state(&self, task_id: &str, state: TaskState) -> Result<(), Error> {
        let Some(task) = self.get(task_id)? else {
            return Err(Error::not_found("task not found"));
        };

        if !is_valid_task_transition(task.state, state) {
            return Err(Error::conflict("invalid task state transition"));
        }

        self.repository
            .update_state(task_id, state)
            .map_err(Into::into)
    }
}
