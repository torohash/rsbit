use rsbit::v5::api::get::market::get_kline::{
    GetKlineParameters,
    GetKlineCategory,
};
use crate::common::setup_api_public;

#[tokio::test]
async fn test_get_kline_success() {
    let api = setup_api_public();
    let categories = vec![
        (GetKlineCategory::Linear, "BTCUSDT".to_string(), "1".to_string(), 100),
        (GetKlineCategory::Inverse, "BTCUSD".to_string(), "360".to_string(), 100),
        (GetKlineCategory::Spot, "BTCUSDT".to_string(), "D".to_string(), 100),
    ];

    for (category, symbol, interval, limit) in categories.into_iter() {
        let params = GetKlineParameters::new(
            category,
            symbol.clone(),
            interval.clone(),
        ).with_limit(limit);
    
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
    let api = setup_api_public();
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