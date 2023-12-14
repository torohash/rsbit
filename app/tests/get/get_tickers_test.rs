use rsbit::v5::api::get::market::get_tickers::{
    GetTickersParameters,
    TickersResult,
    GetTickersCategory,
};
use crate::common::setup_api_public;

#[tokio::test]
async fn test_get_tickers_success() {
    let api = setup_api_public();
    let categories = vec![
        (GetTickersCategory::Linear, "BTCUSDT".to_string()),
        (GetTickersCategory::Inverse, "BTCUSD".to_string()),
        (GetTickersCategory::Option, "BTC-30DEC22-18000-C".to_string()),
        (GetTickersCategory::Spot, "BTCUSDT".to_string()),
    ];

    for (category, symbol) in categories.into_iter() {
        let params = GetTickersParameters::new(
            category,
        ).with_symbol(symbol.clone());
    
        let result = api.get_tickers(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get tickers info: {}", result.ret_msg());
    
                match result.result() {
                    TickersResult::Linear(tickers) => assert_eq!(tickers.list()[0].symbol(), &symbol, "Failed to get tickers info: {}", result.ret_msg()),
                    TickersResult::Inverse(tickers) => assert_eq!(tickers.list()[0].symbol(), &symbol, "Failed to get tickers info: {}", result.ret_msg()),
                    // Option cannot narrow down the symbol name, so it will not be tested.
                    TickersResult::Option(_) => {},
                    TickersResult::Spot(tickers) => assert_eq!(tickers.list()[0].symbol(), &symbol, "Failed to get tickers info: {}", result.ret_msg()),
                }
            },
            Err(err) => {
                assert!(false, "Failed to get tickers info: {:?}", err);
            }
        }
    }


}

#[tokio::test]
async fn test_get_tickers_fail() {
    let api = setup_api_public();
    let params = GetTickersParameters::new(
        GetTickersCategory::Linear,
    ).with_symbol("XXXXXXX".to_string());

    let result = api.get_tickers(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}