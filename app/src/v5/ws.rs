use crate::constants::{
    MAINNET_SPOT_PUBLIC_CHANNEL,
    MAINNET_LINEAR_PUBLIC_CHANNEL,
    MAINNET_INVERSE_PUBLIC_CHANNEL,
    MAINNET_OPTION_PUBLIC_CHANNEL,
    TESTNET_SPOT_PUBLIC_CHANNEL,
    TESTNET_LINEAR_PUBLIC_CHANNEL,
    TESTNET_INVERSE_PUBLIC_CHANNEL,
    TESTNET_OPTION_PUBLIC_CHANNEL,
    MAINNET_PRIVATE_CHANNEL,
    TESTNET_PRIVATE_CHANNEL,
};

pub enum Channel {
    MainnetSpotPublicChannel,
    MainnetLinearPublicChannel,
    MainnetInversePublicChannel,
    MainnetOptionPublicChannel,
    TestnetSpotPublicChannel,
    TestnetLinearPublicChannel,
    TestnetInversePublicChannel,
    TestnetOptionPublicChannel,
    MainnetPrivateChannel,
    TestnetPrivateChannel,
}

impl Channel {
    fn to_string(&self) -> &'static str {
        match self {
            Channel::MainnetSpotPublicChannel => MAINNET_SPOT_PUBLIC_CHANNEL,
            Channel::MainnetLinearPublicChannel => MAINNET_LINEAR_PUBLIC_CHANNEL,
            Channel::MainnetInversePublicChannel => MAINNET_INVERSE_PUBLIC_CHANNEL,
            Channel::MainnetOptionPublicChannel => MAINNET_OPTION_PUBLIC_CHANNEL,
            Channel::TestnetSpotPublicChannel => TESTNET_SPOT_PUBLIC_CHANNEL,
            Channel::TestnetLinearPublicChannel => TESTNET_LINEAR_PUBLIC_CHANNEL,
            Channel::TestnetInversePublicChannel => TESTNET_INVERSE_PUBLIC_CHANNEL,
            Channel::TestnetOptionPublicChannel => TESTNET_OPTION_PUBLIC_CHANNEL,
            Channel::MainnetPrivateChannel => MAINNET_PRIVATE_CHANNEL,
            Channel::TestnetPrivateChannel => TESTNET_PRIVATE_CHANNEL,
        }
    }
}

pub struct BybitWebsocket {
    channel: Channel,
}

impl BybitWebsocket {
    pub fn new(channel: Channel) -> Self {
        Self {
            channel,
        }
    }

    pub fn channel(&self) -> &'static str {
        self.channel.to_string()
    }
}