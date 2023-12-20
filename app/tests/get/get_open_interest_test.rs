use rsbit::v5::api::get::market::get_open_interest::{
    GetOpenInterestParameters,
    GetOpenInterestCategory,
};
use crate::common::setup_api_public;

#[tokio::test]
async fn test_get_open_interest_success() {
    let api = setup_api_public();
    let categories = vec![
        (GetOpenInterestCategory::Linear, "BTCUSDT".to_string(), "5min".to_string()),
        (GetOpenInterestCategory::Inverse, "BTCUSD".to_string(), "5min".to_string()),
    ];

    for (category, symbol, interval_time) in categories.into_iter() {
        let params = GetOpenInterestParameters::new(
            category,
            symbol.clone(),
            interval_time.clone(),
        );
    
        let result = api.get_open_interest(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get open interest: {}", result.ret_msg());
                assert_eq!(result.result().symbol(), &symbol);
            },
            Err(err) => {
                assert!(false, "Failed to get open interest: {:?}", err);
            }
        }
    }


}

#[tokio::test]
async fn test_get_open_interest_fail() {
    let api = setup_api_public();
    let params = GetOpenInterestParameters::new(
        GetOpenInterestCategory::Linear,
        "XXXXXXX".to_string(),
        "5min".to_string(),
    );

    let result = api.get_open_interest(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}