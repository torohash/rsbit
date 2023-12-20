use rsbit::v5::api::get::trade::get_borrow_quota::{
    GetBorrowQuotaParameters,
    GetBorrowQuotaCategory,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_borrow_quota_success() {
    let api = setup_api_private();
    let categories = vec![
        (GetBorrowQuotaCategory::Spot, "BTCUSDT".to_string(), "Buy".to_string()),
    ];

    for (category, symbol, side) in categories.into_iter() {
        let params = GetBorrowQuotaParameters::new(
            category,
            symbol.clone(),
            side.clone(),
        );
    
        let result = api.get_borrow_quota(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get borrow quota: {}", result.ret_msg());
                assert_eq!(result.result().symbol(), &symbol);
                println!("result: {:?}", result.result());
            },
            Err(err) => {
                assert!(false, "Failed to get borrow quota: {:?}", err);
            }
        }
    }
}

#[tokio::test]
async fn test_get_borrow_quota_fail() {
    let api = setup_api_private();
    let params = GetBorrowQuotaParameters::new(
        GetBorrowQuotaCategory::Spot,
        "XXXXXXX".to_string(),
        "Buy".to_string(),
    );

    let result = api.get_borrow_quota(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}