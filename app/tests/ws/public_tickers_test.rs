use rsbit::{
    v5::ws::{
        Channel,
        DeserializedMessage,
    },
    constants::PUBLIC_TICKERS_TOPIC,
};
use crate::common::setup_ws;
use futures_util::stream::StreamExt;

#[tokio::test]
async fn test_public_linear_tickers_success() {
    let mut ws = setup_ws(Channel::TestnetLinearPublicChannel);
    let symbol = "BTCUSDT";
    ws.add_tickers_args(symbol);
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
                            DeserializedMessage::PublicLinearTickers(response) => {
                                assert_eq!(response.topic(), format!("{}.{}", PUBLIC_TICKERS_TOPIC, symbol));
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

#[tokio::test]
async fn test_public_inverse_tickers_success() {
    let mut ws = setup_ws(Channel::TestnetInversePublicChannel);
    let symbol = "BTCUSD";
    ws.add_tickers_args(symbol);
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
                            DeserializedMessage::PublicInverseTickers(response) => {
                                assert_eq!(response.topic(), format!("{}.{}", PUBLIC_TICKERS_TOPIC, symbol));
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

#[tokio::test]
async fn test_public_option_tickers_success() {
    let mut ws = setup_ws(Channel::TestnetOptionPublicChannel);
    let symbol = "BTC-6JAN23-17500-C";
    ws.add_tickers_args(symbol);
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
                                break
                            },
                            // Since responses do not come frequently, this will not be tested
                            // DeserializedMessage::PublicOptionTickers(response) => {
                            //     assert_eq!(response.topic(), format!("{}.{}", PUBLIC_TICKERS_TOPIC, symbol));
                            //     break
                            // },
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

#[tokio::test]
async fn test_public_spot_tickers_success() {
    let mut ws = setup_ws(Channel::TestnetSpotPublicChannel);
    let symbol = "BTCUSDT";
    ws.add_tickers_args(symbol);
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
                            DeserializedMessage::PublicSpotTickers(response) => {
                                assert_eq!(response.topic(), format!("{}.{}", PUBLIC_TICKERS_TOPIC, symbol));
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