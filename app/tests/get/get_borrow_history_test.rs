use rsbit::v5::api::get::account::get_borrow_history::GetBorrowHistoryParameters;
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_borrow_history_success() {
    let api = setup_api_private();
    let params = GetBorrowHistoryParameters::new();

    let result = api.get_borrow_history(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to get borrow history: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to get borrow history: {:?}", err);
        }
    }
}