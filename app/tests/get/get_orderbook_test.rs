use rsbit::api::v5::get::market::get_orderbook::{
    GetOrderbookParameters,
    GetOrderbookCategory,
};
use crate::common::setup_api_public;

#[tokio::test]
async fn test_get_orderbook_success() {
    let api = setup_api_public();
    let categories = vec![
        (GetOrderbookCategory::Linear, "BTCUSDT".to_string()),
        (GetOrderbookCategory::Inverse, "BTCUSD".to_string()),
        // (GetOrderbookCategory::Option, "".to_string()), // Options are difficult to test due to the relationship with symbol names.
        (GetOrderbookCategory::Spot, "BTCUSDT".to_string()),
    ];

    for (category, symbol) in categories.into_iter() {
        let params = GetOrderbookParameters::new(
            category,
            symbol.clone(),
        );
    
        let result = api.get_orderbook(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get orderbook: {}", result.ret_msg());
    
            },
            Err(err) => {
                assert!(false, "Failed to get orderbook: {:?}", err);
            }
        }
    }


}

#[tokio::test]
async fn test_get_orderbook_fail() {
    let api = setup_api_public();
    let params = GetOrderbookParameters::new(
        GetOrderbookCategory::Linear,
        "XXXXXXX".to_string(),
    );

    let result = api.get_orderbook(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}