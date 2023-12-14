use rsbit::v5::api::get::market::get_public_recent_trading_history::{
    GetPublicRecentTradingHistoryParameters,
    GetPublicRecentTradingHistoryCategory,
};
use crate::common::setup_api_public;

#[tokio::test]
async fn test_get_public_recent_trading_history_success() {
    let api = setup_api_public();
    let categories = vec![
        (GetPublicRecentTradingHistoryCategory::Linear, "BTCUSDT".to_string()),
        (GetPublicRecentTradingHistoryCategory::Inverse, "BTCUSD".to_string()),
        // (GetPublicRecentTradingHistoryCategory::Option, "".to_string()), // Options are difficult to test due to the relationship with symbol names.
        (GetPublicRecentTradingHistoryCategory::Spot, "BTCUSDT".to_string()),
    ];

    for (category, symbol) in categories.into_iter() {
        let params = GetPublicRecentTradingHistoryParameters::new(
            category,
        ).with_symbol(symbol.clone());
    
        let result = api.get_public_recent_trading_history(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get tickers info: {}", result.ret_msg());
                assert_eq!(result.result().list()[0].symbol(), &symbol);
            },
            Err(err) => {
                assert!(false, "Failed to get tickers info: {:?}", err);
            }
        }
    }


}

#[tokio::test]
async fn test_get_public_recent_trading_history_fail() {
    let api = setup_api_public();
    let params = GetPublicRecentTradingHistoryParameters::new(
        GetPublicRecentTradingHistoryCategory::Linear,
    ).with_symbol("XXXXXXX".to_string());

    let result = api.get_public_recent_trading_history(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}