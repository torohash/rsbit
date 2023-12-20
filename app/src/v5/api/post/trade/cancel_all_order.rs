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

const PATH: &'static str = "/v5/order/cancel-all";

impl BybitApi {
    /// Cancels all open orders.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for canceling all orders.
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
    ///         cancel_all_order::{
    ///             CancelAllOrderParameters,
    ///             CancelAllOrderCategory
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
    ///         let params = CancelAllOrderParameters::new(
    ///             CancelAllOrderCategory::Linear,
    ///         );
    ///         let response = api.cancel_all_order(params).await;
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
    /// ```
    pub async fn cancel_all_order(&self, params: CancelAllOrderParameters) -> Result<CancelAllOrderResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CancelAllOrderCategory {
    Spot,
    Linear,
    Inverse,
    Option,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrderParameters {
    category: CancelAllOrderCategory,
    symbol: Option<String>,
    base_coin: Option<String>,
    settle_coin: Option<String>,
    order_filter: Option<String>,
    stop_order_type: Option<String>,
}

impl CancelAllOrderParameters {
    /// Creates a new instance of `CancelAllOrderParameters` with the specified category.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of orders to cancel.
    ///
    /// # Returns
    ///
    /// A new instance of `CancelAllOrderParameters`.
    pub fn new(category: CancelAllOrderCategory) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            settle_coin: None,
            order_filter: None,
            stop_order_type: None,
        }
    }

    /// Sets the symbol for the cancel all order parameters.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to set.
    ///
    /// # Returns
    ///
    /// The updated `CancelAllOrderParameters` instance.
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Sets the base coin for the cancel all order parameters.
    ///
    /// # Arguments
    ///
    /// * `base_coin` - The base coin to set.
    ///
    /// # Returns
    ///
    /// The updated `CancelAllOrderParameters` instance.
    pub fn with_base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }

    /// Sets the settle coin for the cancel all order parameters.
    ///
    /// # Arguments
    ///
    /// * `settle_coin` - The settle coin to set.
    ///
    /// # Returns
    ///
    /// The updated `CancelAllOrderParameters` instance.
    pub fn with_settle_coin(mut self, settle_coin: String) -> Self {
        self.settle_coin = Some(settle_coin);
        self
    }

    /// Sets the order filter for the cancel all order parameters.
    ///
    /// # Arguments
    ///
    /// * `order_filter` - The order filter to set.
    ///
    /// # Returns
    ///
    /// The updated `CancelAllOrderParameters` instance.
    pub fn with_order_filter(mut self, order_filter: String) -> Self {
        self.order_filter = Some(order_filter);
        self
    }

    /// Sets the stop order type for the cancel all order parameters.
    ///
    /// # Arguments
    ///
    /// * `stop_order_type` - The stop order type to set.
    ///
    /// # Returns
    ///
    /// The updated `CancelAllOrderParameters` instance.
    pub fn with_stop_order_type(mut self, stop_order_type: String) -> Self {
        self.stop_order_type = Some(stop_order_type);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrderResponse {
    ret_code: i32,
    ret_msg: String,
    result: CancelAllOrderResult,
    ret_ext_info: Value,
    time: u64,
}
impl CancelAllOrderResponse {
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

    pub fn result(&self) -> &CancelAllOrderResult {
        &self.result
    }

    pub fn set_result(&mut self, result: CancelAllOrderResult) {
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
pub struct CancelAllOrderResult {
    list: Vec<CancelAllOrder>,
}
impl CancelAllOrderResult {
    pub fn list(&self) -> &Vec<CancelAllOrder> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<CancelAllOrder>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrder {
    order_id: String,
    order_link_id: String,
}

impl CancelAllOrder {
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