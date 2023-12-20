use rsbit::v5::api::get::trade::get_order_history::{
    GetOrderHistoryParameters,
    GetOrderHistoryCategory,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_order_history_success() {
    let api = setup_api_private();
    let categories = vec![
        (GetOrderHistoryCategory::Linear, 10),
    ];

    for (category, limit) in categories.into_iter() {
        let params = GetOrderHistoryParameters::new(
            category,
        ).with_limit(limit);
    
        let result = api.get_order_history(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get order history: {}", result.ret_msg());
    
            },
            Err(err) => {
                assert!(false, "Failed to get order history: {:?}", err);
            }
        }
    }


}