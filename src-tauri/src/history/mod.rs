use metis_contract::history::HistoryEvent;

pub trait HistoryService {
    fn append_event(&self, event: HistoryEvent) -> Result<HistoryEvent, &'static str>;
}

pub struct StubHistoryService;

impl HistoryService for StubHistoryService {
    fn append_event(&self, _event: HistoryEvent) -> Result<HistoryEvent, &'static str> {
        Err("history is scaffolding-only in phase 1")
    }
}
