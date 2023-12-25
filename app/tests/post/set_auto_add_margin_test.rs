use rsbit::v5::api::post::position::set_auto_add_margin::{
    SetAutoAddMarginCategory,
    SetAutoAddMarginParameters,
};
use crate::common::setup_api_private;

// #[tokio::test]
// async fn test_set_auto_add_margin_success() {
//     let api = setup_api_private();
//     let params = SetAutoAddMarginParameters::new(
//         SetAutoAddMarginCategory::Linear,
//         "BTCUSDT".to_string(),
//         1,
//     );
//     let result = api.set_auto_add_margin(params).await;
//     match result {
//         Ok(result) => {
//             let ret_code = result.ret_code();
//             assert_eq!(ret_code, 0, "Failed to set auto add margin: {}", result.ret_msg());
//         },
//         Err(err) => {
//             assert!(false, "Failed to set auto add margin: {:?}", err);
//         }
//     }

//     let params = SetAutoAddMarginParameters::new(
//         SetAutoAddMarginCategory::Linear,
//         "BTCUSDT".to_string(),
//         0
//     );
//     let result = api.set_auto_add_margin(params).await;
//     match result {
//         Ok(result) => {
//             let ret_code = result.ret_code();
//             assert_eq!(ret_code, 0, "Failed to set auto add margin: {}", result.ret_msg());
//         },
//         Err(err) => {
//             assert!(false, "Failed to set auto add margin: {:?}", err);
//         }
//     }
// }

#[tokio::test]
async fn test_set_auto_add_margin_fail() {
    let api = setup_api_private();
    let params = SetAutoAddMarginParameters::new(
        SetAutoAddMarginCategory::Linear,
        "XXXXXXX".to_string(),
        0
    );
    let result = api.set_auto_add_margin(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}