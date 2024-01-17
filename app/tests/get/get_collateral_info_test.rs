use rsbit::v5::api::get::account::get_collateral_info::GetCollateralInfoParameters;
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_collateral_info_success() {
    let api = setup_api_private();
    let params = GetCollateralInfoParameters::new();

    let result = api.get_collateral_info(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to get collateral info: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to get collateral info: {:?}", err);
        }
    }
}