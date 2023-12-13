use rsbit::api::v5::get::market::get_risk_limit::{
    GetRiskLimitParameters,
    GetRiskLimitCategory,
};
use crate::common::setup_api_public;

#[tokio::test]
async fn test_get_risk_limit_success() {
    let api = setup_api_public();
    let categories = vec![
        (GetRiskLimitCategory::Linear, "BTCUSDT".to_string()),
        (GetRiskLimitCategory::Inverse, "BTCUSD".to_string()),
    ];

    for (category, symbol) in categories.into_iter() {
        let params = GetRiskLimitParameters::new(
            category,
        ).with_symbol(symbol.clone());
    
        let result = api.get_risk_limit(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get risk limit: {}", result.ret_msg());
                assert_eq!(result.result().list()[0].symbol(), &symbol);
            },
            Err(err) => {
                assert!(false, "Failed to get risk limit: {:?}", err);
            }
        }
    }
}

#[tokio::test]
async fn test_get_risk_limit_fail() {
    let api = setup_api_public();
    let params = GetRiskLimitParameters::new(
        GetRiskLimitCategory::Linear,
    ).with_symbol("XXXXXXX".to_string());

    let result = api.get_risk_limit(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}