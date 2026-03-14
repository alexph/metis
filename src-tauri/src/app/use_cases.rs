use std::sync::Arc;

use metis_contract::{
    branch::Branch,
    channel::Channel,
    history::{HistoryEvent, HistoryRole},
    task::Task,
    worker::Worker,
};
use tokio::sync::mpsc;

use crate::{
    app::errors::CommandError,
    app::events::{DomainEvent, FanoutDomainEventPublisher},
    app::requests::{
        AppendHistoryRequest, CreateChannelRequest, CreateWorkerRequest, EnqueueTaskRequest,
        ListBranchesByChannelRequest, ListHistoryByBranchRequest, ListHistoryByChannelRequest,
        ListTasksByChannelRequest, ListWorkersByTaskRequest, UpdateChannelStatusRequest,
        UpdateTaskStateRequest, UpdateWorkerStateRequest, WorkerHeartbeatRequest,
    },
    app::service::CommandService,
    runtime::RuntimeCommand,
};

pub trait CommandUseCases: Send + Sync {
    fn channels_list(&self) -> Result<Vec<Channel>, CommandError>;
    fn channels_create(&self, request: CreateChannelRequest) -> Result<Channel, CommandError>;
    fn channels_update_status(
        &self,
        request: UpdateChannelStatusRequest,
    ) -> Result<Channel, CommandError>;
    fn branches_list_by_channel(
        &self,
        request: ListBranchesByChannelRequest,
    ) -> Result<Vec<Branch>, CommandError>;
    fn tasks_enqueue(&self, request: EnqueueTaskRequest) -> Result<Task, CommandError>;
    fn tasks_update_state(&self, request: UpdateTaskStateRequest) -> Result<Task, CommandError>;
    fn tasks_list_by_channel(
        &self,
        request: ListTasksByChannelRequest,
    ) -> Result<Vec<Task>, CommandError>;
    fn workers_list_by_task(
        &self,
        request: ListWorkersByTaskRequest,
    ) -> Result<Vec<Worker>, CommandError>;
    fn workers_create(&self, request: CreateWorkerRequest) -> Result<Worker, CommandError>;
    fn workers_update_state(
        &self,
        request: UpdateWorkerStateRequest,
    ) -> Result<Worker, CommandError>;
    fn workers_heartbeat(&self, request: WorkerHeartbeatRequest) -> Result<Worker, CommandError>;
    fn history_list_by_channel(
        &self,
        request: ListHistoryByChannelRequest,
    ) -> Result<Vec<HistoryEvent>, CommandError>;
    fn history_list_by_branch(
        &self,
        request: ListHistoryByBranchRequest,
    ) -> Result<Vec<HistoryEvent>, CommandError>;
    fn history_append(&self, request: AppendHistoryRequest) -> Result<HistoryEvent, CommandError>;
}

impl<T> CommandUseCases for T
where
    T: CommandService + Send + Sync,
{
    fn channels_list(&self) -> Result<Vec<Channel>, CommandError> {
        CommandService::channels_list(self)
    }

    fn channels_create(&self, request: CreateChannelRequest) -> Result<Channel, CommandError> {
        CommandService::channels_create(self, request)
    }

    fn channels_update_status(
        &self,
        request: UpdateChannelStatusRequest,
    ) -> Result<Channel, CommandError> {
        CommandService::channels_update_status(self, request)
    }

    fn branches_list_by_channel(
        &self,
        request: ListBranchesByChannelRequest,
    ) -> Result<Vec<Branch>, CommandError> {
        CommandService::branches_list_by_channel(self, request)
    }

    fn tasks_enqueue(&self, request: EnqueueTaskRequest) -> Result<Task, CommandError> {
        CommandService::tasks_enqueue(self, request)
    }

    fn tasks_update_state(&self, request: UpdateTaskStateRequest) -> Result<Task, CommandError> {
        CommandService::tasks_update_state(self, request)
    }

    fn tasks_list_by_channel(
        &self,
        request: ListTasksByChannelRequest,
    ) -> Result<Vec<Task>, CommandError> {
        CommandService::tasks_list_by_channel(self, request)
    }

    fn workers_list_by_task(
        &self,
        request: ListWorkersByTaskRequest,
    ) -> Result<Vec<Worker>, CommandError> {
        CommandService::workers_list_by_task(self, request)
    }

    fn workers_create(&self, request: CreateWorkerRequest) -> Result<Worker, CommandError> {
        CommandService::workers_create(self, request)
    }

    fn workers_update_state(
        &self,
        request: UpdateWorkerStateRequest,
    ) -> Result<Worker, CommandError> {
        CommandService::workers_update_state(self, request)
    }

    fn workers_heartbeat(&self, request: WorkerHeartbeatRequest) -> Result<Worker, CommandError> {
        CommandService::workers_heartbeat(self, request)
    }

    fn history_list_by_channel(
        &self,
        request: ListHistoryByChannelRequest,
    ) -> Result<Vec<HistoryEvent>, CommandError> {
        CommandService::history_list_by_channel(self, request)
    }

    fn history_list_by_branch(
        &self,
        request: ListHistoryByBranchRequest,
    ) -> Result<Vec<HistoryEvent>, CommandError> {
        CommandService::history_list_by_branch(self, request)
    }

    fn history_append(&self, request: AppendHistoryRequest) -> Result<HistoryEvent, CommandError> {
        CommandService::history_append(self, request)
    }
}

pub struct AppUseCases {
    command_service: Arc<dyn CommandService>,
    runtime_sender: mpsc::Sender<RuntimeCommand>,
    event_publisher: Arc<FanoutDomainEventPublisher>,
}

impl AppUseCases {
    pub fn new(
        command_service: Arc<dyn CommandService>,
        runtime_sender: mpsc::Sender<RuntimeCommand>,
        event_publisher: Arc<FanoutDomainEventPublisher>,
    ) -> Self {
        Self {
            command_service,
            runtime_sender,
            event_publisher,
        }
    }

    fn maybe_queue_runtime_turn(&self, event: &HistoryEvent) {
        if !matches!(event.role, Some(HistoryRole::User)) {
            return;
        }

        let correlation_id = event
            .correlation_id
            .clone()
            .unwrap_or_else(|| format!("history:{}", event.id));

        let runtime_command = RuntimeCommand::ProcessTurn {
            channel_id: event.channel_id.clone(),
            trigger_history_id: event.id.clone(),
            correlation_id,
        };

        if self
            .runtime_sender
            .try_send(runtime_command.clone())
            .is_err()
        {
            tracing::warn!(
                history_id = %event.id,
                channel_id = %event.channel_id,
                "runtime turn enqueue failed"
            );
            return;
        }

        self.event_publisher
            .publish(DomainEvent::RuntimeTurnQueued(runtime_command));
    }
}

impl CommandUseCases for AppUseCases {
    fn channels_list(&self) -> Result<Vec<Channel>, CommandError> {
        self.command_service.channels_list()
    }

    fn channels_create(&self, request: CreateChannelRequest) -> Result<Channel, CommandError> {
        self.command_service.channels_create(request)
    }

    fn channels_update_status(
        &self,
        request: UpdateChannelStatusRequest,
    ) -> Result<Channel, CommandError> {
        self.command_service.channels_update_status(request)
    }

    fn branches_list_by_channel(
        &self,
        request: ListBranchesByChannelRequest,
    ) -> Result<Vec<Branch>, CommandError> {
        self.command_service.branches_list_by_channel(request)
    }

    fn tasks_enqueue(&self, request: EnqueueTaskRequest) -> Result<Task, CommandError> {
        self.command_service.tasks_enqueue(request)
    }

    fn tasks_update_state(&self, request: UpdateTaskStateRequest) -> Result<Task, CommandError> {
        self.command_service.tasks_update_state(request)
    }

    fn tasks_list_by_channel(
        &self,
        request: ListTasksByChannelRequest,
    ) -> Result<Vec<Task>, CommandError> {
        self.command_service.tasks_list_by_channel(request)
    }

    fn workers_list_by_task(
        &self,
        request: ListWorkersByTaskRequest,
    ) -> Result<Vec<Worker>, CommandError> {
        self.command_service.workers_list_by_task(request)
    }

    fn workers_create(&self, request: CreateWorkerRequest) -> Result<Worker, CommandError> {
        self.command_service.workers_create(request)
    }

    fn workers_update_state(
        &self,
        request: UpdateWorkerStateRequest,
    ) -> Result<Worker, CommandError> {
        self.command_service.workers_update_state(request)
    }

    fn workers_heartbeat(&self, request: WorkerHeartbeatRequest) -> Result<Worker, CommandError> {
        self.command_service.workers_heartbeat(request)
    }

    fn history_list_by_channel(
        &self,
        request: ListHistoryByChannelRequest,
    ) -> Result<Vec<HistoryEvent>, CommandError> {
        self.command_service.history_list_by_channel(request)
    }

    fn history_list_by_branch(
        &self,
        request: ListHistoryByBranchRequest,
    ) -> Result<Vec<HistoryEvent>, CommandError> {
        self.command_service.history_list_by_branch(request)
    }

    fn history_append(&self, request: AppendHistoryRequest) -> Result<HistoryEvent, CommandError> {
        let event = self.command_service.history_append(request)?;
        self.event_publisher
            .publish(DomainEvent::HistoryAppended(event.clone()));
        self.maybe_queue_runtime_turn(&event);
        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    use crate::{
        app::events::{DomainEventSink, NoopDomainEventSink},
        app::service::StubCommandService,
    };

    struct RecordingSink {
        events: Arc<Mutex<Vec<DomainEvent>>>,
    }

    impl DomainEventSink for RecordingSink {
        fn publish(&self, event: &DomainEvent) -> Result<(), metis_contract::error::ErrorEnvelope> {
            self.events.lock().expect("lock events").push(event.clone());
            Ok(())
        }
    }

    struct FixedHistoryService {
        event: HistoryEvent,
    }

    impl CommandService for FixedHistoryService {
        fn channels_list(&self) -> Result<Vec<Channel>, CommandError> {
            panic!("unexpected call")
        }

        fn channels_create(&self, _request: CreateChannelRequest) -> Result<Channel, CommandError> {
            panic!("unexpected call")
        }

        fn channels_update_status(
            &self,
            _request: UpdateChannelStatusRequest,
        ) -> Result<Channel, CommandError> {
            panic!("unexpected call")
        }

        fn branches_list_by_channel(
            &self,
            _request: ListBranchesByChannelRequest,
        ) -> Result<Vec<Branch>, CommandError> {
            panic!("unexpected call")
        }

        fn tasks_enqueue(&self, _request: EnqueueTaskRequest) -> Result<Task, CommandError> {
            panic!("unexpected call")
        }

        fn tasks_update_state(
            &self,
            _request: UpdateTaskStateRequest,
        ) -> Result<Task, CommandError> {
            panic!("unexpected call")
        }

        fn tasks_list_by_channel(
            &self,
            _request: ListTasksByChannelRequest,
        ) -> Result<Vec<Task>, CommandError> {
            panic!("unexpected call")
        }

        fn workers_list_by_task(
            &self,
            _request: ListWorkersByTaskRequest,
        ) -> Result<Vec<Worker>, CommandError> {
            panic!("unexpected call")
        }

        fn workers_create(&self, _request: CreateWorkerRequest) -> Result<Worker, CommandError> {
            panic!("unexpected call")
        }

        fn workers_update_state(
            &self,
            _request: UpdateWorkerStateRequest,
        ) -> Result<Worker, CommandError> {
            panic!("unexpected call")
        }

        fn workers_heartbeat(
            &self,
            _request: WorkerHeartbeatRequest,
        ) -> Result<Worker, CommandError> {
            panic!("unexpected call")
        }

        fn history_list_by_channel(
            &self,
            _request: ListHistoryByChannelRequest,
        ) -> Result<Vec<HistoryEvent>, CommandError> {
            panic!("unexpected call")
        }

        fn history_list_by_branch(
            &self,
            _request: ListHistoryByBranchRequest,
        ) -> Result<Vec<HistoryEvent>, CommandError> {
            panic!("unexpected call")
        }

        fn history_append(
            &self,
            _request: AppendHistoryRequest,
        ) -> Result<HistoryEvent, CommandError> {
            Ok(self.event.clone())
        }
    }

    #[test]
    fn history_append_user_event_enqueues_runtime_turn() {
        let (sender, mut receiver) = mpsc::channel(8);
        let events = Arc::new(Mutex::new(Vec::new()));

        let publisher = Arc::new(FanoutDomainEventPublisher::new(vec![Arc::new(
            RecordingSink {
                events: events.clone(),
            },
        )]));

        let use_cases = AppUseCases::new(
            Arc::new(FixedHistoryService {
                event: HistoryEvent {
                    id: "hist-1".to_string(),
                    channel_id: "channel-1".to_string(),
                    branch_id: None,
                    task_id: None,
                    worker_id: None,
                    event_type: "message".to_string(),
                    role: Some(HistoryRole::User),
                    content_json: "{}".to_string(),
                    correlation_id: Some("corr-1".to_string()),
                    created_at: "2026-01-01T00:00:00Z".to_string(),
                },
            }),
            sender,
            publisher,
        );

        let response = use_cases.history_append(AppendHistoryRequest {
            event: HistoryEvent {
                id: "req-hist-1".to_string(),
                channel_id: "channel-1".to_string(),
                branch_id: None,
                task_id: None,
                worker_id: None,
                event_type: "message".to_string(),
                role: Some(HistoryRole::User),
                content_json: "{}".to_string(),
                correlation_id: Some("corr-1".to_string()),
                created_at: "2026-01-01T00:00:00Z".to_string(),
            },
        });

        assert!(response.is_ok());

        let queued = receiver.try_recv().expect("runtime command queued");
        match queued {
            RuntimeCommand::ProcessTurn {
                channel_id,
                trigger_history_id,
                correlation_id,
            } => {
                assert_eq!(channel_id, "channel-1");
                assert_eq!(trigger_history_id, "hist-1");
                assert_eq!(correlation_id, "corr-1");
            }
        }

        assert_eq!(events.lock().expect("lock events").len(), 2);
    }

    #[test]
    fn history_append_non_user_event_does_not_enqueue_runtime_turn() {
        let (sender, mut receiver) = mpsc::channel(8);
        let publisher = Arc::new(FanoutDomainEventPublisher::new(vec![Arc::new(
            NoopDomainEventSink,
        )]));

        let use_cases = AppUseCases::new(
            Arc::new(FixedHistoryService {
                event: HistoryEvent {
                    id: "hist-1".to_string(),
                    channel_id: "channel-1".to_string(),
                    branch_id: None,
                    task_id: None,
                    worker_id: None,
                    event_type: "message".to_string(),
                    role: Some(HistoryRole::Assistant),
                    content_json: "{}".to_string(),
                    correlation_id: Some("corr-1".to_string()),
                    created_at: "2026-01-01T00:00:00Z".to_string(),
                },
            }),
            sender,
            publisher,
        );

        let response = use_cases.history_append(AppendHistoryRequest {
            event: HistoryEvent {
                id: "req-hist-1".to_string(),
                channel_id: "channel-1".to_string(),
                branch_id: None,
                task_id: None,
                worker_id: None,
                event_type: "message".to_string(),
                role: Some(HistoryRole::Assistant),
                content_json: "{}".to_string(),
                correlation_id: Some("corr-1".to_string()),
                created_at: "2026-01-01T00:00:00Z".to_string(),
            },
        });

        assert!(response.is_ok());
        assert!(receiver.try_recv().is_err());
    }

    #[test]
    fn history_append_runtime_enqueue_failure_is_best_effort() {
        let (sender, receiver) = mpsc::channel(1);
        drop(receiver);

        let use_cases = AppUseCases::new(
            Arc::new(FixedHistoryService {
                event: HistoryEvent {
                    id: "hist-1".to_string(),
                    channel_id: "channel-1".to_string(),
                    branch_id: None,
                    task_id: None,
                    worker_id: None,
                    event_type: "message".to_string(),
                    role: Some(HistoryRole::User),
                    content_json: "{}".to_string(),
                    correlation_id: Some("corr-1".to_string()),
                    created_at: "2026-01-01T00:00:00Z".to_string(),
                },
            }),
            sender,
            Arc::new(FanoutDomainEventPublisher::new(vec![Arc::new(
                NoopDomainEventSink,
            )])),
        );

        let response = use_cases.history_append(AppendHistoryRequest {
            event: HistoryEvent {
                id: "req-hist-1".to_string(),
                channel_id: "channel-1".to_string(),
                branch_id: None,
                task_id: None,
                worker_id: None,
                event_type: "message".to_string(),
                role: Some(HistoryRole::User),
                content_json: "{}".to_string(),
                correlation_id: Some("corr-1".to_string()),
                created_at: "2026-01-01T00:00:00Z".to_string(),
            },
        });

        assert!(response.is_ok());
    }

    #[test]
    fn blanket_trait_impl_for_command_service_is_available() {
        let use_cases: &dyn CommandUseCases = &StubCommandService;
        let response = use_cases.history_append(AppendHistoryRequest {
            event: HistoryEvent {
                id: "hist-1".to_string(),
                channel_id: "channel-1".to_string(),
                branch_id: None,
                task_id: None,
                worker_id: None,
                event_type: "message".to_string(),
                role: Some(HistoryRole::User),
                content_json: "{}".to_string(),
                correlation_id: None,
                created_at: "2026-01-01T00:00:00Z".to_string(),
            },
        });

        assert!(response.is_err());
    }
}
