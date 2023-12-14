use rsbit::api::v5::post::trade::place_order::{
    PlaceOrderParameters,
    PlaceOrderCategory,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_order_success() {
    let api = setup_api_private();
    let params = PlaceOrderParameters::new(
        PlaceOrderCategory::Linear,
        "BTCUSDT".to_string(),
        "Buy".to_string(),
        "Market".to_string(),
        0.01,
    );
    
    let result = api.place_order(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to place order: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to place order: {:?}", err);
        }
    }
}

#[tokio::test]
async fn test_order_fail() {
    let api = setup_api_private();
    let params = PlaceOrderParameters::new(
        PlaceOrderCategory::Linear,
        "XXXXXXX".to_string(),
        "Buy".to_string(),
        "Market".to_string(),
        0.01,
    );

    let result = api.place_order(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}