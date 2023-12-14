use rsbit::v5::api::get::market::get_insurance::GetInsuranceParameters;
use crate::common::setup_api_public;

#[tokio::test]
async fn test_get_insurance_success() {
    let api = setup_api_public();
    let coins = vec![
         "BTC".to_string(),
    ];

    for coin in coins.into_iter() {
        let params = GetInsuranceParameters::new().with_coin(coin);
    
        let result = api.get_insurance(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get insurance: {}", result.ret_msg());
            },
            Err(err) => {
                assert!(false, "Failed to get insurance: {:?}", err);
            }
        }
    }


}

#[tokio::test]
async fn test_get_insurance_fail() {
    let api = setup_api_public();
    let params = GetInsuranceParameters::new().with_coin("XXXXXXX".to_string());

    let result = api.get_insurance(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}