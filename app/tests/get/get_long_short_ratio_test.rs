use rsbit::v5::api::get::market::get_long_short_ratio::{
    GetLongShortRatioParameters,
    GetLongShortRatioCategory,
};
use crate::common::setup_api_public;

#[tokio::test]
async fn test_get_long_short_ratio_success() {
    let api = setup_api_public();
    let categories = vec![
        (GetLongShortRatioCategory::Linear, "BTCUSDT".to_string(), "5min".to_string()),
        (GetLongShortRatioCategory::Inverse, "BTCUSD".to_string(), "5min".to_string()),
    ];

    for (category, symbol, period) in categories.into_iter() {
        let params = GetLongShortRatioParameters::new(
            category,
            symbol.clone(),
            period.clone(),
        );
    
        let result = api.get_long_short_ratio(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get long short ratio: {}", result.ret_msg());
                assert_eq!(result.result().list()[0].symbol(), &symbol);
            },
            Err(err) => {
                assert!(false, "Failed to get long short ratio: {:?}", err);
            }
        }
    }
}

#[tokio::test]
async fn test_get_long_short_ratio_fail() {
    let api = setup_api_public();
    let params = GetLongShortRatioParameters::new(
        GetLongShortRatioCategory::Linear,
        "XXXXXXX".to_string(),
        "5min".to_string(),
    );

    let result = api.get_long_short_ratio(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}