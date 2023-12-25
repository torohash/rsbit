use rsbit::v5::api::get::position::get_closed_pnl::{
    GetClosedPnlParameters,
    GetClosedPnlCategory,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_closed_pnl_success() {
    let api = setup_api_private();
    let categories = vec![
        (GetClosedPnlCategory::Linear),
    ];

    for category in categories.into_iter() {
        let params = GetClosedPnlParameters::new(
            category,
        );
    
        let result = api.get_closed_pnl(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get closed pnl: {}", result.ret_msg());
            },
            Err(err) => {
                assert!(false, "Failed to get closed pnl: {:?}", err);
            }
        }
    }
}