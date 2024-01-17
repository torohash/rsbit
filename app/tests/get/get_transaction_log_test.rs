use rsbit::v5::api::get::account::get_transaction_log::GetTransactionLogParameters;
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_transaction_log_success() {
    let api = setup_api_private();
    let params = GetTransactionLogParameters::new();

    let result = api.get_transaction_log(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to get transaction log: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to get transaction log: {:?}", err);
        }
    }
}