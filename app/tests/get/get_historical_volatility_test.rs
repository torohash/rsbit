use rsbit::api::v5::get::market::get_historical_volatility::{
    GetHistoricalVolatilityParameters,
    GetHistoricalVolatilityCategory,
};
use crate::common::setup_api_public;

#[tokio::test]
async fn test_get_historical_volatility_success() {
    let api = setup_api_public();
    let categories = vec![
        (GetHistoricalVolatilityCategory::Option, "BTC".to_string()),
    ];

    for (category, base_coin) in categories.into_iter() {
        let params = GetHistoricalVolatilityParameters::new(
            category,
        ).with_base_coin(base_coin);
    
        let result = api.get_historical_volatility(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get historical volatility: {}", result.ret_msg());
            },
            Err(err) => {
                assert!(false, "Failed to get historical volatility: {:?}", err);
            }
        }
    }


}