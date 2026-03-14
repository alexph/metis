use std::sync::Arc;

use crate::runtime::RuntimeCommand;
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

pub fn emit_tauri_event(app: &tauri::AppHandle, event: CommandEvent) -> Result<(), ErrorEnvelope> {
    let name = event_name(&event);
    app.emit(name, event).map_err(|error| ErrorEnvelope {
        code: "desktop_event_emit_error".to_string(),
        message: "failed to emit desktop event".to_string(),
        details: Some(error.to_string()),
    })
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

#[derive(Debug, Clone)]
pub enum DomainEvent {
    HistoryAppended(HistoryEvent),
    RuntimeTurnQueued(RuntimeCommand),
}

pub trait DomainEventSink: Send + Sync {
    fn publish(&self, event: &DomainEvent) -> Result<(), ErrorEnvelope>;
}

pub struct FanoutDomainEventPublisher {
    sinks: Vec<Arc<dyn DomainEventSink>>,
}

impl FanoutDomainEventPublisher {
    pub fn new(sinks: Vec<Arc<dyn DomainEventSink>>) -> Self {
        Self { sinks }
    }

    pub fn publish(&self, event: DomainEvent) {
        for sink in &self.sinks {
            if let Err(error) = sink.publish(&event) {
                tracing::warn!(
                    code = %error.code,
                    message = %error.message,
                    "domain event sink publish failed"
                );
            }
        }
    }
}

pub struct TauriDomainEventSink {
    app: tauri::AppHandle,
}

impl TauriDomainEventSink {
    pub fn new(app: tauri::AppHandle) -> Self {
        Self { app }
    }
}

impl DomainEventSink for TauriDomainEventSink {
    fn publish(&self, event: &DomainEvent) -> Result<(), ErrorEnvelope> {
        match event {
            DomainEvent::HistoryAppended(history) => {
                emit_tauri_event(&self.app, CommandEvent::HistoryAppended(history.clone()))
            }
            DomainEvent::RuntimeTurnQueued(RuntimeCommand::ProcessTurn {
                correlation_id, ..
            }) => emit_tauri_event(
                &self.app,
                CommandEvent::RuntimeStatusChanged(RuntimeStatusPayload {
                    status: "turn_queued".to_string(),
                    correlation_id: correlation_id.clone(),
                }),
            ),
        }
    }
}

pub struct NoopDomainEventSink;

impl DomainEventSink for NoopDomainEventSink {
    fn publish(&self, _event: &DomainEvent) -> Result<(), ErrorEnvelope> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct RecordingSink {
        events: Arc<Mutex<Vec<DomainEvent>>>,
    }

    impl DomainEventSink for RecordingSink {
        fn publish(&self, event: &DomainEvent) -> Result<(), ErrorEnvelope> {
            self.events.lock().expect("lock events").push(event.clone());
            Ok(())
        }
    }

    struct FailingSink;

    impl DomainEventSink for FailingSink {
        fn publish(&self, _event: &DomainEvent) -> Result<(), ErrorEnvelope> {
            Err(ErrorEnvelope {
                code: "sink_failed".to_string(),
                message: "sink failed".to_string(),
                details: None,
            })
        }
    }

    #[test]
    fn fanout_publishes_to_multiple_sinks() {
        let events = Arc::new(Mutex::new(Vec::new()));
        let publisher = FanoutDomainEventPublisher::new(vec![
            Arc::new(RecordingSink {
                events: events.clone(),
            }),
            Arc::new(NoopDomainEventSink),
        ]);

        let event = DomainEvent::RuntimeTurnQueued(RuntimeCommand::ProcessTurn {
            channel_id: "channel-1".to_string(),
            trigger_history_id: "hist-1".to_string(),
            correlation_id: "corr-1".to_string(),
        });

        publisher.publish(event.clone());

        let published = events.lock().expect("lock events");
        assert_eq!(published.len(), 1);
        match &published[0] {
            DomainEvent::RuntimeTurnQueued(RuntimeCommand::ProcessTurn {
                channel_id,
                trigger_history_id,
                correlation_id,
            }) => {
                assert_eq!(channel_id, "channel-1");
                assert_eq!(trigger_history_id, "hist-1");
                assert_eq!(correlation_id, "corr-1");
            }
            _ => panic!("unexpected event variant"),
        }
    }

    #[test]
    fn fanout_isolates_sink_failures() {
        let events = Arc::new(Mutex::new(Vec::new()));
        let publisher = FanoutDomainEventPublisher::new(vec![
            Arc::new(FailingSink),
            Arc::new(RecordingSink {
                events: events.clone(),
            }),
        ]);

        publisher.publish(DomainEvent::RuntimeTurnQueued(
            RuntimeCommand::ProcessTurn {
                channel_id: "channel-1".to_string(),
                trigger_history_id: "hist-1".to_string(),
                correlation_id: "corr-1".to_string(),
            },
        ));

        assert_eq!(events.lock().expect("lock events").len(), 1);
    }
}
