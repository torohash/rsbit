use rsbit::v5::api::get::asset::get_deposit_records::GetDepositRecordsParameters;
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_deposit_records_success() {
    let api = setup_api_private();
    let params = GetDepositRecordsParameters::new(
        "BTC".to_string(),
    );

    let result = api.get_deposit_records(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to get deposit records: {}", result.ret_msg());
        },
        Err(err) => {
            assert!(false, "Failed to get deposit records: {:?}", err);
        }
    }
}