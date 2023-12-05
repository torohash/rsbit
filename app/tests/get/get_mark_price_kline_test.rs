use rsbit::api::get::market::get_mark_price_kline::{
    GetMarkPriceKlineParameters,
    GetMarkPriceKlineCategory,
};
use crate::common::setup_api;

#[tokio::test]
async fn test_get_instruments_info_success() {
    let api = setup_api();
    let categories = vec![
        (GetMarkPriceKlineCategory::Linear, "BTCUSDT".to_string(), "1".to_string()),
        (GetMarkPriceKlineCategory::Inverse, "BTCUSD".to_string(), "360".to_string()),
    ];

    for (category, symbol, interval) in categories.into_iter() {
        let params = GetMarkPriceKlineParameters::new(
            category,
            symbol.clone(),
            interval.clone(),
        );
    
        let result = api.get_mark_price_kline(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get mark price kline: {}", result.ret_msg());
    
            },
            Err(err) => {
                assert!(false, "Failed to get mark price kline: {:?}", err);
            }
        }
    }


}

#[tokio::test]
async fn test_get_instruments_info_fail() {
    let api = setup_api();
    let params = GetMarkPriceKlineParameters::new(
        GetMarkPriceKlineCategory::Linear,
        "XXXXXXX".to_string(),
        "1".to_string(),
    );

    let result = api.get_mark_price_kline(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}