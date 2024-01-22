use rsbit::v5::api::post::lending::deposit_funds::DepositFundsParameters;
use crate::common::setup_api_private;

#[tokio::test]
async fn test_deposit_funds_success() {
    let api = setup_api_private();
    let params = DepositFundsParameters::new(
        "USDT".to_string(),
        10.0,
    );
    let result = api.deposit_funds(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to deposit funds: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to deposit funds: {:?}", err);
        }
    }

}