use metis_contract::{
    channel::Channel, error::ErrorEnvelope, history::HistoryEvent, task::Task, worker::Worker,
};
use serde::{Deserialize, Serialize};
use tauri::Emitter;

pub const EVENT_CHANNEL_CREATED: &str = "metis://channel-created";
pub const EVENT_CHANNEL_UPDATED: &str = "metis://channel-updated";
pub const EVENT_TASK_ENQUEUED: &str = "metis://task-enqueued";
pub const EVENT_TASK_STATE_CHANGED: &str = "metis://task-state-changed";
pub const EVENT_WORKER_CREATED: &str = "metis://worker-created";
pub const EVENT_WORKER_STATE_CHANGED: &str = "metis://worker-state-changed";
pub const EVENT_WORKER_HEARTBEAT: &str = "metis://worker-heartbeat";
pub const EVENT_HISTORY_APPENDED: &str = "metis://history-appended";
pub const EVENT_RUNTIME_STATUS_CHANGED: &str = "metis://runtime-status-changed";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeStatusPayload {
    pub status: String,
    pub correlation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "payload", rename_all = "snake_case")]
pub enum CommandEvent {
    ChannelCreated(Channel),
    ChannelUpdated(Channel),
    TaskEnqueued(Task),
    TaskStateChanged(Task),
    WorkerCreated(Worker),
    WorkerStateChanged(Worker),
    WorkerHeartbeat(Worker),
    HistoryAppended(HistoryEvent),
    RuntimeStatusChanged(RuntimeStatusPayload),
}

pub trait CommandEventPublisher {
    fn publish(&self, event: CommandEvent) -> Result<(), ErrorEnvelope>;
}

pub struct NoopCommandEventPublisher;

impl CommandEventPublisher for NoopCommandEventPublisher {
    fn publish(&self, _event: CommandEvent) -> Result<(), ErrorEnvelope> {
        Ok(())
    }
}

pub fn event_name(event: &CommandEvent) -> &'static str {
    match event {
        CommandEvent::ChannelCreated(_) => EVENT_CHANNEL_CREATED,
        CommandEvent::ChannelUpdated(_) => EVENT_CHANNEL_UPDATED,
        CommandEvent::TaskEnqueued(_) => EVENT_TASK_ENQUEUED,
        CommandEvent::TaskStateChanged(_) => EVENT_TASK_STATE_CHANGED,
        CommandEvent::WorkerCreated(_) => EVENT_WORKER_CREATED,
        CommandEvent::WorkerStateChanged(_) => EVENT_WORKER_STATE_CHANGED,
        CommandEvent::WorkerHeartbeat(_) => EVENT_WORKER_HEARTBEAT,
        CommandEvent::HistoryAppended(_) => EVENT_HISTORY_APPENDED,
        CommandEvent::RuntimeStatusChanged(_) => EVENT_RUNTIME_STATUS_CHANGED,
    }
}

pub fn emit_tauri_event(app: &tauri::AppHandle, event: CommandEvent) -> Result<(), ErrorEnvelope> {
    let name = event_name(&event);
    app.emit(name, event).map_err(|error| ErrorEnvelope {
        code: "desktop_event_emit_error".to_string(),
        message: "failed to emit desktop event".to_string(),
        details: Some(error.to_string()),
    })
}
