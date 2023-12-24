use rsbit::v5::api::post::position::switch_position_mode::{
    SwitchPositionModeCategory,
    SwitchPositionModeParameters,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_switch_position_mode_success() {
    let api = setup_api_private();
    let params = SwitchPositionModeParameters::new(
        SwitchPositionModeCategory::Linear,
        3,
    ).with_symbol("ETHUSDT".to_string());
    let result = api.switch_position_mode(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to switch position mode: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to switch position mode: {:?}", err);
        }
    }

    let params = SwitchPositionModeParameters::new(
        SwitchPositionModeCategory::Linear,
        0,
    ).with_symbol("ETHUSDT".to_string());
    let result = api.switch_position_mode(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to switch position mode: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to switch position mode: {:?}", err);
        }
    }
}

#[tokio::test]
async fn test_switch_position_mode_fail() {
    let api = setup_api_private();
    let params = SwitchPositionModeParameters::new(
        SwitchPositionModeCategory::Linear,
        0,
    );
    let result = api.switch_position_mode(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}