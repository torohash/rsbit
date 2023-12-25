use rsbit::v5::api::post::position::set_tpsl_mode::{
    SetTpslModeCategory,
    SetTpslModeParameters,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_set_tpsl_mode_success() {
    let api = setup_api_private();
    let params = SetTpslModeParameters::new(
        SetTpslModeCategory::Linear,
        "BTCUSDT".to_string(),
        "Partial".to_string(),
    );
    let result = api.set_tpsl_mode(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to set tpsl mode: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to set tpsl mode: {:?}", err);
        }
    }

    let params = SetTpslModeParameters::new(
        SetTpslModeCategory::Linear,
        "BTCUSDT".to_string(),
        "Full".to_string(),
    );
    let result = api.set_tpsl_mode(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to set tpsl mode: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to set tpsl mode: {:?}", err);
        }
    }
}

#[tokio::test]
async fn test_set_tpsl_mode_fail() {
    let api = setup_api_private();
    let params = SetTpslModeParameters::new(
        SetTpslModeCategory::Inverse,
        "XXXXXXX".to_string(),
        "Full".to_string(),
    );
    let result = api.set_tpsl_mode(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}