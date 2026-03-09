use metis_contract::history::HistoryEvent;

use crate::{core::service_error::ServiceError, storage::repositories::HistoryRepository};

pub trait HistoryService {
    fn append_event(&self, event: HistoryEvent) -> Result<HistoryEvent, ServiceError>;
    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<HistoryEvent>, ServiceError>;
    fn list_by_branch(&self, branch_id: &str) -> Result<Vec<HistoryEvent>, ServiceError>;
}

pub struct HistoryDomainService<R>
where
    R: HistoryRepository,
{
    repository: R,
}

impl<R> HistoryDomainService<R>
where
    R: HistoryRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> HistoryService for HistoryDomainService<R>
where
    R: HistoryRepository,
{
    fn append_event(&self, event: HistoryEvent) -> Result<HistoryEvent, ServiceError> {
        if event.id.trim().is_empty() {
            return Err(ServiceError::validation("history id is required"));
        }
        if event.channel_id.trim().is_empty() {
            return Err(ServiceError::validation("history channel_id is required"));
        }
        if event.event_type.trim().is_empty() {
            return Err(ServiceError::validation("history event_type is required"));
        }
        if event.content_json.trim().is_empty() {
            return Err(ServiceError::validation("history content_json is required"));
        }

        self.repository.append(event).map_err(Into::into)
    }

    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<HistoryEvent>, ServiceError> {
        if channel_id.trim().is_empty() {
            return Err(ServiceError::validation("channel id is required"));
        }
        self.repository
            .list_by_channel(channel_id)
            .map_err(Into::into)
    }

    fn list_by_branch(&self, branch_id: &str) -> Result<Vec<HistoryEvent>, ServiceError> {
        if branch_id.trim().is_empty() {
            return Err(ServiceError::validation("branch id is required"));
        }
        self.repository
            .list_by_branch(branch_id)
            .map_err(Into::into)
    }
}
