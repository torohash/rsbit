use rsbit::api::get::market::get_funding_rate_history::{
    GetFundingRateHistoryParameters,
    GetFundingRateHistoryCategory,
};
use crate::common::setup_api;

#[tokio::test]
async fn test_get_funding_rate_history_success() {
    let api = setup_api();
    let categories = vec![
        (GetFundingRateHistoryCategory::Linear, "BTCUSDT".to_string()),
        (GetFundingRateHistoryCategory::Inverse, "BTCUSD".to_string()),
    ];

    for (category, symbol) in categories.into_iter() {
        let params = GetFundingRateHistoryParameters::new(
            category,
            symbol.clone(),
        );
    
        let result = api.get_funding_rate_history(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get funding rate history: {}", result.ret_msg());
    
            },
            Err(err) => {
                assert!(false, "Failed to get funding rate history: {:?}", err);
            }
        }
    }


}

#[tokio::test]
async fn test_get_funding_rate_history_fail() {
    let api = setup_api();
    let params = GetFundingRateHistoryParameters::new(
        GetFundingRateHistoryCategory::Linear,
        "XXXXXXX".to_string(),
    );

    let result = api.get_funding_rate_history(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}