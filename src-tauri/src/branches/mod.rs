use metis_contract::branch::Branch;

use crate::{
    channels::ChannelService, core::service_error::ServiceError,
    storage::repositories::BranchRepository,
};

pub trait BranchService {
    fn create_branch(&self, branch: Branch) -> Result<Branch, ServiceError>;
    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<Branch>, ServiceError>;
}

pub struct BranchDomainService<R, C>
where
    R: BranchRepository,
    C: ChannelService,
{
    repository: R,
    channels: C,
}

impl<R, C> BranchDomainService<R, C>
where
    R: BranchRepository,
    C: ChannelService,
{
    pub fn new(repository: R, channels: C) -> Self {
        Self {
            repository,
            channels,
        }
    }
}

impl<R, C> BranchService for BranchDomainService<R, C>
where
    R: BranchRepository,
    C: ChannelService,
{
    fn create_branch(&self, branch: Branch) -> Result<Branch, ServiceError> {
        if branch.id.trim().is_empty() {
            return Err(ServiceError::validation("branch id is required"));
        }
        if branch.channel_id.trim().is_empty() {
            return Err(ServiceError::validation("branch channel_id is required"));
        }
        if branch.name.trim().is_empty() {
            return Err(ServiceError::validation("branch name is required"));
        }

        if self.channels.get_channel(&branch.channel_id)?.is_none() {
            return Err(ServiceError::not_found("channel not found for branch"));
        }

        self.repository.create(branch).map_err(Into::into)
    }

    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<Branch>, ServiceError> {
        if channel_id.trim().is_empty() {
            return Err(ServiceError::validation("channel id is required"));
        }

        self.repository
            .list_by_channel(channel_id)
            .map_err(Into::into)
    }
}
