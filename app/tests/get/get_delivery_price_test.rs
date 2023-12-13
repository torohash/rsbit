use rsbit::api::v5::get::market::get_delivery_price::{
    GetDeliveryPriceParameters,
    GetDeliveryPriceCategory,
};
use crate::common::setup_api_public;

#[tokio::test]
async fn test_get_delivery_price_success() {
    let api = setup_api_public();
    let categories = vec![
        (GetDeliveryPriceCategory::Option, "ETH-26DEC22-1400-C".to_string()),
    ];

    for (category, symbol) in categories.into_iter() {
        let params = GetDeliveryPriceParameters::new(
            category,
        ).with_symbol(symbol.clone());
    
        let result = api.get_delivery_price(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get delivery price: {}", result.ret_msg());
                assert_eq!(result.result().list()[0].symbol(), &symbol);
            },
            Err(err) => {
                assert!(false, "Failed to get delivery price: {:?}", err);
            }
        }
    }
}

#[tokio::test]
async fn test_get_delivery_price_fail() {
    let api = setup_api_public();
    let params = GetDeliveryPriceParameters::new(
        GetDeliveryPriceCategory::Option,
    ).with_symbol("XXXXXXX".to_string());

    let result = api.get_delivery_price(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}