use rsbit::v5::api::get::lending::get_lending_coin_info::GetLendingCoinInfoParameters;
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_lending_coin_info_success() {
    let api = setup_api_private();
    let params = GetLendingCoinInfoParameters::new();

    let result = api.get_lending_coin_info(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to get lending coin info: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to get lending coin info: {:?}", err);
        }
    }
}