use rsbit::v5::api::get::account::get_wallet_balance::{
    GetWalletBalanceParameters,
    GetWalletBalanceAccountType,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_wallet_balance_success() {
    let api = setup_api_private();
    let categories = vec![
        (GetWalletBalanceAccountType::UNIFIED),
    ];

    for category in categories.into_iter() {
        let params = GetWalletBalanceParameters::new(
            category,
        );
    
        let result = api.get_wallet_balance(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get wallet balance: {}", result.ret_msg());
            },
            Err(err) => {
                assert!(false, "Failed to get wallet balance: {:?}", err);
            }
        }
    }
}