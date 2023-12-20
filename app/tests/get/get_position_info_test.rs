use rsbit::v5::api::get::position::get_position_info::{
    GetPositionInfoParameters,
    GetPositionInfoCategory,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_position_info_success() {
    let api = setup_api_private();
    let categories = vec![
        (GetPositionInfoCategory::Linear, "BTCUSDT".to_string()),
    ];

    for (category, symbol) in categories.into_iter() {
        let params = GetPositionInfoParameters::new(
            category,
        ).with_symbol(symbol.clone());
    
        let result = api.get_position_info(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get position info: {}", result.ret_msg());
                assert_eq!(result.result().list()[0].symbol(), &symbol);
            },
            Err(err) => {
                assert!(false, "Failed to get position info: {:?}", err);
            }
        }
    }
}

#[tokio::test]
async fn test_get_position_info_fail() {
    let api = setup_api_private();
    let params = GetPositionInfoParameters::new(
        GetPositionInfoCategory::Linear,
    ).with_symbol("XXXXXXX".to_string());

    let result = api.get_position_info(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}