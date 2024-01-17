use rsbit::v5::api::post::account::batch_set_collateral_coin::{
    BatchSetCollateralCoinParameters,
    BatchSetCollateralCoinRequestParameters,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_batch_set_collateral_coin_success() {
    let api = setup_api_private();
    let params = BatchSetCollateralCoinParameters::new(
        vec!(
            BatchSetCollateralCoinRequestParameters::new(
                "BTC".to_string(),
                "ON".to_string(),
            ),
            BatchSetCollateralCoinRequestParameters::new(
                "ETH".to_string(),
                "ON".to_string(),
            ),
        )
    );
    let result = api.batch_set_collateral_coin(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to batch set collateral coin: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to batch set collateral coin: {:?}", err);
        }
    }

    let params = BatchSetCollateralCoinParameters::new(
        vec!(
            BatchSetCollateralCoinRequestParameters::new(
                "BTC".to_string(),
                "OFF".to_string(),
            ),
            BatchSetCollateralCoinRequestParameters::new(
                "ETH".to_string(),
                "OFF".to_string(),
            ),
        )
    );
    let result = api.batch_set_collateral_coin(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to batch set collateral coin: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to batch set collateral coin: {:?}", err);
        }
    }

}

#[tokio::test]
async fn test_batch_set_collateral_coin_fail() {
    let api = setup_api_private();
    let params = BatchSetCollateralCoinParameters::new(
        vec!(
            BatchSetCollateralCoinRequestParameters::new(
                "XXXXXXX".to_string(),
                "XXXXXXX".to_string(),
            ),
            BatchSetCollateralCoinRequestParameters::new(
                "XXXXXXX".to_string(),
                "XXXXXXX".to_string(),
            ),
        )
    );
    let result = api.batch_set_collateral_coin(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}