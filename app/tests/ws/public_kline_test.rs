use rsbit::{
    v5::ws::{
        Channel,
        DeserializedMessage,
    },
    constants::PUBLIC_KLINE_TOPIC,
};
use crate::common::setup_ws;
use futures_util::stream::StreamExt;

#[tokio::test]
async fn test_public_kline_success() {
    let mut ws = setup_ws(Channel::TestnetLinearPublicChannel);
    let symbol = "BTCUSDT";
    let interval = "1";
    ws.add_kline_args(interval, symbol);
    let result = ws.execute().await;

    let (_write, mut read) = if let Ok(result) = result {
        result
    } else {
        assert!(false, "Failed to connect to websocket: {:?}", result);
        return;
    };

    while let Some(response) = read.next().await {
        match response {
            Ok(response) => {
                match ws.deserialize_message(response).await {
                    Ok(deserialized_message) => {
                        match deserialized_message {
                            DeserializedMessage::SubscribePublicSuccess(response) => {
                                assert!(response.success);
                            },
                            DeserializedMessage::PublicKline(response) => {
                                assert_eq!(response.topic(), format!("{}.{}.{}", PUBLIC_KLINE_TOPIC, interval, symbol));
                                break
                            },
                            _ => {
                                assert!(false, "Unexpected message: {:?}", deserialized_message);
                                break;
                            }
                        }
                    },
                    Err(err) => {
                        assert!(false, "Failed to deserialize message: {:?}", err);
                        break;
                    }
                }
            },
            Err(err) => {
                assert!(false, "Failed to read message: {:?}", err);
                break;
            }
        }
    }
}