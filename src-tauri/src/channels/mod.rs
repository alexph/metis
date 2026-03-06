use metis_contract::channel::Channel;

pub trait ChannelService {
    fn create_channel(&self, channel: Channel) -> Result<Channel, &'static str>;
}

pub struct StubChannelService;

impl ChannelService for StubChannelService {
    fn create_channel(&self, _channel: Channel) -> Result<Channel, &'static str> {
        Err("channels are scaffolding-only in phase 1")
    }
}
