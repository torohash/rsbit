use rsbit::v5::api::post::position::add_or_reduce_margin::{
    AddOrReduceMarginCategory,
    AddOrReduceMarginParameters,
};
use crate::common::setup_api_private;

// #[tokio::test]
// async fn test_add_or_reduce_margin_success() {
//     let api = setup_api_private();
//     let params = AddOrReduceMarginParameters::new(
//         AddOrReduceMarginCategory::Linear,
//         "BTCUSDT".to_string(),
//         "1".to_string(),
//     );
//     let result = api.add_or_reduce_margin(params).await;
//     match result {
//         Ok(result) => {
//             let ret_code = result.ret_code();
//             assert_eq!(ret_code, 0, "Failed to add or reduce margin: {}", result.ret_msg());
//         },
//         Err(err) => {
//             assert!(false, "Failed to add or reduce margin: {:?}", err);
//         }
//     }

//     let params = AddOrReduceMarginParameters::new(
//         AddOrReduceMarginCategory::Linear,
//         "BTCUSDT".to_string(),
//         "1".to_string()
//     );
//     let result = api.add_or_reduce_margin(params).await;
//     match result {
//         Ok(result) => {
//             let ret_code = result.ret_code();
//             assert_eq!(ret_code, 0, "Failed to add or reduce margin: {}", result.ret_msg());
//         },
//         Err(err) => {
//             assert!(false, "Failed to add or reduce margin: {:?}", err);
//         }
//     }
// }

#[tokio::test]
async fn test_add_or_reduce_margin_fail() {
    let api = setup_api_private();
    let params = AddOrReduceMarginParameters::new(
        AddOrReduceMarginCategory::Linear,
        "XXXXXXX".to_string(),
        "1".to_string()
    );
    let result = api.add_or_reduce_margin(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}