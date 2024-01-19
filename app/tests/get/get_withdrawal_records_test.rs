use rsbit::v5::api::get::asset::get_withdrawal_records::GetWithdrawalRecordsParameters;
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_withdrawal_records_success() {
    let api = setup_api_private();
    let params = GetWithdrawalRecordsParameters::new();

    let result = api.get_withdrawal_records(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to get withdrawal records: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to get withdrawal records: {:?}", err);
        }
    }
}