use rsbit::v5::api::{
    get::{
        market::get_tickers::{
            GetTickersParameters,
            GetTickersCategory,
        },
        trade::get_open_orders::{
            GetOpenOrdersParameters,
            GetOpenOrdersCategory,
        }
    },
    post::trade::{
        place_order::{
            PlaceOrderParameters,
            PlaceOrderCategory,
        },
        amend_order::{
            AmendOrderParameters,
            AmendOrderCategory,
        },
        cancel_order::{
            CancelOrderParameters,
            CancelOrderCategory,
        }
    }
};
use crate::common::setup_api_private;

#[tokio::test]
async fn test_order_success() {
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
    let params = PlaceOrderParameters::new(
        PlaceOrderCategory::Linear,
        target_symbol.clone(),
        "Buy".to_string(),
        "Limit".to_string(),
        0.01,
    ).with_price(target_price);
    
    let result = api.place_order(params).await;
    
    match result {
        Ok(result) => {
            let ret_code = result.ret_code();
            assert_eq!(ret_code, 0, "Failed to place order: {}", result.ret_msg());

            let order_id = result.result().order_id();
            let params = AmendOrderParameters::new(
                AmendOrderCategory::Linear,
                target_symbol.clone(),
            ).with_order_id(
                order_id.to_string()
            ).with_qty(0.02);

            let result = api.amend_order(params).await;
            match result {
                Ok(result) => {
                    let ret_code = result.ret_code();
                    assert_eq!(ret_code, 0, "Failed to amend order: {}", result.ret_msg());
                },
                Err(err) => {
                    assert!(false, "Failed to amend order: {:?}", err);
                }
            }

            let params = GetOpenOrdersParameters::new(
                GetOpenOrdersCategory::Linear,
            ).with_symbol(target_symbol.clone());
            let result = api.get_open_orders(params).await;
            match result {
                Ok(result) => {
                    let ret_code = result.ret_code();
                    assert_eq!(ret_code, 0, "Failed to get open orders: {}", result.ret_msg());
                },
                Err(err) => {
                    assert!(false, "Failed to get open orders: {:?}", err);
                }
            }


            let params = CancelOrderParameters::new(
                CancelOrderCategory::Linear,
                target_symbol.clone(),
            ).with_order_id(
                order_id.to_string()
            );
            let result = api.cancel_order(params).await;
            match result {
                Ok(result) => {
                    let ret_code = result.ret_code();
                    assert_eq!(ret_code, 0, "Failed to cancel order: {}", result.ret_msg());
                },
                Err(err) => {
                    assert!(false, "Failed to cancel order: {:?}", err);
                }
            }
        },
        Err(err) => {
            assert!(false, "Failed to place order: {:?}", err);
        }
    }

}

#[tokio::test]
async fn test_order_fail() {
    let api = setup_api_private();
    let params = PlaceOrderParameters::new(
        PlaceOrderCategory::Linear,
        "XXXXXXX".to_string(),
        "Buy".to_string(),
        "Market".to_string(),
        0.01,
    );

    let result = api.place_order(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }

    let params = AmendOrderParameters::new(
        AmendOrderCategory::Linear,
        "XXXXXXX".to_string(),
    ).with_order_id(
        "1234567890".to_string()
    ).with_qty(0.02);

    let result = api.amend_order(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }

    let params = CancelOrderParameters::new(
        CancelOrderCategory::Linear,
        "XXXXXXX".to_string(),
    ).with_order_id(
        "1234567890".to_string()
    );

    let result = api.cancel_order(params).await;
    match result {
        Ok(result) => {
            assert!(false, "Request should not have succeeded: {:?}", result);
        },
        Err(_) => {
            assert!(true);
        }
    }
}