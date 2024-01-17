use rsbit::v5::api::get::account::get_fee_rate::{
    GetFeeRateParameters,
    GetFeeRateCategory,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_fee_rate_success() {
    let api = setup_api_private();
    let categories = vec![
        (GetFeeRateCategory::Linear),
    ];

    for category in categories.into_iter() {
        let params = GetFeeRateParameters::new(
            category,
        );
    
        let result = api.get_fee_rate(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get fee rate: {}", result.ret_msg());
            },
            Err(err) => {
                assert!(false, "Failed to get fee rate: {:?}", err);
            }
        }
    }
}