pub mod connect;
pub mod private;
pub mod public;

use crate::{
    constants::{
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
        PUBLIC_TRADE_TOPIC,
        PUBLIC_ORDERBOOK_TOPIC,
        PUBLIC_TICKERS_TOPIC,
        PUBLIC_KLINE_TOPIC,
        PUBLIC_LIQUIDATION_TOPIC,
        PRIVATE_POSITION_TOPIC,
        PRIVATE_EXECUTION_TOPIC,
        PRIVATE_ORDER_TOPIC,
    },
    v5::ws::{
        public::{
            trade::PublicTradeResponse,
            orderbook::PublicOrderbookResponse,
            tickers::{
                linear::PublicLinearTickersResponse,
                spot::PublicSpotTickersResponse,
                inverse::PublicInverseTickersResponse,
                option::PublicOptionTickersResponse,
            },
            kline::PublicKlineResponse,
            liquidation::PublicLiquidationResponse,
        },
        private::{
            position::PrivatePositionResponse,
            execution::PrivateExecutionResponse,
            order::PrivateOrderResponse,
        },
    },
};
use serde::Deserialize;
use std::collections::HashMap;
use serde_json::{Value, to_string};
use tokio_tungstenite::{
    WebSocketStream,
    MaybeTlsStream,
    tungstenite::Message,
};
use futures_util::{
    SinkExt,
    stream::{
        SplitSink,
        SplitStream,
    },
};
use tokio::net::TcpStream;
use anyhow::Result;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum DeserializedMessage {
    SubscribePublicSuccess(SubscribePublicSuccessResponse),
    PublicTrade(PublicTradeResponse),
    PublicOrderbook(PublicOrderbookResponse),
    PublicLinearTickers(PublicLinearTickersResponse),
    PublicSpotTickers(PublicSpotTickersResponse),
    PublicInverseTickers(PublicInverseTickersResponse),
    PublicOptionTickers(PublicOptionTickersResponse),
    PublicKline(PublicKlineResponse),
    PublicLiquidation(PublicLiquidationResponse),
    PrivatePosition(PrivatePositionResponse),
    PrivateExecution(PrivateExecutionResponse),
    PrivateOrder(PrivateOrderResponse),
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubscribePublicSuccessResponse {
    pub success: bool,
    pub ret_msg: Option<String>,
    pub conn_id: String,
    pub req_id: Option<String>,
    pub op: Option<String>
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

    pub fn is_private(&self) -> bool {
        match self {
            Channel::MainnetSpotPublicChannel => false,
            Channel::MainnetLinearPublicChannel => false,
            Channel::MainnetInversePublicChannel => false,
            Channel::MainnetOptionPublicChannel => false,
            Channel::TestnetSpotPublicChannel => false,
            Channel::TestnetLinearPublicChannel => false,
            Channel::TestnetInversePublicChannel => false,
            Channel::TestnetOptionPublicChannel => false,
            Channel::MainnetPrivateChannel => true,
            Channel::TestnetPrivateChannel => true,
        }
    }

    pub fn channel_category(&self) -> ChannelCategory {
        match self {
            Channel::MainnetSpotPublicChannel => ChannelCategory::Spot,
            Channel::MainnetLinearPublicChannel => ChannelCategory::Linear,
            Channel::MainnetInversePublicChannel => ChannelCategory::Inverse,
            Channel::MainnetOptionPublicChannel => ChannelCategory::Option,
            Channel::TestnetSpotPublicChannel => ChannelCategory::Spot,
            Channel::TestnetLinearPublicChannel => ChannelCategory::Linear,
            Channel::TestnetInversePublicChannel => ChannelCategory::Inverse,
            Channel::TestnetOptionPublicChannel => ChannelCategory::Option,
            Channel::MainnetPrivateChannel => ChannelCategory::Private,
            Channel::TestnetPrivateChannel => ChannelCategory::Private,
        }
    }

}

pub enum ChannelCategory {
    Linear,
    Inverse,
    Spot,
    Option,
    Private,
}

pub struct BybitWS {
    channel: Channel,
    api_key: Option<String>,
    api_secret: Option<String>,
    args: Vec<String>,
}

impl BybitWS {
    pub fn new(channel: Channel) -> Self {
        Self {
            channel,
            api_key: None,
            api_secret: None,
            args: Vec::new(),
        }
    }

    pub fn channel(&self) -> &'static str {
        self.channel.to_string()
    }

    pub fn is_private_channel(&self) -> bool {
        self.channel.is_private()
    }

    pub fn api_key(&self) -> Option<&str> {
        match &self.api_key {
            Some(api_key) => Some(api_key),
            None => None,
        }
    }

    pub fn api_secret(&self) -> Option<&str> {
        match &self.api_secret {
            Some(api_secret) => Some(api_secret),
            None => None,
        }
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn with_api_secret(mut self, api_secret: String) -> Self {
        self.api_secret = Some(api_secret);
        self
    }

    // argsに追加したtopicをsubscribeする。
    pub async fn execute(&self) -> Result<(
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
    )> {
        let (mut write, read) = self.connect(self.is_private_channel()).await?;
        let mut subscribe: HashMap<&str, Value> = HashMap::new();
        subscribe.insert("op", Value::String("subscribe".to_string()));
        let args = self.args.iter().map(|arg| Value::String(arg.to_string())).collect::<Vec<Value>>();
        subscribe.insert("args", Value::Array(args));
        let subscribe_message = Message::Text(to_string(&subscribe)?);
        write.send(subscribe_message).await?;
        Ok((write, read))
    }

    // subscribeしたtopicからのメッセージを適切な構造体にデシリアライズする。
    pub async fn deserialize_message(&self, message: Message) -> Result<DeserializedMessage> {
        let message = match message {
            Message::Text(message) => message,
            _ => return Err(anyhow::anyhow!("Message is not text")),
        };
        let value: Value = serde_json::from_str(&message)?;

        println!("message: {:?}", message);

        match value.get("conn_id").and_then(Value::as_str) {
            Some(_conn_id) => {
                let response: SubscribePublicSuccessResponse = serde_json::from_str(&message)?;
                if response.success {
                    Ok(DeserializedMessage::SubscribePublicSuccess(response))
                } else {
                    Err(anyhow::anyhow!("Subscribe failed"))
                }
            },
            None => match value.get("topic").and_then(Value::as_str) {
                Some(topic) if topic.contains(PUBLIC_TRADE_TOPIC) => {
                    let response: PublicTradeResponse = serde_json::from_str(&message)?;
                    Ok(DeserializedMessage::PublicTrade(response))
                },
                Some(topic) if topic.contains(PUBLIC_ORDERBOOK_TOPIC) => {
                    let response: PublicOrderbookResponse = serde_json::from_str(&message)?;
                    Ok(DeserializedMessage::PublicOrderbook(response))
                },
                Some(topic) if topic.contains(PUBLIC_TICKERS_TOPIC) => {
                    match self.channel.channel_category() {
                        ChannelCategory::Linear => {
                            let response: PublicLinearTickersResponse = serde_json::from_str(&message)?;
                            Ok(DeserializedMessage::PublicLinearTickers(response))
                        },
                        ChannelCategory::Spot => {
                            let response: PublicSpotTickersResponse = serde_json::from_str(&message)?;
                            Ok(DeserializedMessage::PublicSpotTickers(response))
                        },
                        ChannelCategory::Inverse => {
                            let response: PublicInverseTickersResponse = serde_json::from_str(&message)?;
                            Ok(DeserializedMessage::PublicInverseTickers(response))
                        },
                        ChannelCategory::Option => {
                            let response: PublicOptionTickersResponse = serde_json::from_str(&message)?;
                            Ok(DeserializedMessage::PublicOptionTickers(response))
                        },
                        ChannelCategory::Private => {
                            Err(anyhow::anyhow!("Private category is not supported for tickers"))
                        }
                    }
                },
                Some(topic) if topic.contains(PUBLIC_KLINE_TOPIC) => {
                    let response: PublicKlineResponse = serde_json::from_str(&message)?;
                    Ok(DeserializedMessage::PublicKline(response))
                },
                Some(topic) if topic.contains(PUBLIC_LIQUIDATION_TOPIC) => {
                    let response: PublicLiquidationResponse = serde_json::from_str(&message)?;
                    Ok(DeserializedMessage::PublicLiquidation(response))
                },
                Some(topic) if topic.contains(PRIVATE_POSITION_TOPIC) => {
                    let response: PrivatePositionResponse = serde_json::from_str(&message)?;
                    Ok(DeserializedMessage::PrivatePosition(response))
                },
                Some(topic) if topic.contains(PRIVATE_EXECUTION_TOPIC) => {
                    let response: PrivateExecutionResponse = serde_json::from_str(&message)?;
                    Ok(DeserializedMessage::PrivateExecution(response))
                },
                Some(topic) if topic.contains(PRIVATE_ORDER_TOPIC) => {
                    let response: PrivateOrderResponse = serde_json::from_str(&message)?;
                    Ok(DeserializedMessage::PrivateOrder(response))
                },
                Some(_) | None => {
                    Err(anyhow::anyhow!("Unknown message"))
                }
            }
        }
    }
}
