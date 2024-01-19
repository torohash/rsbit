use rsbit::v5::api::get::asset::get_withdrawable_amount::GetWithdrawableAmountParameters;
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_withdrawable_amount_success() {
    let api = setup_api_private();
    let params = GetWithdrawableAmountParameters::new(
        "USDT".to_string()
    );

    let result = api.get_withdrawable_amount(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to get withdrawable amount: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to get withdrawable amount: {:?}", err);
        }
    }
}