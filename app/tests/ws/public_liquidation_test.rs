// Since the timing of liquidations varies and sometimes tests do not pass, we will not proceed with it.

// use rsbit::v5::ws::{
//     Channel,
//     DeserializedMessage,
// };
// use crate::common::setup_ws;
// use futures_util::stream::StreamExt;



// #[tokio::test]
// async fn test_public_liquidation_success() {
//     let mut ws = setup_ws(Channel::MainnetLinearPublicChannel);
//     let symbols = vec![
//         "BTCUSDT",
//         "ETHUSDT",
//         "SOLUSDT",
//         "XRPUSDT",
//         "SUIUSDT",
//         "ZETAUSDT",
//     ];
//     for symbol in symbols.clone() {
//         ws.add_liquidation_args(symbol);
//     }
//     let result = ws.execute().await;

//     let (_write, mut read) = if let Ok(result) = result {
//         result
//     } else {
//         assert!(false, "Failed to connect to websocket: {:?}", result);
//         return;
//     };

//     while let Some(response) = read.next().await {
//         match response {
//             Ok(response) => {
//                 match ws.deserialize_message(response).await {
//                     Ok(deserialized_message) => {
//                         match deserialized_message {
//                             DeserializedMessage::SubscribePublicSuccess(response) => {
//                                 assert!(response.success);
//                             },
//                             DeserializedMessage::PublicLiquidation(response) => {
//                                 assert!(symbols.contains(&response.data().symbol()));
//                                 break
//                             },
//                             _ => {
//                                 assert!(false, "Unexpected message: {:?}", deserialized_message);
//                                 break;
//                             }
//                         }
//                     },
//                     Err(err) => {
//                         assert!(false, "Failed to deserialize message: {:?}", err);
//                         break;
//                     }
//                 }
//             },
//             Err(err) => {
//                 assert!(false, "Failed to read message: {:?}", err);
//                 break;
//             }
//         }
//     }
// }