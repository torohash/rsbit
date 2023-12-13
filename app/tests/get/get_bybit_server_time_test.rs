use crate::common::setup_api_public;

#[tokio::test]
async fn test_get_bybit_server_time_success() {
    let api = setup_api_public();

    let result = api.get_bybit_server_time().await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to get bybit server time: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to get bybit server time: {:?}", err);
        }
    }
}