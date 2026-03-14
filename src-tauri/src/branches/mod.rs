use metis_contract::branch::Branch;

use crate::{
    app::errors::Error, channels::ChannelService, storage::repositories::BranchRepository,
};

pub trait BranchService {
    fn create_branch(&self, branch: Branch) -> Result<Branch, Error>;
    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<Branch>, Error>;
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
    fn create_branch(&self, branch: Branch) -> Result<Branch, Error> {
        if branch.id.trim().is_empty() {
            return Err(Error::validation("branch id is required"));
        }
        if branch.channel_id.trim().is_empty() {
            return Err(Error::validation("branch channel_id is required"));
        }
        if branch.name.trim().is_empty() {
            return Err(Error::validation("branch name is required"));
        }

        if self.channels.get_channel(&branch.channel_id)?.is_none() {
            return Err(Error::not_found("channel not found for branch"));
        }

        self.repository.create(branch).map_err(Into::into)
    }

    fn list_by_channel(&self, channel_id: &str) -> Result<Vec<Branch>, Error> {
        if channel_id.trim().is_empty() {
            return Err(Error::validation("channel id is required"));
        }

        self.repository
            .list_by_channel(channel_id)
            .map_err(Into::into)
    }
}
