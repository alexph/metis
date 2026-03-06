use metis_contract::{
    branch::Branch, channel::Channel, history::HistoryEvent, metadata::Metadata, task::Task,
    worker::Worker,
};

use crate::storage::models::{
    BranchRecord, ChannelRecord, HistoryRecord, MetadataRecord, TaskRecord, WorkerRecord,
};

pub fn metadata_record_to_contract(record: MetadataRecord) -> Metadata {
    Metadata {
        id: record.id,
        instance_id: record.instance_id,
        schema_version: record.schema_version,
        created_at: record.created_at,
        updated_at: record.updated_at,
    }
}

pub fn channel_record_to_contract(_record: ChannelRecord) -> Option<Channel> {
    None
}

pub fn branch_record_to_contract(_record: BranchRecord) -> Option<Branch> {
    None
}

pub fn task_record_to_contract(_record: TaskRecord) -> Option<Task> {
    None
}

pub fn worker_record_to_contract(_record: WorkerRecord) -> Option<Worker> {
    None
}

pub fn history_record_to_contract(_record: HistoryRecord) -> Option<HistoryEvent> {
    None
}
