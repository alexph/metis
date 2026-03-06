use metis_contract::{
    branch::Branch, channel::Channel, error::ErrorEnvelope, history::HistoryEvent, task::Task,
    worker::Worker,
};
use serde::{Deserialize, Serialize};

use crate::adapters::desktop::errors::DesktopAdapterError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum CommandResponse<T> {
    Ok { data: T },
    Err { error: ErrorEnvelope },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelRequest {
    pub channel: Channel,
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
pub struct ListWorkersByTaskRequest {
    pub task_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHistoryByChannelRequest {
    pub channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListHistoryByBranchRequest {
    pub branch_id: String,
}

#[tauri::command]
pub async fn desktop_channels_list() -> CommandResponse<Vec<Channel>> {
    command_not_implemented("channels.list")
}

#[tauri::command]
pub async fn desktop_channels_create(_request: CreateChannelRequest) -> CommandResponse<Channel> {
    command_not_implemented("channels.create")
}

#[tauri::command]
pub async fn desktop_branches_list_by_channel(
    _request: ListBranchesByChannelRequest,
) -> CommandResponse<Vec<Branch>> {
    command_not_implemented("branches.list_by_channel")
}

#[tauri::command]
pub async fn desktop_tasks_enqueue(_request: EnqueueTaskRequest) -> CommandResponse<Task> {
    command_not_implemented("tasks.enqueue")
}

#[tauri::command]
pub async fn desktop_tasks_list_by_channel(
    _request: ListTasksByChannelRequest,
) -> CommandResponse<Vec<Task>> {
    command_not_implemented("tasks.list_by_channel")
}

#[tauri::command]
pub async fn desktop_workers_list_by_task(
    _request: ListWorkersByTaskRequest,
) -> CommandResponse<Vec<Worker>> {
    command_not_implemented("workers.get_by_task")
}

#[tauri::command]
pub async fn desktop_history_list_by_channel(
    _request: ListHistoryByChannelRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    command_not_implemented("history.list_by_channel")
}

#[tauri::command]
pub async fn desktop_history_list_by_branch(
    _request: ListHistoryByBranchRequest,
) -> CommandResponse<Vec<HistoryEvent>> {
    command_not_implemented("history.list_by_branch")
}

fn command_not_implemented<T>(operation: &'static str) -> CommandResponse<T> {
    let error = DesktopAdapterError::NotImplemented(operation).to_envelope();
    CommandResponse::Err { error }
}
