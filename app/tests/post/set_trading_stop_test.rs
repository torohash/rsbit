use rsbit::v5::api::{
    get::market::get_tickers::{
        GetTickersParameters,
        GetTickersCategory,
    },
    post::position::set_trading_stop::{
        SetTradingStopCategory,
        SetTradingStopParameters,
    }
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_set_trading_stop_success() {
    let api = setup_api_private();
    let target_symbol = "BTCUSDT".to_string();
    let params = GetTickersParameters::new(
        GetTickersCategory::Linear,
    ).with_symbol(target_symbol.clone());
    let result = api.get_tickers(params).await;
    let price = result.unwrap_or_else(|err| {
        assert!(false, "Failed to get ticker: {:?}", err);
        panic!();
    }).result().linear().unwrap_or_else(|| {
        assert!(false, "Failed to get ticker: {:?}", "No value");
        panic!();
    }).list()[0].last_price();
    
    let params = SetTradingStopParameters::new(
        SetTradingStopCategory::Linear,
        "BTCUSDT".to_string(),
        0
    ).with_take_profit(price + 10000.0);
    let result = api.set_trading_stop(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to set trading stop: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to set trading stop: {:?}", err);
        }
    }

    let params = SetTradingStopParameters::new(
        SetTradingStopCategory::Linear,
        "BTCUSDT".to_string(),
        0
    ).with_take_profit(price + 10001.0);
    let result = api.set_trading_stop(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to set trading stop: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to set trading stop: {:?}", err);
        }
    }

}

#[tokio::test]
async fn test_set_trading_stop_fail() {
    let api = setup_api_private();
    let params = SetTradingStopParameters::new(
        SetTradingStopCategory::Inverse,
        "XXXXXXX".to_string(),
        1
    );
    let result = api.set_trading_stop(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}