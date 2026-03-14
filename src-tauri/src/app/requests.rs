use metis_contract::{
    channel::{Channel, ChannelStatus},
    history::HistoryEvent,
    task::{Task, TaskState},
    worker::{Worker, WorkerState},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelRequest {
    pub channel: Channel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChannelStatusRequest {
    pub channel_id: String,
    pub status: ChannelStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBranchesByChannelRequest {
    pub channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnqueueTaskRequest {
    pub task: Task,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTasksByChannelRequest {
    pub channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskStateRequest {
    pub task_id: String,
    pub state: TaskState,
}

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
