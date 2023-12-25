use rsbit::v5::api::get::position::get_execution::{
    GetExecutionParameters,
    GetExecutionCategory,
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_get_execution_success() {
    let api = setup_api_private();
    let categories = vec![
        (GetExecutionCategory::Linear),
    ];

    for category in categories.into_iter() {
        let params = GetExecutionParameters::new(
            category,
        );
    
        let result = api.get_execution(params).await;
        match result {
            Ok(result) => {
                let ret_code = result.ret_code();
                assert_eq!(ret_code, 0, "Failed to get execution: {}", result.ret_msg());
            },
            Err(err) => {
                assert!(false, "Failed to get execution: {:?}", err);
            }
        }
    }
}

// #[tokio::test]
// async fn test_get_execution_fail() {

//     let api = setup_api_private();
//     let params = GetExecutionParameters::new(
//         GetExecutionCategory::Linear,
//     ).with_symbol("XXXXXXX".to_string()).with_limit(0);

//     let result = api.get_execution(params).await;
//     match result {
//         Ok(result) => {
//             assert!(false, "Request should not have succeeded: {:?}", result);
//         },
//         Err(_) => {
//             assert!(true);
//         }
//     }
// }