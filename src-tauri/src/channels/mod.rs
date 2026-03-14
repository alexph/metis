use metis_contract::channel::{Channel, ChannelStatus};

use crate::{app::errors::Error, storage::repositories::ChannelRepository};

pub trait ChannelService {
    fn create_channel(&self, channel: Channel) -> Result<Channel, Error>;
    fn get_channel(&self, channel_id: &str) -> Result<Option<Channel>, Error>;
    fn list_channels(&self) -> Result<Vec<Channel>, Error>;
    fn update_channel_status(&self, channel_id: &str, status: ChannelStatus) -> Result<(), Error>;
}

pub struct ChannelDomainService<R>
where
    R: ChannelRepository,
{
    repository: R,
}

impl<R> ChannelDomainService<R>
where
    R: ChannelRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> ChannelService for ChannelDomainService<R>
where
    R: ChannelRepository,
{
    fn create_channel(&self, channel: Channel) -> Result<Channel, Error> {
        if channel.id.trim().is_empty() {
            return Err(Error::validation("channel id is required"));
        }

        if channel.title.trim().is_empty() {
            return Err(Error::validation("channel title is required"));
        }

        self.repository.create(channel).map_err(Into::into)
    }

    fn get_channel(&self, channel_id: &str) -> Result<Option<Channel>, Error> {
        if channel_id.trim().is_empty() {
            return Err(Error::validation("channel id is required"));
        }

        self.repository.get(channel_id).map_err(Into::into)
    }

    fn list_channels(&self) -> Result<Vec<Channel>, Error> {
        self.repository.list().map_err(Into::into)
    }

    fn update_channel_status(&self, channel_id: &str, status: ChannelStatus) -> Result<(), Error> {
        if channel_id.trim().is_empty() {
            return Err(Error::validation("channel id is required"));
        }

        if self
            .repository
            .get(channel_id)
            .map_err(Error::from)?
            .is_none()
        {
            return Err(Error::not_found("channel not found"));
        }

        self.repository
            .update_status(channel_id, status)
            .map_err(Into::into)
    }
}
