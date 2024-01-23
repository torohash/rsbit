use rsbit::v5::api::get::lending::get_order_records::GetOrderRecordsParameters;
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_order_records_success() {
    let api = setup_api_private();
    let params = GetOrderRecordsParameters::new();

    let result = api.get_order_records(params).await;
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to get order records: {}", result.ret_msg());

        },
        Err(err) => {
            assert!(false, "Failed to get order records: {:?}", err);
        }
    }

}