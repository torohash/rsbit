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

const PATH: &'static str = "/v5/order/cancel";

impl BybitApi {
    /// Cancels an order on the Bybit exchange.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for canceling the order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     post::trade::{
    ///         place_order::{
    ///             PlaceOrderParameters,
    ///             PlaceOrderCategory
    ///         },
    ///         cancel_order::{
    ///             CancelOrderParameters,
    ///             CancelOrderCategory
    ///         },
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = PlaceOrderParameters::new(
    ///         PlaceOrderCategory::Linear,
    ///         "BTCUSDT".to_string(),
    ///         "Buy".to_string(),
    ///         "Limit".to_string(),
    ///         0.01,
    ///     ).with_price(30000.0);
    ///     let response = api.place_order(params).await;
    ///     match response {
    ///         Ok(info) => {
    ///         let params = CancelOrderParameters::new(
    ///             CancelOrderCategory::Linear,
    ///             "BTCUSDT".to_string(),
    ///         ).with_order_id(info.result().order_id().to_string());
    ///         let response = api.cancel_order(params).await;
    ///             match response {
    ///                 Ok(info) => {
    ///                     // Handle the data
    ///                 },
    ///                 Err(err) => {
    ///                     // Handle the error
    ///                 }
    ///             }
    ///         }  
    ///         Err(err) => {}
    ///     }
    /// }
    pub async fn cancel_order(&self, params: CancelOrderParameters) -> Result<CancelOrderResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CancelOrderCategory {
    Spot,
    Linear,
    Inverse,
    Option,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderParameters {
    category: CancelOrderCategory,
    symbol: String,
    order_id: Option<String>,
    order_link_id: Option<String>,
    order_filter: Option<String>,
}

impl CancelOrderParameters {
    /// Creates a new instance of `CancelOrderParameters` with the specified category and symbol.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the order to cancel.
    /// * `symbol` - The symbol of the order to cancel.
    ///
    /// # Returns
    ///
    /// A new instance of `CancelOrderParameters`.
    pub fn new(category: CancelOrderCategory, symbol: String) -> Self {
        Self {
            category,
            symbol,
            order_id: None,
            order_link_id: None,
            order_filter: None,
        }
    }

    /// Sets the order ID for the `CancelOrderParameters`.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The order ID to set.
    ///
    /// # Returns
    ///
    /// The modified `CancelOrderParameters` instance.
    pub fn with_order_id(mut self, order_id: String) -> Self {
        self.order_id = Some(order_id);
        self
    }

    /// Sets the order link ID for the `CancelOrderParameters`.
    ///
    /// # Arguments
    ///
    /// * `order_link_id` - The order link ID to set.
    ///
    /// # Returns
    ///
    /// The modified `CancelOrderParameters` instance.
    pub fn with_order_link_id(mut self, order_link_id: String) -> Self {
        self.order_link_id = Some(order_link_id);
        self
    }

    /// Sets the order filter for the `CancelOrderParameters`.
    ///
    /// # Arguments
    ///
    /// * `order_filter` - The order filter to set.
    ///
    /// # Returns
    ///
    /// The modified `CancelOrderParameters` instance.
    pub fn with_order_filter(mut self, order_filter: String) -> Self {
        self.order_filter = Some(order_filter);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    ret_code: i32,
    ret_msg: String,
    result: CancelOrderResult,
    ret_ext_info: Value,
    time: u64,
}
impl CancelOrderResponse {
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

    pub fn result(&self) -> &CancelOrderResult {
        &self.result
    }

    pub fn set_result(&mut self, result: CancelOrderResult) {
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
pub struct CancelOrderResult {
    order_id: String,
    order_link_id: String,
}
impl CancelOrderResult {
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