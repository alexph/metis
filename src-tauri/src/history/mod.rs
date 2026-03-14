use metis_contract::history::HistoryEvent;

use crate::{app::errors::Error, storage::repositories::HistoryRepository};

pub trait HistoryService {
    fn append_event(&self, event: HistoryEvent) -> Result<HistoryEvent, Error>;
    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<HistoryEvent>, Error>;
    fn list_by_branch(&self, branch_id: &str) -> Result<Vec<HistoryEvent>, Error>;
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
    fn append_event(&self, event: HistoryEvent) -> Result<HistoryEvent, Error> {
        if event.id.trim().is_empty() {
            return Err(Error::validation("history id is required"));
        }
        if event.channel_id.trim().is_empty() {
            return Err(Error::validation("history channel_id is required"));
        }
        if event.event_type.trim().is_empty() {
            return Err(Error::validation("history event_type is required"));
        }
        if event.content_json.trim().is_empty() {
            return Err(Error::validation("history content_json is required"));
        }

        self.repository.append(event).map_err(Into::into)
    }

    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<HistoryEvent>, Error> {
        if channel_id.trim().is_empty() {
            return Err(Error::validation("channel id is required"));
        }
        self.repository
            .list_by_channel(channel_id)
            .map_err(Into::into)
    }

    fn list_by_branch(&self, branch_id: &str) -> Result<Vec<HistoryEvent>, Error> {
        if branch_id.trim().is_empty() {
            return Err(Error::validation("branch id is required"));
        }
        self.repository
            .list_by_branch(branch_id)
            .map_err(Into::into)
    }
}
