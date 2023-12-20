use crate::v5::api::{
    BybitApi,
    post::Post,
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/order/cancel-batch";

impl BybitApi {
    /// batch cancel orders.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for batch cancel orders.
    ///
    /// # Examples
    /// 
    /// ```rust
    /// use rsbit::v5::api::{
    ///     post::trade::{
    ///         batch_place_order::{
    ///             BatchPlaceOrderParameters,
    ///             BatchPlaceOrderRequestParameters,
    ///             BatchPlaceOrderCategory
    ///         },
    ///         batch_cancel_order::{
    ///             BatchCancelOrderParameters,
    ///             BatchCancelOrderRequestParameters,
    ///             BatchCancelOrderCategory
    ///         },
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = BatchPlaceOrderParameters::new(
    ///         BatchPlaceOrderCategory::Linear,
    ///         vec![
    ///             BatchPlaceOrderRequestParameters::new("BTCUSDT".to_string(), "Buy".to_string(), "Limit".to_string(), 0.01).with_price(30000.0),
    ///             BatchPlaceOrderRequestParameters::new("BTCUSDT".to_string(), "Buy".to_string(), "Limit".to_string(), 0.01).with_price(30000.0),
    ///         ],
    ///     );
    ///     let response = api.batch_place_order(params).await;
    ///     match response {
    ///         Ok(info) => {
    ///             let params = BatchCancelOrderParameters::new(
    ///                 BatchCancelOrderCategory::Linear,
    ///                 vec![
    ///                     BatchCancelOrderRequestParameters::new("BTCUSDT".to_string()).with_order_id(info.result().list()[0].order_id().to_string()),
    ///                     BatchCancelOrderRequestParameters::new("BTCUSDT".to_string()).with_order_id(info.result().list()[1].order_id().to_string()),
    ///                 ],
    ///             );
    ///             let response = api.batch_cancel_order(params).await;
    ///             match response {
    ///                 Ok(info) => {
    ///                     // Handle the data
    ///                 },
    ///                 Err(err) => {
    ///                     // Handle the error
    ///                 }
    ///             }
    ///         },
    ///         Err(err) => {
    ///             // Handle the error
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn batch_cancel_order(&self, params: BatchCancelOrderParameters) -> Result<BatchCancelOrderResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BatchCancelOrderCategory {
    Linear,
    Option,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrderParameters {
    category: BatchCancelOrderCategory,
    request: Vec<BatchCancelOrderRequestParameters>,
}

impl BatchCancelOrderParameters {
    pub fn new(category: BatchCancelOrderCategory, request: Vec<BatchCancelOrderRequestParameters>) -> Self {
        Self {
            category,
            request,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrderRequestParameters {
    symbol: String,
    order_id: Option<String>,
    order_link_id: Option<String>,
}

impl BatchCancelOrderRequestParameters {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            order_id: None,
            order_link_id: None,
        }
    }

    /// Sets the sl limit price for the batch amend order.
    ///
    /// # Arguments
    ///
    /// * `sl_limit_price` - The sl limit price to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchAmendOrderParameters` instance.
    pub fn with_order_id(mut self, order_id: String) -> Self {
        self.order_id = Some(order_id);
        self
    }

    /// Sets the sl limit price for the batch amend order.
    ///
    /// # Arguments
    ///
    /// * `sl_limit_price` - The sl limit price to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchAmendOrderParameters` instance.
    pub fn with_order_link_id(mut self, order_link_id: String) -> Self {
        self.order_link_id = Some(order_link_id);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrderResponse {
    ret_code: i32,
    ret_msg: String,
    result: BatchCancelOrderResult,
    ret_ext_info: Value,
    time: u64,
}
impl BatchCancelOrderResponse {
    pub fn ret_code(&self) -> i32 {
        self.ret_code
    }

    pub fn set_ret_code(&mut self, ret_code: i32) {
        self.ret_code = ret_code;
    }

    pub fn ret_msg(&self) -> &str {
        &self.ret_msg
    }

    pub fn set_ret_msg(&mut self, ret_msg: String) {
        self.ret_msg = ret_msg;
    }

    pub fn result(&self) -> &BatchCancelOrderResult {
        &self.result
    }

    pub fn set_result(&mut self, result: BatchCancelOrderResult) {
        self.result = result;
    }

    pub fn ret_ext_info(&self) -> &Value {
        &self.ret_ext_info
    }

    pub fn set_ret_ext_info(&mut self, ret_ext_info: Value) {
        self.ret_ext_info = ret_ext_info;
    }

    pub fn time(&self) -> u64 {
        self.time
    }

    pub fn set_time(&mut self, time: u64) {
        self.time = time;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrderResult {
    list: Vec<BatchCancelOrder>,
}
impl BatchCancelOrderResult {
    pub fn list(&self) -> &Vec<BatchCancelOrder> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<BatchCancelOrder>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrder {
    category: String,
    symbol: String,
    order_id: String,
    order_link_id: String,
}

impl BatchCancelOrder {
    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    pub fn set_order_id(&mut self, order_id: String) {
        self.order_id = order_id;
    }

    pub fn order_link_id(&self) -> &str {
        &self.order_link_id
    }

    pub fn set_order_link_id(&mut self, order_link_id: String) {
        self.order_link_id = order_link_id;
    }

}