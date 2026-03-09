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
pub enum DesktopEvent {
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

pub trait DesktopEventPublisher {
    fn publish(&self, event: DesktopEvent) -> Result<(), ErrorEnvelope>;
}

pub struct NoopDesktopEventPublisher;

impl DesktopEventPublisher for NoopDesktopEventPublisher {
    fn publish(&self, _event: DesktopEvent) -> Result<(), ErrorEnvelope> {
        Ok(())
    }
}

pub fn event_name(event: &DesktopEvent) -> &'static str {
    match event {
        DesktopEvent::ChannelCreated(_) => EVENT_CHANNEL_CREATED,
        DesktopEvent::ChannelUpdated(_) => EVENT_CHANNEL_UPDATED,
        DesktopEvent::TaskEnqueued(_) => EVENT_TASK_ENQUEUED,
        DesktopEvent::TaskStateChanged(_) => EVENT_TASK_STATE_CHANGED,
        DesktopEvent::WorkerCreated(_) => EVENT_WORKER_CREATED,
        DesktopEvent::WorkerStateChanged(_) => EVENT_WORKER_STATE_CHANGED,
        DesktopEvent::WorkerHeartbeat(_) => EVENT_WORKER_HEARTBEAT,
        DesktopEvent::HistoryAppended(_) => EVENT_HISTORY_APPENDED,
        DesktopEvent::RuntimeStatusChanged(_) => EVENT_RUNTIME_STATUS_CHANGED,
    }
}

pub fn emit_tauri_event(app: &tauri::AppHandle, event: DesktopEvent) -> Result<(), ErrorEnvelope> {
    let name = event_name(&event);
    app.emit(name, event).map_err(|error| ErrorEnvelope {
        code: "desktop_event_emit_error".to_string(),
        message: "failed to emit desktop event".to_string(),
        details: Some(error.to_string()),
    })
}
