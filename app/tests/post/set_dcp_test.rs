// Pending as dcp needs to be subscribed to trigger successfully (if not subscribed, fixed with ret_code: 10006?)


// use rsbit::v5::api::post::trade::set_dcp::SetDCPParameters;
// use crate::common::setup_api_private;

// #[tokio::test]
// async fn test_set_dcp_success() {
//     let api = setup_api_private();
//     let params = SetDCPParameters::new(
//         100,
//     );
//     let result = api.set_dcp(params).await;
//     match result {
//         Ok(result) => {
//             let ret_code = result.ret_code();
//             assert_eq!(ret_code, 0, "Failed to set DCP: {}", result.ret_msg());
//         },
//         Err(err) => {
//             assert!(false, "Failed to set DCP: {:?}", err);
//         }
//     }
// }

// #[tokio::test]
// async fn test_set_dcp_fail() {
//     let api = setup_api_private();
//     let params = SetDCPParameters::new(
//         100,
//     );
//     let result = api.set_dcp(params).await;
//     match result {
//         Ok(result) => {
//             assert!(false, "Request should not have succeeded: {:?}", result);
//         },
//         Err(_) => {
//             assert!(true);
//         }
//     }
// }