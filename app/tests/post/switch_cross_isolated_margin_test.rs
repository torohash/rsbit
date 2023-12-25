use rsbit::v5::api::post::position::switch_cross_isolated_margin::{
    SwitchCrossIsolatedMarginCategory,
    SwitchCrossIsolatedMarginParameters,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_switch_cross_isolated_margin_success() {
    let api = setup_api_private();
    let params = SwitchCrossIsolatedMarginParameters::new(
        SwitchCrossIsolatedMarginCategory::Inverse,
        "BTCUSD".to_string(),
        1,
        10.0,
        10.0,
    );
    let result = api.switch_cross_isolated_margin(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to switch cross isolated margin: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to switch cross isolated margin: {:?}", err);
        }
    }

    let params = SwitchCrossIsolatedMarginParameters::new(
        SwitchCrossIsolatedMarginCategory::Inverse,
        "BTCUSD".to_string(),
        0,
        10.0,
        10.0,
    );
    let result = api.switch_cross_isolated_margin(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to switch cross isolated margin: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to switch cross isolated margin: {:?}", err);
        }
    }
}

#[tokio::test]
async fn test_switch_cross_isolated_margin_fail() {
    let api = setup_api_private();
    let params = SwitchCrossIsolatedMarginParameters::new(
        SwitchCrossIsolatedMarginCategory::Inverse,
        "XXXXXXX".to_string(),
        0,
        10.0,
        10.0,
    );
    let result = api.switch_cross_isolated_margin(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}