use rsbit::v5::api::get::market::get_index_price_kline::{
    GetIndexPriceKlineParameters,
    GetIndexPriceKlineCategory,
};
use crate::common::setup_api_public;

#[tokio::test]
async fn test_get_index_price_kline_success() {
    let api = setup_api_public();
    let categories = vec![
        (GetIndexPriceKlineCategory::Linear, "BTCUSDT".to_string(), "1".to_string()),
        (GetIndexPriceKlineCategory::Inverse, "BTCUSD".to_string(), "360".to_string()),
    ];

    for (category, symbol, interval) in categories.into_iter() {
        let params = GetIndexPriceKlineParameters::new(
            category,
            symbol.clone(),
            interval.clone(),
        );
    
        let result = api.get_index_price_kline(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get index price kline: {}", result.ret_msg());
    
            },
            Err(err) => {
                assert!(false, "Failed to get index price kline: {:?}", err);
            }
        }
    }


}

#[tokio::test]
async fn test_get_index_price_kline_fail() {
    let api = setup_api_public();
    let params = GetIndexPriceKlineParameters::new(
        GetIndexPriceKlineCategory::Linear,
        "XXXXXXX".to_string(),
        "1".to_string(),
    );

    let result = api.get_index_price_kline(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}