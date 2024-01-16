use rsbit::v5::api::get::asset::get_asset_info::{
    GetAssetInfoParameters,
    GetAssetInfoAccountType,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_asset_info_success() {
    let api = setup_api_private();
    let categories = vec![
        (GetAssetInfoAccountType::SPOT),
    ];

    for category in categories.into_iter() {
        let params = GetAssetInfoParameters::new(
            category,
        );
    
        let result = api.get_asset_info(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get asset info: {}", result.ret_msg());
            },
            Err(err) => {
                assert!(false, "Failed to get asset info: {:?}", err);
            }
        }
    }
}