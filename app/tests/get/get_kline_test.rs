use rsbit::api::get::market::get_kline::{
    GetKlineParameters,
    GetKlineCategory,
};
use crate::common::setup_api;

#[tokio::test]
async fn test_get_kline_success() {
    let api = setup_api();
    let categories = vec![
        (GetKlineCategory::Linear, "BTCUSDT".to_string(), "1".to_string()),
        (GetKlineCategory::Inverse, "BTCUSD".to_string(), "360".to_string()),
        (GetKlineCategory::Spot, "BTCUSDT".to_string(), "D".to_string()),
    ];

    for (category, symbol, interval) in categories.into_iter() {
        let params = GetKlineParameters::new(
            category,
            symbol.clone(),
            interval.clone(),
        );
    
        let result = api.get_kline(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get kline: {}", result.ret_msg());
    
            },
            Err(err) => {
                assert!(false, "Failed to get kline: {:?}", err);
            }
        }
    }


}

#[tokio::test]
async fn test_get_kline_fail() {
    let api = setup_api();
    let params = GetKlineParameters::new(
        GetKlineCategory::Linear,
        "XXXXXXX".to_string(),
        "1".to_string(),
    );

    let result = api.get_kline(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}