use rsbit::v5::api::post::position::set_risk_limit::{
    SetRiskLimitCategory,
    SetRiskLimitParameters,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_set_risk_limit_success() {
    let api = setup_api_private();
    let params = SetRiskLimitParameters::new(
        SetRiskLimitCategory::Linear,
        "BTCUSDT".to_string(),
        1,
    );
    let result = api.set_risk_limit(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to set risk limit: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to set risk limit: {:?}", err);
        }
    }

    let params = SetRiskLimitParameters::new(
        SetRiskLimitCategory::Linear,
        "BTCUSDT".to_string(),
        2,
    );
    let result = api.set_risk_limit(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to set risk limit: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to set risk limit: {:?}", err);
        }
    }
}

#[tokio::test]
async fn test_set_risk_limit_fail() {
    let api = setup_api_private();
    let params = SetRiskLimitParameters::new(
        SetRiskLimitCategory::Inverse,
        "XXXXXXX".to_string(),
        10000,
    );
    let result = api.set_risk_limit(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}