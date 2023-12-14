use crate::{
    v5::api::{
        BybitApi,
        post::Post,
    },
    utils::serialize_option_as_string,
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/order/amend";

impl BybitApi {
    /// Amend an order.
    ///
    /// This method allows you to amend an existing order on the Bybit exchange.
    /// It takes in the `AmendOrderParameters` struct as a parameter and returns
    /// a `Result` containing the `AmendOrderResponse` if successful.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for amending the order.
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
    ///         amend_order::{
    ///             AmendOrderParameters,
    ///             AmendOrderCategory
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
    ///         let params = AmendOrderParameters::new(
    ///             AmendOrderCategory::Linear,
    ///             "BTCUSDT".to_string(),
    ///         ).with_order_id(info.result().order_id().to_string());
    ///         let response = api.amend_order(params).await;
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
    pub async fn amend_order(&self, params: AmendOrderParameters) -> Result<AmendOrderResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AmendOrderCategory {
    Spot,
    Linear,
    Inverse,
    Option,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderParameters {
    category: AmendOrderCategory,
    symbol: String,
    order_id: Option<String>,
    order_link_id: Option<String>,
    order_iv: Option<String>,
    #[serde(serialize_with = "serialize_option_as_string")]
    qty: Option<f64>,
    #[serde(serialize_with = "serialize_option_as_string")]
    price: Option<f64>,
    tpsl_mode: Option<String>,
    take_profit: Option<String>,
    stop_loss: Option<String>,
    tp_trigger_by: Option<String>,
    sl_trigger_by: Option<String>,
    tp_limit_price: Option<String>,
    sl_limit_price: Option<String>,
}

impl AmendOrderParameters {
    /// Creates a new instance of `AmendOrderParameters`.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the order.
    /// * `symbol` - The symbol of the order.
    ///
    /// # Returns
    ///
    /// A new instance of `AmendOrderParameters`.
    pub fn new(category: AmendOrderCategory, symbol: String) -> Self {
        Self {
            category,
            symbol,
            order_id: None,
            order_link_id: None,
            order_iv: None,
            qty: None,
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        }
    }

    /// Sets the order id for the amend order.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The order id to set.
    ///
    /// # Returns
    ///
    /// The modified `AmendOrderParameters` instance.
    pub fn with_order_id(mut self, order_id: String) -> Self {
        self.order_id = Some(order_id);
        self
    }
    
    /// Sets the order link id for the amend order.
    ///
    /// # Arguments
    ///
    /// * `order_link_id` - The order link id to set.
    ///
    /// # Returns
    ///
    /// The modified `AmendOrderParameters` instance.
    pub fn with_order_link_id(mut self, order_link_id: String) -> Self {
        self.order_link_id = Some(order_link_id);
        self
    }

    /// Sets the order iv for the amend order.
    ///
    /// # Arguments
    ///
    /// * `order_iv` - The order iv to set.
    ///
    /// # Returns
    ///
    /// The modified `AmendOrderParameters` instance.
    pub fn with_order_iv(mut self, order_iv: String) -> Self {
        self.order_iv = Some(order_iv);
        self
    }

    /// Sets the qty for the amend order.
    ///
    /// # Arguments
    ///
    /// * `qty` - The qty to set.
    ///
    /// # Returns
    ///
    /// The modified `AmendOrderParameters` instance.
    pub fn with_qty(mut self, qty: f64) -> Self {
        self.qty = Some(qty);
        self
    }

    /// Sets the price for the amend order.
    ///
    /// # Arguments
    ///
    /// * `price` - The price to set.
    ///
    /// # Returns
    ///
    /// The modified `AmendOrderParameters` instance.
    pub fn with_price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    /// Sets the tpsl mode for the amend order.
    ///
    /// # Arguments
    ///
    /// * `tpsl mode` - The tpsl mode to set.
    ///
    /// # Returns
    ///
    /// The modified `AmendOrderParameters` instance.
    pub fn with_tpsl_mode(mut self, tpsl_mode: String) -> Self {
        self.tpsl_mode = Some(tpsl_mode);
        self
    }

    /// Sets the take profit for the amend order.
    ///
    /// # Arguments
    ///
    /// * `take_profit` - The take profit to set.
    ///
    /// # Returns
    ///
    /// The modified `AmendOrderParameters` instance.
    pub fn with_take_profit(mut self, take_profit: String) -> Self {
        self.take_profit = Some(take_profit);
        self
    }

    /// Sets the stop loss for the amend order.
    ///
    /// # Arguments
    ///
    /// * `stop_loss` - The stop loss to set.
    ///
    /// # Returns
    ///
    /// The modified `AmendOrderParameters` instance.
    pub fn with_stop_loss(mut self, stop_loss: String) -> Self {
        self.stop_loss = Some(stop_loss);
        self
    }

    /// Sets the tp trigger by for the amend order.
    ///
    /// # Arguments
    ///
    /// * `tp_trigger_by` - The tp trigger by to set.
    ///
    /// # Returns
    ///
    /// The modified `AmendOrderParameters` instance.
    pub fn with_tp_trigger_by(mut self, tp_trigger_by: String) -> Self {
        self.tp_trigger_by = Some(tp_trigger_by);
        self
    }

    /// Sets the sl trigger by for the amend order.
    ///
    /// # Arguments
    ///
    /// * `sl trigger by` - The sl trigger by to set.
    ///
    /// # Returns
    ///
    /// The modified `AmendOrderParameters` instance.
    pub fn with_sl_trigger_by(mut self, sl_trigger_by: String) -> Self {
        self.sl_trigger_by = Some(sl_trigger_by);
        self
    }

    /// Sets the tp limit price for the amend order.
    ///
    /// # Arguments
    ///
    /// * `tp limit price` - The tp limit price to set.
    ///
    /// # Returns
    ///
    /// The modified `AmendOrderParameters` instance.
    pub fn with_tp_limit_price(mut self, tp_limit_price: String) -> Self {
        self.tp_limit_price = Some(tp_limit_price);
        self
    }

    /// Sets the sl limit price for the amend order.
    ///
    /// # Arguments
    ///
    /// * `sl_limit_price` - The sl limit price to set.
    ///
    /// # Returns
    ///
    /// The modified `AmendOrderParameters` instance.
    pub fn with_sl_limit_price(mut self, sl_limit_price: String) -> Self {
        self.sl_limit_price = Some(sl_limit_price);
        self
    }

}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderResponse {
    ret_code: i32,
    ret_msg: String,
    result: AmendOrderResult,
    ret_ext_info: Value,
    time: u64,
}
impl AmendOrderResponse {
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

    pub fn result(&self) -> &AmendOrderResult {
        &self.result
    }

    pub fn set_result(&mut self, result: AmendOrderResult) {
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
pub struct AmendOrderResult {
    order_id: String,
    order_link_id: String,
}
impl AmendOrderResult {
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