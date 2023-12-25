use rsbit::v5::api::post::position::set_leverage::{
    SetLeverageCategory,
    SetLeverageParameters,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_set_leverage_success() {
    let api = setup_api_private();
    let params = SetLeverageParameters::new(
        SetLeverageCategory::Linear,
        "BTCUSDT".to_string(),
        10.0,
        10.0,
    );
    let result = api.set_leverage(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to set Leverage: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to set Leverage: {:?}", err);
        }
    }

    let params = SetLeverageParameters::new(
        SetLeverageCategory::Linear,
        "BTCUSDT".to_string(),
        12.0,
        12.0,
    );
    let result = api.set_leverage(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to set Leverage: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to set Leverage: {:?}", err);
        }
    }
}

#[tokio::test]
async fn test_set_leverage_fail() {
    let api = setup_api_private();
    let params = SetLeverageParameters::new(
        SetLeverageCategory::Linear,
        "XXXXXXX".to_string(),
        10.0,
        10.0,
    );
    let result = api.set_leverage(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}