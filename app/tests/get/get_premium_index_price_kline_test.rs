use rsbit::api::get::market::get_premium_index_price_kline::{
    GetPremiumIndexPriceKlineParameters,
    GetPremiumIndexPriceKlineCategory,
};
use crate::common::setup_api;

#[tokio::test]
async fn test_get_index_price_kline_success() {
    let api = setup_api();
    let categories = vec![
        (GetPremiumIndexPriceKlineCategory::Linear, "BTCUSDT".to_string(), "1".to_string()),
    ];

    for (category, symbol, interval) in categories.into_iter() {
        let params = GetPremiumIndexPriceKlineParameters::new(
            category,
            symbol.clone(),
            interval.clone(),
        );
    
        let result = api.get_premium_index_price_kline(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get premium index price kline: {}", result.ret_msg());
    
            },
            Err(err) => {
                assert!(false, "Failed to get premium index price kline: {:?}", err);
            }
        }
    }


}

#[tokio::test]
async fn test_get_index_price_kline_fail() {
    let api = setup_api();
    let params = GetPremiumIndexPriceKlineParameters::new(
        GetPremiumIndexPriceKlineCategory::Linear,
        "XXXXXXX".to_string(),
        "1".to_string(),
    );

    let result = api.get_premium_index_price_kline(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}