use rsbit::api::get::market::get_instruments_info::{
    GetInstrumentsInfoParameters,
    InstrumentsInfoResult,
    GetInstrumentsInfoCategory,
};
use crate::common::setup_api;

#[tokio::test]
async fn test_get_instruments_info_success() {
    let api = setup_api();
    let categories = vec![
        (GetInstrumentsInfoCategory::Linear, "BTCUSDT".to_string()),
        (GetInstrumentsInfoCategory::Inverse, "BTCUSD".to_string()),
        (GetInstrumentsInfoCategory::Option, "".to_string()),
        (GetInstrumentsInfoCategory::Spot, "BTCUSDT".to_string()),
    ];

    for (category, symbol) in categories.into_iter() {
        let params = GetInstrumentsInfoParameters::new(
            category,
        ).with_symbol(symbol.clone());
    
        let result = api.get_instruments_info(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get instruments info: {}", result.ret_msg());
    
                match result.result() {
                    InstrumentsInfoResult::Linear(instruments_info) => assert_eq!(instruments_info.list()[0].symbol(), &symbol, "Failed to get instruments info: {}", result.ret_msg()),
                    InstrumentsInfoResult::Inverse(instruments_info) => assert_eq!(instruments_info.list()[0].symbol(), &symbol, "Failed to get instruments info: {}", result.ret_msg()),
                    // Option cannot narrow down the symbol name, so it will not be tested.
                    InstrumentsInfoResult::Option(_) => {},
                    InstrumentsInfoResult::Spot(instruments_info) => assert_eq!(instruments_info.list()[0].symbol(), &symbol, "Failed to get instruments info: {}", result.ret_msg()),
                }
            },
            Err(err) => {
                assert!(false, "Failed to get instruments info: {:?}", err);
            }
        }
    }


}

#[tokio::test]
async fn test_get_instruments_info_fail() {
    let api = setup_api();
    let params = GetInstrumentsInfoParameters::new(
        GetInstrumentsInfoCategory::Linear,
    ).with_symbol("XXXXXXX".to_string());

    let result = api.get_instruments_info(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}