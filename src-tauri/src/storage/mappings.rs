use metis_contract::{
    branch::Branch,
    channel::{Channel, ChannelSourceType, ChannelStatus},
    history::{HistoryEvent, HistoryRole},
    metadata::Metadata,
    task::{Task, TaskState},
    worker::{Worker, WorkerState},
};

use crate::storage::models::{
    BranchRecord, ChannelRecord, HistoryRecord, MetadataRecord, TaskRecord, WorkerRecord,
};
use crate::storage::StorageError;

pub fn metadata_record_to_contract(record: MetadataRecord) -> Metadata {
    Metadata {
        id: record.id,
        instance_id: record.instance_id,
        schema_version: record.schema_version,
        created_at: record.created_at,
        updated_at: record.updated_at,
    }
}

pub fn metadata_contract_to_record(metadata: Metadata) -> MetadataRecord {
    MetadataRecord {
        id: metadata.id,
        instance_id: metadata.instance_id,
        schema_version: metadata.schema_version,
        created_at: metadata.created_at,
        updated_at: metadata.updated_at,
    }
}

pub fn channel_record_to_contract(record: ChannelRecord) -> Result<Channel, StorageError> {
    Ok(Channel {
        id: record.id,
        title: record.title,
        source_type: parse_channel_source_type(&record.source_type)?,
        source_ref: record.source_ref,
        status: parse_channel_status(&record.status)?,
        created_at: record.created_at,
        updated_at: record.updated_at,
    })
}

pub fn channel_contract_to_record(channel: Channel) -> ChannelRecord {
    ChannelRecord {
        id: channel.id,
        title: channel.title,
        source_type: format_channel_source_type(channel.source_type),
        source_ref: channel.source_ref,
        status: format_channel_status(channel.status),
        created_at: channel.created_at,
        updated_at: channel.updated_at,
    }
}

pub fn branch_record_to_contract(record: BranchRecord) -> Branch {
    Branch {
        id: record.id,
        channel_id: record.channel_id,
        parent_branch_id: record.parent_branch_id,
        name: record.name,
        is_active: record.is_active != 0,
        created_at: record.created_at,
        updated_at: record.updated_at,
    }
}

pub fn branch_contract_to_record(branch: Branch) -> BranchRecord {
    BranchRecord {
        id: branch.id,
        channel_id: branch.channel_id,
        parent_branch_id: branch.parent_branch_id,
        name: branch.name,
        is_active: i64::from(branch.is_active),
        created_at: branch.created_at,
        updated_at: branch.updated_at,
    }
}

pub fn task_record_to_contract(record: TaskRecord) -> Result<Task, StorageError> {
    Ok(Task {
        id: record.id,
        channel_id: record.channel_id,
        branch_id: record.branch_id,
        kind: record.kind,
        state: parse_task_state(&record.state)?,
        priority: record.priority,
        payload_json: record.payload_json,
        created_at: record.created_at,
        updated_at: record.updated_at,
        started_at: record.started_at,
        finished_at: record.finished_at,
    })
}

pub fn task_contract_to_record(task: Task) -> TaskRecord {
    TaskRecord {
        id: task.id,
        channel_id: task.channel_id,
        branch_id: task.branch_id,
        kind: task.kind,
        state: format_task_state(task.state),
        priority: task.priority,
        payload_json: task.payload_json,
        created_at: task.created_at,
        updated_at: task.updated_at,
        started_at: task.started_at,
        finished_at: task.finished_at,
    }
}

pub fn worker_record_to_contract(record: WorkerRecord) -> Result<Worker, StorageError> {
    Ok(Worker {
        id: record.id,
        task_id: record.task_id,
        worker_type: record.worker_type,
        state: parse_worker_state(&record.state)?,
        attempt: record.attempt,
        last_heartbeat_at: record.last_heartbeat_at,
        started_at: record.started_at,
        finished_at: record.finished_at,
        created_at: record.created_at,
        updated_at: record.updated_at,
    })
}

pub fn worker_contract_to_record(worker: Worker) -> WorkerRecord {
    WorkerRecord {
        id: worker.id,
        task_id: worker.task_id,
        worker_type: worker.worker_type,
        state: format_worker_state(worker.state),
        attempt: worker.attempt,
        last_heartbeat_at: worker.last_heartbeat_at,
        started_at: worker.started_at,
        finished_at: worker.finished_at,
        created_at: worker.created_at,
        updated_at: worker.updated_at,
    }
}

pub fn history_record_to_contract(record: HistoryRecord) -> Result<HistoryEvent, StorageError> {
    Ok(HistoryEvent {
        id: record.id,
        channel_id: record.channel_id,
        branch_id: record.branch_id,
        task_id: record.task_id,
        worker_id: record.worker_id,
        event_type: record.event_type,
        role: parse_optional_history_role(record.role)?,
        content_json: record.content_json,
        correlation_id: record.correlation_id,
        created_at: record.created_at,
    })
}

pub fn history_contract_to_record(event: HistoryEvent) -> HistoryRecord {
    HistoryRecord {
        id: event.id,
        channel_id: event.channel_id,
        branch_id: event.branch_id,
        task_id: event.task_id,
        worker_id: event.worker_id,
        event_type: event.event_type,
        role: event.role.map(format_history_role),
        content_json: event.content_json,
        correlation_id: event.correlation_id,
        created_at: event.created_at,
    }
}

fn parse_channel_source_type(value: &str) -> Result<ChannelSourceType, StorageError> {
    match value.to_ascii_lowercase().as_str() {
        "manual" => Ok(ChannelSourceType::Manual),
        "external" => Ok(ChannelSourceType::External),
        _ => Err(StorageError::Mapping {
            context: "channels.source_type",
            value: value.to_string(),
        }),
    }
}

fn format_channel_source_type(value: ChannelSourceType) -> String {
    match value {
        ChannelSourceType::Manual => "manual",
        ChannelSourceType::External => "external",
    }
    .to_string()
}

fn parse_channel_status(value: &str) -> Result<ChannelStatus, StorageError> {
    match value.to_ascii_lowercase().as_str() {
        "active" => Ok(ChannelStatus::Active),
        "archived" => Ok(ChannelStatus::Archived),
        "suspended" => Ok(ChannelStatus::Suspended),
        _ => Err(StorageError::Mapping {
            context: "channels.status",
            value: value.to_string(),
        }),
    }
}

fn format_channel_status(value: ChannelStatus) -> String {
    match value {
        ChannelStatus::Active => "active",
        ChannelStatus::Archived => "archived",
        ChannelStatus::Suspended => "suspended",
    }
    .to_string()
}

fn parse_task_state(value: &str) -> Result<TaskState, StorageError> {
    match value.to_ascii_lowercase().as_str() {
        "queued" => Ok(TaskState::Queued),
        "running" => Ok(TaskState::Running),
        "completed" => Ok(TaskState::Completed),
        "failed" => Ok(TaskState::Failed),
        "cancelled" => Ok(TaskState::Cancelled),
        _ => Err(StorageError::Mapping {
            context: "tasks.state",
            value: value.to_string(),
        }),
    }
}

fn format_task_state(value: TaskState) -> String {
    match value {
        TaskState::Queued => "queued",
        TaskState::Running => "running",
        TaskState::Completed => "completed",
        TaskState::Failed => "failed",
        TaskState::Cancelled => "cancelled",
    }
    .to_string()
}

fn parse_worker_state(value: &str) -> Result<WorkerState, StorageError> {
    match value.to_ascii_lowercase().as_str() {
        "pending" => Ok(WorkerState::Pending),
        "running" => Ok(WorkerState::Running),
        "completed" => Ok(WorkerState::Completed),
        "failed" => Ok(WorkerState::Failed),
        "stopped" => Ok(WorkerState::Stopped),
        _ => Err(StorageError::Mapping {
            context: "workers.state",
            value: value.to_string(),
        }),
    }
}

fn format_worker_state(value: WorkerState) -> String {
    match value {
        WorkerState::Pending => "pending",
        WorkerState::Running => "running",
        WorkerState::Completed => "completed",
        WorkerState::Failed => "failed",
        WorkerState::Stopped => "stopped",
    }
    .to_string()
}

fn parse_optional_history_role(value: Option<String>) -> Result<Option<HistoryRole>, StorageError> {
    match value {
        Some(role) => Ok(Some(parse_history_role(&role)?)),
        None => Ok(None),
    }
}

fn parse_history_role(value: &str) -> Result<HistoryRole, StorageError> {
    match value.to_ascii_lowercase().as_str() {
        "system" => Ok(HistoryRole::System),
        "user" => Ok(HistoryRole::User),
        "assistant" => Ok(HistoryRole::Assistant),
        "tool" => Ok(HistoryRole::Tool),
        _ => Err(StorageError::Mapping {
            context: "history.role",
            value: value.to_string(),
        }),
    }
}

fn format_history_role(value: HistoryRole) -> String {
    match value {
        HistoryRole::System => "system",
        HistoryRole::User => "user",
        HistoryRole::Assistant => "assistant",
        HistoryRole::Tool => "tool",
    }
    .to_string()
}
