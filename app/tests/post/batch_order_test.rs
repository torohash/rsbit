use rsbit::v5::api::{
    get::market::get_tickers::{
        GetTickersParameters,
        GetTickersCategory,
    },
    post::trade::{
        batch_place_order::{
            BatchPlaceOrderParameters,
            BatchPlaceOrderRequestParameters,
            BatchPlaceOrderCategory,
        },
        batch_amend_order::{
            BatchAmendOrderParameters,
            BatchAmendOrderRequestParameters,
            BatchAmendOrderCategory,
        },
        cancel_all_order::{
            CancelAllOrderParameters,
            CancelAllOrderCategory,
        }
    }
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_batch_order_success() {
    let api = setup_api_private();
    let target_symbol = "BTCUSDT".to_string();
    let params = GetTickersParameters::new(
        GetTickersCategory::Linear,
    ).with_symbol(target_symbol.clone());
    let result = api.get_tickers(params).await;
    let price = result.unwrap_or_else(|err| {
        assert!(false, "Failed to get ticker: {:?}", err);
        panic!();
    }).result().linear().unwrap_or_else(|| {
        assert!(false, "Failed to get ticker: {:?}", "No value");
        panic!();
    }).list()[0].last_price();
    
    let target_price = price - 1000.0;
    let request = BatchPlaceOrderRequestParameters::new(
        target_symbol.clone(),
        "Buy".to_string(),
        "Limit".to_string(),
        0.01,
    ).with_price(target_price);
    let params = BatchPlaceOrderParameters::new(
        BatchPlaceOrderCategory::Linear,
        vec![request.clone(), request.clone(), request.clone()]
    );
    
    let result = api.batch_place_order(params).await;
    
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to batch place order: {}", result.ret_msg());

            let params = BatchAmendOrderParameters::new(
                BatchAmendOrderCategory::Linear,
                vec![
                    BatchAmendOrderRequestParameters::new(target_symbol.clone()).with_order_id(result.result().list()[0].order_id().to_string()).with_qty(0.02),
                    BatchAmendOrderRequestParameters::new(target_symbol.clone()).with_order_id(result.result().list()[1].order_id().to_string()).with_qty(0.02),
                    BatchAmendOrderRequestParameters::new(target_symbol.clone()).with_order_id(result.result().list()[2].order_id().to_string()).with_qty(0.02),
                ],
            );
            
            // batch amend order
            let result = api.batch_amend_order(params).await;
            match result {
                Ok(result) => {
                    let ret_code = result.ret_code();
                    assert_eq!(ret_code, 0, "Failed to batch amend order: {}", result.ret_msg());
                    let result = result.result();
                    let list = result.list();
                    assert_eq!(list.len(), 3);
                },
                Err(err) => {
                    assert!(false, "Failed to batch amend order: {:?}", err);
                }
            }

            // cancel all order
            let params = CancelAllOrderParameters::new(
                CancelAllOrderCategory::Linear,
            ).with_symbol(target_symbol);      
            let result = api.cancel_all_order(params).await;
            match result {
                Ok(result) => {
                    let ret_code = result.ret_code();
                    assert_eq!(ret_code, 0, "Failed to cancel all order: {}", result.ret_msg());
                },
                Err(err) => {
                    assert!(false, "Failed to cancel all order: {:?}", err);
                }
            }
        },
        Err(err) => {
            assert!(false, "Failed to place order: {:?}", err);
        }
    }

}

#[tokio::test]
async fn test_batch_order_fail() {
    let api = setup_api_private();
    let request = BatchPlaceOrderRequestParameters::new(
        "XXXXXXX".to_string(),
        "Buy".to_string(),
        "Market".to_string(),
        0.01,
    );
    let params = BatchPlaceOrderParameters::new(
        BatchPlaceOrderCategory::Linear,
        vec![request.clone(), request.clone(), request.clone()]
    );
    
    let result = api.batch_place_order(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }

    let params = BatchAmendOrderParameters::new(
        BatchAmendOrderCategory::Linear,
        vec![
            BatchAmendOrderRequestParameters::new("XXXXXXX".to_string()).with_order_id("1234567890".to_string()),
            BatchAmendOrderRequestParameters::new("XXXXXXX".to_string()).with_order_id("1234567890".to_string()),
            BatchAmendOrderRequestParameters::new("XXXXXXX".to_string()).with_order_id("1234567890".to_string()),
        ],
    );
    let result = api.batch_amend_order(params).await;
    match result {
        Ok(result) => {
            // Error of batch amend order is output to ret_ext_info
            if let Some(list) = result.ret_ext_info().get("list").and_then(|list| list.as_array()) {
                if let Some(item) = list.get(0) {
                    let code = item.get("code").and_then(|code| code.as_i64());
                    let msg = item.get("msg").and_then(|msg| msg.as_str());
                    if code.is_some() && msg.is_some() {
                        match code {
                            Some(code) => {
                                if code == 0 {
                                    assert!(false, "Request should not have succeeded: {:?}", msg.unwrap());
                                }
                            },
                            None => {
                                assert!(true);
                            }
                        }
                    }
                }
            }
        },
        Err(_) => {
            assert!(true);
        }
    }

    let params = CancelAllOrderParameters::new(
        CancelAllOrderCategory::Linear,
    );

    let result = api.cancel_all_order(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
    
}